// --- System App Tray ---
use tauri::{
    AppHandle, CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu,
    SystemTrayMenuItem,
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
    // .with_menu(tray_menu)
    // .with_menu_on_left_click(false)
}

pub fn app_tray_event_handler(app: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::LeftClick {
            position, size: _, ..
        } => {
            println!("system tray received a LEFT click");

            let window_width = 200.00;
            let window_height = 200.00;

            let window = tauri::WindowBuilder::new(
                app,
                TRAY_WINDOW_LABEL, /* the unique window label */
                tauri::WindowUrl::App(TRAY_WINDOW_LABEL.into()),
            )
            .inner_size(window_width, window_height)
            // NOTE: depending on the screen this has to be changed.
            // Retina displays need x & y divided by 2 to by attached to icon
            .position(position.x / 2.0, position.y / 2.0)
            .decorations(true)
            .resizable(false)
            .decorations(false)
            .always_on_top(true)
            .skip_taskbar(true)
            .transparent(true)
            .focus();

            if let Err(_error) = window.build() {
                let tray_window = app.get_window(TRAY_WINDOW_LABEL).unwrap();
                tray_window
                    .close()
                    .unwrap_or_else(|_err| println!("Failed to close tray window."));
            }
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
