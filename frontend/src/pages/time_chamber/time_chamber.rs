use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

#[function_component(TimeChamber)]
pub fn time_chamber() -> Html {
    let history = use_history().unwrap();

    let onclick = Callback::once(move |_| history.push(Route::Home));
    html! {
        <div class="rounded-b-xl bg-neutral-500 h-96">
            <h1 class="text-5xl text-center">{ "Time Chamber" }</h1>
            <div class="flex ">
                <button {onclick} class="rounded-md p-2 ml-4 -mt-10 bg-slate-800">{ "Go Back" }</button>
            </div>

            <Timer />
        </div>
    }
}

#[function_component(Timer)]
pub fn timer() -> Html {
    let counter = use_state(|| 20);

    html! {
        <div class="w-100 text-center">
            <h1>{"Timer"}</h1>
            <div>{format!("{}", *counter)}</div>
        </div>
    }
}
