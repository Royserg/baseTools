// --- System App Tray ---
use tauri::{
    AppHandle, CustomMenuItem, LogicalPosition, Manager, PhysicalPosition, SystemTray,
    SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem,
};

use crate::{MAIN_WINDOW_LABEL, TRAY_ITEM_OPEN_APP_ID, TRAY_ITEM_QUIT_ID, TRAY_WINDOW_LABEL};

pub fn init_app_tray() -> SystemTray {
    let open_app = CustomMenuItem::new(TRAY_ITEM_OPEN_APP_ID, "Open App");
    let quit = CustomMenuItem::new(TRAY_ITEM_QUIT_ID, "Quit");

    let _tray_menu = SystemTrayMenu::new()
        .add_item(open_app)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);

    SystemTray::new().with_id("tray")
}

pub fn build_app_tray_menu(app: &AppHandle, position: PhysicalPosition<f64>) {
    let window_width = 200.00;
    let window_height = 200.00;

    let win_setup = tauri::WindowBuilder::new(
        app,
        TRAY_WINDOW_LABEL, /* the unique window label */
        tauri::WindowUrl::App(TRAY_WINDOW_LABEL.into()),
    )
    .inner_size(window_width, window_height)
    .resizable(false)
    .decorations(false)
    .always_on_top(true)
    .skip_taskbar(true)
    .transparent(true)
    .focus();

    match win_setup.build() {
        Ok(win) => {
            let scale_factor = win.scale_factor().unwrap();
            let l_position =
                LogicalPosition::new(position.x / scale_factor, position.y / scale_factor);
            win.set_position(l_position).unwrap();
        }
        Err(_error) => panic!("Failed to build tray window"),
    }
}

pub fn app_tray_event_handler(app: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::LeftClick {
            position, size: _, ..
        } => {
            println!("system tray received a LEFT click");

            build_app_tray_menu(app, position)
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
