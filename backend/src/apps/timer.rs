use std::rc::Rc;

use tauri::{App, Manager};
// use tauri::{Manager, Window};

// --- Reducer ---
pub trait Reducible {
    type Action;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self>;
}

// --- Reducer -- end

#[derive(Default, Debug, PartialEq, Copy, Clone)]
enum TimerStatus {
    #[default]
    Idle,
    Running,
    Paused,
}

#[derive(Debug)]
pub enum TimerStateAction {
    // IncrementSeconds,
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
#[derive(Copy, Clone)]
pub struct TimerState {
    minutes: i32,
    seconds: i32,
    status: TimerStatus,
}

impl Default for TimerState {
    fn default() -> Self {
        TimerState {
            minutes: 17,
            seconds: 00,
            status: TimerStatus::Idle,
        }
    }
}

// impl TimerState {
//     /// Reducer Action Type
//     type Action = TimerStateAction;

//     /// Reducer Function
//     fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
//         match action {
//             TimerStateAction::DecrementSeconds => {
//                 let is_seconds_zero = self.seconds == 0;
//                 let is_minutes_zero = self.minutes == 0;

//                 if is_minutes_zero && is_seconds_zero {
//                     return Self {
//                         minutes: 0,
//                         seconds: 0,
//                         status: TimerStatus::Idle,
//                     }
//                     .into();
//                 }

//                 Self {
//                     status: self.status,
//                     seconds: if is_seconds_zero {
//                         59
//                     } else {
//                         self.seconds - 1
//                     },
//                     minutes: if is_seconds_zero {
//                         self.minutes - 1
//                     } else {
//                         self.minutes
//                     },
//                 }
//                 .into()
//             }
//             TimerStateAction::IncrementMinutes => Self {
//                 status: self.status,
//                 minutes: self.minutes + 1,
//                 seconds: self.seconds,
//             }
//             .into(),
//             TimerStateAction::DecrementMinutes => Self {
//                 status: self.status,
//                 seconds: self.seconds,
//                 minutes: if self.minutes != 0 {
//                     self.minutes - 1
//                 } else {
//                     0
//                 },
//             }
//             .into(),
//             TimerStateAction::Start => Self {
//                 status: TimerStatus::Running,
//                 seconds: self.seconds,
//                 minutes: self.minutes,
//             }
//             .into(),
//             TimerStateAction::Pause => Self {
//                 status: TimerStatus::Paused,
//                 seconds: self.seconds,
//                 minutes: self.minutes,
//             }
//             .into(),
//             TimerStateAction::Reset => Self {
//                 ..Default::default()
//             }
//             .into(),
//         }
//     }
// }

// --------------------------------
// --- Timer ----------------------
// --------------------------------
pub struct Timer {
    pub state: TimerState,
}

impl Default for Timer {
    fn default() -> Self {
        Timer {
            state: TimerState {
                ..Default::default()
            },
        }
    }
}

impl Timer {
    pub fn dispatch(&self, action: TimerStateAction) -> TimerState {
        println!("Dispatch called with action: {:?}", action);
        // TODO: add handlers for each action
        self.state
    }

    pub fn current_time_string(&self) -> String {
        format!("{}:{}", self.state.minutes, self.state.seconds)
    }
}

// ---------------------------
// --- Commands --------------
// ---------------------------
#[tauri::command]
pub fn timer_test(window: tauri::Window, timer: tauri::State<'_, Timer>) {
    let app_handle = window.app_handle();

    // Dispatch action
    timer.dispatch(TimerStateAction::Start);

    println!("{}", timer.current_time_string());

    // Emit event to notify all windows
    app_handle
        .emit_all(
            "timer_started",
            TimerEventPayload {
                time: timer.current_time_string(),
            },
        )
        .unwrap();
}

// ------------------------------
// --- Events -------------------
// -- Event Payload
#[derive(Clone, serde::Serialize)]
struct TimerEventPayload {
    time: String,
}
