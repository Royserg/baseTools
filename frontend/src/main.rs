use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

fn main() {
    yew::start_app::<App>();
}

// --- Triggers for js Tauri API commands ---
#[wasm_bindgen(module = "/public/glue.js")]
extern "C" {
    #[wasm_bindgen(js_name = hideWindow)]
    pub async fn hideWindow();
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        // Main App wrapper
        <div class="rounded-xl text-white w-screen h-screen border-1 flex flex-col">
            <AppBar />

            // Hero section
            <div class="w-100 bg-neutral-500">
                <div class="container mx-auto">
                    <div class="flex flex-col items-center justify-center h-96">
                        <h1 class="text-5xl font-bold">{"Fullstack Rust"}</h1>
                    </div>
                </div>
            </div>

            // Content
            <div class="rounded-b-xl bg-slate-800 p-4 w-100 grow">
            </div>
        </div>
    }
}

#[function_component(AppBar)]
pub fn app_bar() -> Html {
    let nav_control_class = "rounded-full bg-gray-400 w-3 h-3";

    let close_onclick = Callback::from(move |_| handle_nav_close_btn_click());

    html! {
        <nav class="rounded-t-xl w-100 bg-neutral-500 px-4 py-3 flex gap-x-2" data-tauri-drag-region="default">
            <button
                id="titlebar-close"
                onclick={close_onclick}
                class={classes!(nav_control_class, "bg-red-400", "flex", "items-center", "justify-center")}
            >
                <span class={classes!("text-xs", "text-red-400", "hover:text-black")}>{"x"}</span>
            </button>
            <button disabled={true} class={nav_control_class}></button>
            <button disabled={true} class={nav_control_class}></button>
        </nav>
    }
}

// https://dev.to/stevepryde/create-a-desktop-app-in-rust-using-tauri-and-yew-2bhe
fn handle_nav_close_btn_click() {
    spawn_local(async move {
        hideWindow().await;
    });
}
