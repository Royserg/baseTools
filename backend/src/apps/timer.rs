use std::sync::Mutex;
use tauri::State;

// === Store ===
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

#[derive(Default, Debug, PartialEq, Copy, Clone)]
enum TimerStatus {
    #[default]
    Idle,
    Running,
    Paused,
}

#[derive(Debug, Copy, Clone)]
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
#[derive(Copy, Clone, PartialEq)]
pub struct TimerState {
    minutes: i32,
    seconds: i32,
    status: TimerStatus,
}

impl Default for TimerState {
    fn default() -> Self {
        TimerState {
            minutes: 25,
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
                    // Timer finished -> switch to default status
                    status: TimerStatus::Idle,
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
    // pub state: Mutex<TimerState>,
    pub store: Mutex<Store<TimerState, TimerStateAction>>,
}

impl Default for Timer {
    fn default() -> Self {
        let store: Store<TimerState, TimerStateAction> = Store::new(Box::new(timer_state_reducer));
        Timer {
            store: Mutex::new(store),
        }
    }
}

// ---------------------------
// --- Commands --------------
// ---------------------------
#[tauri::command]
pub fn timer_start(window: tauri::Window, timer: State<Timer>) {
    // let app_handle = window.app_handle();

    // Dispatch action
    let mut store_handle = timer.store.lock().unwrap();
    println!("Store state: {}", store_handle.state.minutes);
    store_handle.dispatch(TimerStateAction::IncrementMinutes);
    println!("Store state: {}", store_handle.state.minutes);
    store_handle.dispatch(TimerStateAction::IncrementMinutes);
    println!("Store state: {}", store_handle.state.minutes);

    // timer.dispatch(TimerStateAction::DecrementSeconds);
    // println!("{}", timer.current_time_string());

    // timer.dispatch(TimerStateAction::Start);
    // println!("{}", timer.current_time_string());

    // let thread_data = timer.current_time_string().clone();
    // thread::spawn(move || loop {
    //     thread::sleep(Duration::from_secs(1));
    //     println!("{}", thread_data);
    // });

    // Emit event to notify all windows
    // TODO: emitting should be coming from Timer struct
    // app_handle
    //     .emit_all(
    //         "timer_state_changed",
    //         TimerEventPayload {
    //             status: "running".into(),
    //             time: timer.current_time_string(),
    //         },
    //     )
    //     .unwrap();
}

// ------------------------------
// --- Events -------------------
// -- Event Payload
#[derive(Clone, serde::Serialize)]
struct TimerEventPayload {
    status: String,
    time: String,
}
