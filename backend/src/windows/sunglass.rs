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
            // Set window to be click through
            win.with_webview(|webview| {
                #[cfg(target_os = "macos")]
                unsafe {
                    let () = msg_send![webview.ns_window(), setIgnoresMouseEvents: true];
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
