use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

#[path = "components/mod.rs"]
mod components;

#[path = "../../components/mod.rs"]
mod shared;

use components::timer::Timer;
use shared::app_bar::AppBar;

#[function_component(TimeChamber)]
pub fn time_chamber() -> Html {
    let history = use_history().unwrap();

    // let onclick = Callback::once(move |_| history.push(Route::Home));
    html! {
        <>
            <AppBar />
            <div class="rounded-b-xl bg-neutral-500 flex flex-col grow pt-4 overflow-hidden">

                <h1 class="text-4xl text-center underline">{ "Time Chamber" }</h1>

                // TODO: uncomment when enabling home page
                // <div class="flex">
                //     <button {onclick} class="rounded-md p-2 ml-4 -mt-10 bg-slate-800">{ "Go Back" }</button>
                // </div>

                <Timer />
            </div>
        </>
    }
}
