use std::{sync::Arc, thread, time::Duration};

#[cfg(target_os = "macos")]
#[macro_use]
extern crate objc;

#[cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use tauri::{Builder, Manager};

mod commands;
use commands::{close_main_window, quit_app, show_main_window};

mod app_tray;
use app_tray::{app_tray_event_handler, init_app_tray};

mod windows;

// Apps
mod apps;
use apps::timer::{spawn_timer_thread, timer_start, Timer};

use crate::{
    apps::timer::{
        timer_finished_close_window, timer_finished_start_new, timer_get_state, timer_pause,
        timer_reset,
    },
    commands::{close_sunglass_window, open_sunglass_window},
};

// -------------------------
// --- CONSTANTS -----------
// --- WINDOWS -----
const MAIN_WINDOW_LABEL: &str = "main";
const TRAY_WINDOW_LABEL: &str = "tray";
const TIMER_FINISHED_WINDOW_LABEL: &str = "timer-finished";
const TIMER_FINISHED_OVERLAY_WINDOW_LABEL: &str = "timer-finished-overlay";
const SUNGLASS_OVERLAY_WINDOW_LABEL: &str = "sunglass-overlay";
// --- TRAY ITEMS ---
const TRAY_ITEM_OPEN_APP_ID: &str = "open_app";
const TRAY_ITEM_QUIT_ID: &str = "quit";
const TRAY_ITEM_TIMER_ID: &str = "timer";
// -------------------------

fn main() {
    let app_tray = init_app_tray();

    Builder::default()
        .plugin(tauri_plugin_positioner::init())
        // --- Store
        .manage(Timer {
            ..Default::default()
        })
        // --- Setup
        .setup(|app| {
            // (MacOS) Makes app run in the background, and hides the Dock icon
            // Without this, switching virtual desktop pauses the event loop
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            let app_handle = app.app_handle();
            let timer_store = app.try_state::<Timer>().unwrap();
            let timer_state_handler = Arc::clone(&timer_store.store);
            spawn_timer_thread(&app_handle, timer_state_handler);

            Ok(())
        })
        // --- Tray
        .system_tray(app_tray)
        .on_system_tray_event(app_tray_event_handler)
        // --- Commands
        .invoke_handler(tauri::generate_handler![
            close_main_window,
            show_main_window,
            quit_app,
            timer_start,
            timer_get_state,
            timer_pause,
            timer_reset,
            timer_finished_start_new,
            timer_finished_close_window,
            open_sunglass_window,
            close_sunglass_window
        ])
        // --- Window events
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::Focused(focused) => {
                // Close tray-menu when window loses focus
                if !focused {
                    if event.window().label() == TRAY_WINDOW_LABEL {
                        thread::spawn(move || {
                            // Quick clicking Tray icon causes app to crash (potentially menu building is not finished before trying to close)
                            // Adding short sleep prevents crashing
                            thread::sleep(Duration::from_millis(400));

                            if let Err(error) = event.window().close() {
                                println!("Failed to close tray: {}", error);
                            }
                        });
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
