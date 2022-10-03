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
// enum TimerValueAction {
//     IncrementSeconds,
//     DecrementSeconds,
//     IncrementMinutes,
//     DecrementMinutes,
//     Start,
//     Pause,
//     Stop,
// }

// #[derive(Default)]
// struct SecondsState {
//     minutes: usize,
//     seconds: usize,
// }

#[derive(PartialEq)]
pub enum TimerState {
    Idle,
    Running,
    Stopped,
}

#[function_component(Timer)]
pub fn timer() -> Html {
    let timer = use_state(|| 40);
    let timer_state = use_state(|| TimerState::Idle);

    let btn_classes = "rounded-md w-1/3 p-2 border text-2xl hover:bg-white hover:text-neutral-500";

    let timer_interval: Interval;

    // use_effect_with_deps(callback, deps);

    // let current_timer_state = (*timer_state).clone();
    use_effect_with_deps(
        move |_| {
            // let timer_state = timer_state.clone();
            // let timer = timer.clone();
            // let interval = Interval::new(1000, move || timer.set(*timer - 1));
            move || ()
        },
        (),
    );

    // // console_log!("TIMER STATE CHANGED");
    // timer_interval = Interval::new(1_000, move || {
    //     timer.set(*timer - 1);
    // })
    // .forget();

    // use_effect_with_deps(
    // move |_| {
    //         // ...
    //         || ()
    //     },
    //     (), // dependents
    // );

    let action_btn_onclick = {
        let timer_state = timer_state.clone();

        Callback::from(move |_| match *timer_state {
            TimerState::Idle => timer_state.set(TimerState::Running),
            TimerState::Running => timer_state.set(TimerState::Stopped),
            TimerState::Stopped => timer_state.set(TimerState::Idle),
        })
    };

    let action_buttons = || -> Html {
        match *timer_state {
            TimerState::Idle => {
                html! {
                    <button
                        class={btn_classes}
                        onclick={action_btn_onclick}
                    >
                        {"Start"}
                    </button>
                }
            }
            TimerState::Running => {
                html! {
                    <>
                        <button
                            class={btn_classes}
                            onclick={action_btn_onclick}
                        >
                            {"Stop"}
                        </button>
                    </>
                }
            }
            TimerState::Stopped => {
                html! {
                    <>
                        <button
                            class={btn_classes}
                            onclick={action_btn_onclick}
                        >
                            {"Restart"}
                        </button>
                    </>
                }
            }
        }
    };

    // let timer = timer.clone();
    html! {
        <div class="h-full flex grow flex-col justify-center items-center text-center">
            <div class="text-9xl">
                <span>{*timer}</span>
                {":"}
                <span>{"00"}</span>
            </div>

            // Divider
            <div class="my-10"></div>

            <div class="w-full">
                {action_buttons()}
            </div>
        </div>
    }
}
