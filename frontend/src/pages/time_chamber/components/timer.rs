use std::rc::Rc;

use js_sys::Function;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use wasm_bindgen_futures::spawn_local;
use weblog::console_log;
use yew::prelude::*;

use gloo_timers::callback::Interval;

// --- Triggers for js Tauri API commands ---
type Callback = fn();

#[wasm_bindgen(module = "/public/glue.js")]
extern "C" {
    #[wasm_bindgen(js_name = listenToEvent)]
    pub async fn listenToEvent(evtName: String) -> JsValue;
    #[wasm_bindgen(js_name = timerStart)]
    pub async fn timerStart();
}

#[derive(Default, Debug, PartialEq, Copy, Clone)]
enum TimerStatus {
    #[default]
    Idle,
    Running,
    Paused,
}

enum TimerStateAction {
    // IncrementSeconds,
    DecrementSeconds,
    IncrementMinutes,
    DecrementMinutes,
    Start,
    Pause,
    Reset,
}

struct TimerState {
    minutes: i32,
    seconds: i32,
    status: TimerStatus,
}

impl Default for TimerState {
    fn default() -> Self {
        TimerState {
            minutes: 25,
            seconds: 0,
            status: TimerStatus::Idle,
        }
    }
}

impl Reducible for TimerState {
    /// Reducer Action Type
    type Action = TimerStateAction;

    /// Reducer Function
    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            TimerStateAction::DecrementSeconds => {
                let is_seconds_zero = self.seconds == 0;
                let is_minutes_zero = self.minutes == 0;

                if is_minutes_zero && is_seconds_zero {
                    return Self {
                        minutes: 0,
                        seconds: 0,
                        status: TimerStatus::Idle,
                    }
                    .into();
                }

                Self {
                    status: self.status,
                    seconds: if is_seconds_zero {
                        59
                    } else {
                        self.seconds - 1
                    },
                    minutes: if is_seconds_zero {
                        self.minutes - 1
                    } else {
                        self.minutes
                    },
                }
                .into()
            }
            TimerStateAction::IncrementMinutes => Self {
                status: self.status,
                minutes: self.minutes + 1,
                seconds: self.seconds,
            }
            .into(),
            TimerStateAction::DecrementMinutes => Self {
                status: self.status,
                seconds: self.seconds,
                minutes: if self.minutes != 0 {
                    self.minutes - 1
                } else {
                    0
                },
            }
            .into(),
            TimerStateAction::Start => Self {
                status: TimerStatus::Running,
                seconds: self.seconds,
                minutes: self.minutes,
            }
            .into(),
            TimerStateAction::Pause => Self {
                status: TimerStatus::Paused,
                seconds: self.seconds,
                minutes: self.minutes,
            }
            .into(),
            TimerStateAction::Reset => Self {
                ..Default::default()
            }
            .into(),
        }
    }
}

#[function_component(Timer)]
pub fn timer() -> Html {
    let timer_state_handle = use_reducer(TimerState::default);

    let btn_classes = "rounded-md w-1/3 p-2 border text-2xl hover:bg-white hover:text-neutral-500";

    use_effect({
        let timer_state_handle = timer_state_handle.clone();

        move || {
            let mut interval: Option<Interval> = None;

            if timer_state_handle.status == TimerStatus::Running {
                interval = Some(Interval::new(1000, move || {
                    timer_state_handle.dispatch(TimerStateAction::DecrementSeconds)
                }));
            }

            move || {
                if let Some(i) = interval {
                    drop(i)
                }
            }
        }
    });

    let start_btn_onclick = {
        // let timer_state_handle = timer_state_handle.clone();
        // Callback::from(move |_| {
        //     timer_state_handle.dispatch(TimerStateAction::Start);
        // })
        Callback::from(move |_| {
            spawn_local(async move {
                timerStart().await;
            })
        })
    };

    let pause_btn_onclick = {
        let timer_state_handle = timer_state_handle.clone();
        Callback::from(move |_| {
            timer_state_handle.dispatch(TimerStateAction::Pause);
        })
    };

    let reset_btn_onclick = {
        let timer_state_handle = timer_state_handle.clone();
        Callback::from(move |_| {
            timer_state_handle.dispatch(TimerStateAction::Reset);
        })
    };

    let action_buttons = || -> Html {
        let timer_status = timer_state_handle.clone().status;
        match timer_status {
            TimerStatus::Idle => {
                html! {
                    <button
                        class={btn_classes}
                        onclick={start_btn_onclick}
                    >
                        {"Start"}
                    </button>
                }
            }
            TimerStatus::Running => {
                html! {
                    <button
                        class={btn_classes}
                        onclick={pause_btn_onclick}
                    >
                        {"Pause"}
                    </button>
                }
            }
            TimerStatus::Paused => {
                html! {
                    <div class="flex gap-2 justify-center">
                        <button
                            class={btn_classes}
                            onclick={start_btn_onclick}
                        >
                            {"Resume"}
                        </button>
                        <button
                            class={btn_classes}
                            onclick={reset_btn_onclick}
                        >
                            {"Reset"}
                        </button>
                    </div>
                }
            }
        }
    };

    // -- Util function --
    let pad_int_with_zero = |val: i32| -> String {
        if val < 10 {
            return format!("0{}", val.to_string());
        }
        val.to_string()
    };

    html! {
        <div class="h-full flex grow flex-col justify-center items-center text-center">
            <div class="text-9xl">
                <span>{pad_int_with_zero(timer_state_handle.minutes)}</span>
                {":"}
                <span>{pad_int_with_zero(timer_state_handle.seconds)}</span>
            </div>

            // Divider
            <div class="my-10"></div>

            <div class="w-full">
                {action_buttons()}
            </div>
        </div>
    }
}
