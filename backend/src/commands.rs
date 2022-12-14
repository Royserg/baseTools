// --- Commands ---
use tauri::{Manager, Window};

use crate::{windows::main_window::build_main_window, MAIN_WINDOW_LABEL};

#[tauri::command]
pub fn close_main_window(window: Window) {
    let main_window = window.get_window(MAIN_WINDOW_LABEL).unwrap();
    main_window.close().unwrap();
}

#[tauri::command]
pub fn show_main_window(window: Window) {
    let app_handle = window.app_handle();
    match app_handle.get_window(MAIN_WINDOW_LABEL) {
        Some(win) => win.show().unwrap(),
        None => build_main_window(&app_handle).unwrap(),
    }
}

#[tauri::command]
pub fn quit_app() {
    std::process::exit(0);
}
