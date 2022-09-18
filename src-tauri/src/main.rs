#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{
    AppHandle, Builder, CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu,
    SystemTrayMenuItem, Window,
};

// --- WINDOWS ---
const MAIN_WINDOW_LABEL: &str = "main";
// --- TRAY ITEMS ---
const TRAY_ITEM_OPEN_APP_ID: &str = "open_app";
const TRAY_ITEM_QUIT_ID: &str = "quit";

fn main() {
    let app_tray = init_app_tray();

    Builder::default()
        .setup(|app| {
            let _window = app.get_window(MAIN_WINDOW_LABEL).unwrap();

            Ok(())
        })
        .system_tray(app_tray)
        .on_system_tray_event(app_tray_event_handler)
        .invoke_handler(tauri::generate_handler![hide_window])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// --- System App Tray ---
fn init_app_tray() -> SystemTray {
    let open_app = CustomMenuItem::new(TRAY_ITEM_OPEN_APP_ID, "Open App");
    let quit = CustomMenuItem::new(TRAY_ITEM_QUIT_ID, "Quit");

    let tray_menu = SystemTrayMenu::new()
        .add_item(open_app)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);

    SystemTray::new()
        .with_id("tray")
        .with_menu_on_left_click(false)
        .with_menu(tray_menu)
}

fn app_tray_event_handler(app: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::LeftClick {
            position: _,
            size: _,
            ..
        } => {
            println!("system tray received a LEFT click");
        }
        SystemTrayEvent::RightClick {
            position: _,
            size: _,
            ..
        } => {
            println!("system tray received a RIGHT click");
        }
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            TRAY_ITEM_OPEN_APP_ID => {
                let window = app.get_window(MAIN_WINDOW_LABEL).unwrap();
                window.show().unwrap();
            }
            TRAY_ITEM_QUIT_ID => {
                std::process::exit(0);
            }
            _ => {}
        },
        _ => {}
    }
}

#[tauri::command]
fn hide_window(window: Window) {
    window.hide().unwrap();
}
