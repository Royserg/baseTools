use std::rc::Rc;

use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;
use gloo_timers::callback::Interval;
use weblog::console_log;

#[function_component(TimeChamber)]
pub fn time_chamber() -> Html {
    let history = use_history().unwrap();

    let onclick = Callback::once(move |_| history.push(Route::Home));
    html! {
        <div class="rounded-b-xl bg-neutral-500 flex flex-col grow pt-4 overflow-hidden">
            <h1 class="text-4xl text-center underline">{ "Time Chamber" }</h1>
            <div class="flex">
                <button {onclick} class="rounded-md p-2 ml-4 -mt-10 bg-slate-800">{ "Go Back" }</button>
            </div>

            <Timer />
        </div>
    }
}

// --- TIMER ---
#[derive(Default, Debug, PartialEq, Copy, Clone)]
enum TimerStatus {
    #[default]
    Idle,
    Running,
    Stopped,
}

enum TimerStateAction {
    // IncrementSeconds,
    DecrementSeconds,
    IncrementMinutes,
    DecrementMinutes,
    Start,
    // Pause,
    // Stop,
}

// #[derive(Default)]
struct TimerState {
    minutes: i32,
    seconds: i32,
    status: TimerStatus,
}

impl Default for TimerState {
    fn default() -> Self {
        TimerState {
            minutes: 0,
            seconds: 33,
            status: TimerStatus::Idle,
            // status: TimerStatus::Running,
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
            TimerStateAction::Start => {
                console_log!("GOT START ACTION");

                Self {
                    status: TimerStatus::Running,
                    seconds: self.seconds,
                    minutes: self.minutes,
                }
                .into()
            }
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

    let action_btn_onclick = {
        let timer_state_handle = timer_state_handle.clone();

        Callback::from(move |_| {
            timer_state_handle.dispatch(TimerStateAction::Start);
            console_log!("Button Clicked");
        })
        // let timer_state = timer_state.clone();

        // Callback::from(move |_| match *timer_state {
        //     TimerState::Idle => timer_state.set(TimerState::Running),
        //     TimerState::Running => timer_state.set(TimerState::Stopped),
        //     TimerState::Stopped => timer_state.set(TimerState::Idle),
        // })
    };

    let action_buttons = || -> Html {
        html! {
            <button
                class={btn_classes}
                onclick={action_btn_onclick}
            >
                {"Start"}
            </button>
        }
        // match *timer_state {
        //     TimerState::Idle => {
        //         html! {
        //             <button
        //                 class={btn_classes}
        //                 onclick={action_btn_onclick}
        //             >
        //                 {"Start"}
        //             </button>
        //         }
        //     }
        //     TimerState::Running => {
        //         html! {
        //             <>
        //                 <button
        //                     class={btn_classes}
        //                     onclick={action_btn_onclick}
        //                 >
        //                     {"Stop"}
        //                 </button>
        //             </>
        //         }
        //     }
        //     TimerState::Stopped => {
        //         html! {
        //             <>
        //                 <button
        //                     class={btn_classes}
        //                     onclick={action_btn_onclick}
        //                 >
        //                     {"Restart"}
        //                 </button>
        //             </>
        //         }
        //     }
        // }
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
