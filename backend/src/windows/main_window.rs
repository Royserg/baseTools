use tauri::{AppHandle, WindowBuilder, WindowUrl};

use crate::MAIN_WINDOW_LABEL;

pub fn build_main_window(app: &AppHandle) -> Result<(), &str> {
    // Window config
    const WINDOW_HEIGHT: f64 = 600.0;
    const WINDOW_WIDTH: f64 = 800.0;

    let win_setup = WindowBuilder::new(app, MAIN_WINDOW_LABEL, WindowUrl::App("/".into()))
        .inner_size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .resizable(false)
        .decorations(false)
        .skip_taskbar(true)
        .transparent(true)
        .center()
        .focused(false);

    match win_setup.build() {
        Ok(_win) => Ok(()),
        Err(error) => {
            println!("Failed to build main window: {}", error);
            Err("Failed to build main window.")
        }
    }
}
