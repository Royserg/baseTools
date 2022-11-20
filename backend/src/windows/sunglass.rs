use tauri::{AppHandle, WindowBuilder, WindowUrl};

use crate::SUNGLASS_OVERLAY_WINDOW_LABEL;

pub fn build_sunglass_window(app: &AppHandle) -> Result<(), &str> {
    let win_setup = WindowBuilder::new(
        app,
        SUNGLASS_OVERLAY_WINDOW_LABEL,
        WindowUrl::App("/sunglass".into()),
    )
    .resizable(false)
    .decorations(false)
    .skip_taskbar(true)
    .transparent(true)
    .focused(false)
    .maximized(true)
    .always_on_top(true);

    match win_setup.build() {
        Ok(win) => {
            win.with_webview(|webview| {
                #[cfg(target_os = "macos")]
                unsafe {
                    // Set window to be click through
                    let () = msg_send![webview.ns_window(), setIgnoresMouseEvents: true];
                    // Assign app to all desktops
                    // https://stackoverflow.com/a/12233351
                    let () = msg_send![webview.ns_window(), setCollectionBehavior: true];
                }
            })
            .unwrap();
            Ok(())
        }
        Err(error) => {
            println!("Failed to build sunGlass window: {}", error);
            Err("Failed to build sunGlass window.")
        }
    }
}
