use yew::prelude::*;
use yew_router::prelude::*;

// #[path = "components/mod.rs"]
// mod components;
// use components::timer::Timer;

#[function_component(TrayMenu)]
pub fn tray_menu() -> Html {
    html! {
        <div class="rounded-xl bg-gray-800 flex flex-col grow pt-4 overflow-hidden border">

            <h1 class="text-2xl text-center underline w-full">{ "00:09" }</h1>

            // <Timer />
            <h2 class="text-xl w-full text-center">{"Open app"}</h2>
            <h2 class="text-xl w-full text-center">{"Quit"}</h2>
        </div>
    }
}
