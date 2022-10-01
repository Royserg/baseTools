use yew::prelude::*;

mod components;
use components::app_bar::AppBar;

fn main() {
    yew::start_app::<App>();
}

// --- Main Component ---
#[function_component(App)]
pub fn app() -> Html {
    html! {
        // Main App wrapper
        <div class="rounded-xl text-white w-screen h-screen border-1 flex flex-col">
            <AppBar />

            // Hero section
            <div class="w-100 bg-neutral-500">
                <div class="container mx-auto">
                    <div class="flex flex-col items-center justify-center h-44">
                        <h1 class="text-5xl font-bold">{"Fullstack Rust"}</h1>
                        <div class="border w-1/3 p-4 flex items-center justify-center">{"Logo"}</div>
                    </div>
                </div>
            </div>

            // Content
            <div class="rounded-b-xl bg-slate-800 p-4 w-100 grow">
                // Link to Clock/Pomodoro mini-app
                <button class="border-2">{"Clockster"}</button>
                // <Timer />
            </div>
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
