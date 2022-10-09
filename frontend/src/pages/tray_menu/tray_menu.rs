use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

// #[path = "components/mod.rs"]
// mod components;
// use components::timer::Timer;

// --- Triggers for js Tauri API commands ---
#[wasm_bindgen(module = "/public/glue.js")]
extern "C" {
    #[wasm_bindgen(js_name = showMainWindow)]
    pub async fn showMainWindow();
    #[wasm_bindgen(js_name = quitApp)]
    pub async fn quitApp();
}

fn handle_open_app_btn_click() -> () {
    spawn_local(async move {
        showMainWindow().await;
    });
}

fn handle_quit_btn_click() -> () {
    spawn_local(async move {
        quitApp().await;
    });
}

#[function_component(TrayMenu)]
pub fn tray_menu() -> Html {
    let menu_btn_class = "text-md w-full text-center hover:bg-gray-700 rounded-md";

    let open_app_onclick = Callback::from(move |_| handle_open_app_btn_click());
    let quit_onclick = Callback::from(move |_| handle_quit_btn_click());

    html! {
        <div class="rounded-xl bg-gray-800 flex flex-col justify-between grow pt-4 overflow-hidden border">

            // Timer display and controls
            <div class="w-full mx-1">
                // TODO: Read value from the event coming from backend
                <h2 class="text-2xl text-center">{ "00:09" }</h2>
            </div>

            // --- Menu buttons ---
            <div class="w-100 mx-1">
                <button onclick={open_app_onclick} class={menu_btn_class}>{"Open app"}</button>

                // Divider
                <div class="border"></div>

                <button onclick={quit_onclick} class={menu_btn_class}>{"Quit"}</button>
            </div>
        </div>
    }
}
