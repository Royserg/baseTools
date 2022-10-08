use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::{classes, function_component, html, Callback};

// --- Triggers for js Tauri API commands ---
#[wasm_bindgen(module = "/public/glue.js")]
extern "C" {
    #[wasm_bindgen(js_name = hideMainWindow)]
    pub async fn hideMainWindow();
}

fn handle_nav_close_btn_click() -> () {
    spawn_local(async move {
        hideMainWindow().await;
    });
}

#[function_component(AppBar)]
pub fn app_bar() -> Html {
    let nav_control_class = "rounded-full bg-gray-400 w-3 h-3";

    let close_click = Callback::from(move |_| handle_nav_close_btn_click());

    html! {
        <nav class="rounded-t-xl w-100 bg-neutral-500 px-4 py-3 flex gap-x-2" data-tauri-drag-region="default">
            <button
                id="titlebar-close"
                onclick={close_click}
                class={classes!(nav_control_class, "bg-red-400", "flex", "items-center", "justify-center")}
            >
                <span class={classes!("text-xs", "text-red-400", "hover:text-black")}>{"x"}</span>
            </button>
            <button disabled={true} class={nav_control_class}></button>
            <button disabled={true} class={nav_control_class}></button>
        </nav>
    }
}
