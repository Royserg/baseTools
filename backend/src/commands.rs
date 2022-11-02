// --- Commands ---
use tauri::{Manager, Window};

use crate::MAIN_WINDOW_LABEL;

#[tauri::command]
pub fn close_main_window(window: Window) {
    let main_window = window.get_window(MAIN_WINDOW_LABEL).unwrap();
    main_window.close().unwrap();
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
