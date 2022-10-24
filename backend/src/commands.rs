// --- Commands ---
use tauri::{Manager, Window};

use crate::MAIN_WINDOW_LABEL;

#[tauri::command]
pub fn hide_main_window(window: Window) {
    let main_window = window.get_window(MAIN_WINDOW_LABEL).unwrap();
    // TOOD: use `close` but then `show` should re-create the main window (need a function for win creation)
    // With this, main window could be opened in the current Desktop
    // main_window.close().unwrap();
    main_window.hide().unwrap();
}

#[tauri::command]
pub fn show_main_window(window: Window) {
    let main_window = window.get_window(MAIN_WINDOW_LABEL).unwrap();
    main_window.show().unwrap();
}

#[tauri::command]
pub fn quit_app() {
    std::process::exit(0);
}
