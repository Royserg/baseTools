#[cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use tauri::{Builder, Manager};

mod commands;
use commands::{hide_main_window, quit_app, show_main_window};

mod app_tray;
use app_tray::{app_tray_event_handler, init_app_tray};

// Apps
mod apps;
use apps::timer::{timer_start, Timer};

// -------------------------
// --- CONSTANTS -----------
// --- WINDOWS -----
const MAIN_WINDOW_LABEL: &str = "main";
const TRAY_WINDOW_LABEL: &str = "tray-menu";
// --- TRAY ITEMS ---
const TRAY_ITEM_OPEN_APP_ID: &str = "open_app";
const TRAY_ITEM_QUIT_ID: &str = "quit";
// -------------------------

fn main() {
    let app_tray = init_app_tray();

    Builder::default()
        .manage(Timer {
            ..Default::default()
        })
        .setup(|app| {
            let _window = app.get_window(MAIN_WINDOW_LABEL).unwrap();
            _window.hide().unwrap();

            Ok(())
        })
        .system_tray(app_tray)
        .on_system_tray_event(app_tray_event_handler)
        .invoke_handler(tauri::generate_handler![
            hide_main_window,
            show_main_window,
            quit_app,
            timer_start
        ])
        // --- Window events
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::Focused(focused) => {
                // Close tray-menu when window loses focus
                if !focused {
                    if event.window().label() == TRAY_WINDOW_LABEL {
                        event.window().close().unwrap();
                    }
                }
            }
            _ => {}
        })
        // --- Build
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|_app_handle, event| match event {
            tauri::RunEvent::ExitRequested { api, .. } => {
                api.prevent_exit();
            }
            _ => {}
        });
}
