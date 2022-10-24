use crate::TIMER_FINISHED_WINDOW_LABEL;
use std::{
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
    time::Duration,
};
use tauri::{AppHandle, Manager, State};
use tauri_plugin_positioner::{Position, WindowExt};

// === Store ===
// REFORMAT: move to another file
pub struct Store<T, A> {
    state: T,
    reducer: Box<dyn Fn(T, A) -> T + Send>,
}

impl<T, A> Store<T, A>
where
    T: Copy + Clone + Default + PartialEq,
{
    fn new(reducer: Box<dyn Fn(T, A) -> T + Send>) -> Self {
        Self {
            state: T::default(),
            reducer,
        }
    }

    fn dispatch(&mut self, action: A) {
        self.state = (self.reducer)(self.state, action);
    }
}

// =============

#[derive(Default, Debug, PartialEq, Copy, Clone, serde::Serialize)]
pub enum TimerStatus {
    #[default]
    Idle,
    Running,
    Paused,
    Finished,
}

#[derive(Copy, Clone)]
pub enum TimerStateAction {
    IncrementSeconds,
    DecrementSeconds,
    IncrementMinutes,
    DecrementMinutes,
    Start,
    Pause,
    Reset,
}

// --------------------------------
// --- Timer State ----------------
// --------------------------------
#[derive(Copy, Clone, PartialEq, serde::Serialize, Debug)]
pub struct TimerState {
    minutes: i32,
    seconds: i32,
    status: TimerStatus,
}

impl Default for TimerState {
    fn default() -> Self {
        TimerState {
            minutes: 20,
            seconds: 00,
            status: TimerStatus::Idle,
        }
    }
}

// --------------------------------
// --- Reducer function -----------
fn timer_state_reducer(state: TimerState, action: TimerStateAction) -> TimerState {
    match action {
        // -- Status
        TimerStateAction::Start => TimerState {
            status: TimerStatus::Running,
            seconds: state.seconds,
            minutes: state.minutes,
        },
        TimerStateAction::Pause => TimerState {
            status: TimerStatus::Paused,
            seconds: state.seconds,
            minutes: state.minutes,
        },
        TimerStateAction::Reset => TimerState {
            ..Default::default()
        },
        // -- Minutes
        TimerStateAction::IncrementMinutes => TimerState {
            status: state.status,
            minutes: state.minutes + 1,
            seconds: state.seconds,
        },
        TimerStateAction::DecrementMinutes => TimerState {
            status: state.status,
            minutes: if state.minutes != 0 {
                state.minutes - 1
            } else {
                0
            },
            seconds: state.seconds,
        },
        // -- Seconds
        TimerStateAction::DecrementSeconds => {
            let is_seconds_zero = state.seconds == 0;
            let is_minutes_zero = state.minutes == 0;

            if is_minutes_zero && is_seconds_zero {
                return TimerState {
                    status: TimerStatus::Finished,
                    minutes: 0,
                    seconds: 0,
                };
            }

            TimerState {
                status: state.status,
                seconds: if is_seconds_zero {
                    59
                } else {
                    state.seconds - 1
                },
                minutes: if is_seconds_zero {
                    state.minutes - 1
                } else {
                    state.minutes
                },
            }
        }
        _ => state,
    }
}

// --------------------------------
// --- Timer ----------------------
// --------------------------------
pub struct Timer {
    pub store: Arc<Mutex<Store<TimerState, TimerStateAction>>>,
}

impl Default for Timer {
    fn default() -> Self {
        let store: Store<TimerState, TimerStateAction> = Store::new(Box::new(timer_state_reducer));
        Timer {
            store: Arc::new(Mutex::new(store)),
        }
    }
}

// ---------------------------
// --- Commands --------------
// ---------------------------
#[tauri::command]
pub fn timer_start(timer: State<Timer>) -> TimerState {
    let state_handler = Arc::clone(&timer.store);

    let state_change = thread::spawn(move || {
        let mut timer_handle = state_handler.lock().unwrap();
        timer_handle.dispatch(TimerStateAction::Start);
        timer_handle.state
    });

    let result = state_change.join().unwrap();
    result
}

#[tauri::command]
pub fn timer_pause(timer: State<Timer>) -> TimerState {
    let state_handler = Arc::clone(&timer.store);

    let state_change = thread::spawn(move || {
        let mut timer_handle = state_handler.lock().unwrap();
        timer_handle.dispatch(TimerStateAction::Pause);
        timer_handle.state
    });

    let result = state_change.join().unwrap();
    result
}

#[tauri::command]
pub fn timer_get_state(timer: State<Timer>) -> TimerState {
    let state_handler = Arc::clone(&timer.store);

    let state = thread::spawn(move || {
        let timer_handle = state_handler.lock().unwrap();
        timer_handle.state
    });

    let result = state.join().unwrap();
    result
}

#[tauri::command]
pub fn timer_reset(timer: State<Timer>) -> TimerState {
    let state_handler = Arc::clone(&timer.store);

    let state = thread::spawn(move || {
        let mut timer_handle = state_handler.lock().unwrap();
        timer_handle.dispatch(TimerStateAction::Reset);
        timer_handle.state
    });

    let result = state.join().unwrap();
    result
}

// ------------------------------
// --- Events -------------------
// -- Event Payload
#[derive(Clone, serde::Serialize)]
pub struct TimerEventPayload {
    status: TimerStatus,
    minutes: i32,
    seconds: i32,
}

// -------------------------------
// --- Thread --------------------
pub fn spawn_timer_thread(
    app_handle: &AppHandle,
    state_handler: Arc<Mutex<Store<TimerState, TimerStateAction>>>,
) -> JoinHandle<()> {
    let app_handle = app_handle.clone();

    thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(1));

        let mut state_handler = state_handler.lock().unwrap();

        let timer_status = state_handler.state.status;

        if timer_status == TimerStatus::Finished {
            let window_width = 400.00;
            let window_height = 400.00;

            let timer_finished_win = tauri::WindowBuilder::new(
                &app_handle,
                TIMER_FINISHED_WINDOW_LABEL,
                tauri::WindowUrl::App("timer/timer-finished".into()),
            )
            .inner_size(window_width, window_height)
            .resizable(false)
            .decorations(false)
            .always_on_top(true)
            .skip_taskbar(true)
            .transparent(true)
            .build()
            .unwrap();

            timer_finished_win.move_window(Position::TopRight).unwrap();
        }

        if timer_status == TimerStatus::Running {
            state_handler.dispatch(TimerStateAction::DecrementSeconds);

            // Emit state change event
            let minutes = state_handler.state.minutes;
            let seconds = state_handler.state.seconds;
            app_handle
                .emit_all(
                    "timer_state_changed",
                    TimerEventPayload {
                        status: timer_status,
                        minutes,
                        seconds,
                    },
                )
                .unwrap();
        }
    })
}
