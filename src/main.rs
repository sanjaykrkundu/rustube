#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Manager, Size, WebviewUrl, WebviewWindowBuilder};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

const NORMAL_SIZE: (f64, f64) = (1100.0, 760.0);
const WIDGET_SIZE: (f64, f64) = (360.0, 180.0);

fn set_widget_mode(app: &tauri::AppHandle, enabled: bool) {
    if let Some(window) = app.get_webview_window("main") {
        let (width, height) = if enabled { WIDGET_SIZE } else { NORMAL_SIZE };
        let _ = window.set_size(Size::Logical(tauri::LogicalSize::new(width, height)));
        let _ = window.set_always_on_top(enabled);
        let _ = window.set_resizable(!enabled);
        // Keep the title bar so the compact window remains easy to drag.
        let _ = window.set_decorations(true);
        let _ = window.set_title(if enabled {
            "YouTube Music — Widget"
        } else {
            "YouTube Music"
        });
        let _ = window.center();
    }
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .setup(|app| {
            let shortcut = "CommandOrControl+Shift+M";
            app.global_shortcut().on_shortcut(shortcut, |app, _, event| {
                if event.state == ShortcutState::Pressed {
                    let is_widget = app
                        .get_webview_window("main")
                        .and_then(|window| window.is_always_on_top().ok())
                        .unwrap_or(false);
                    set_widget_mode(app, !is_widget);
                }
            })?;

            // Keep the service in one WebView so its sign-in and playback session
            // remain intact as the native window changes between full and widget size.
            let _window = WebviewWindowBuilder::new(
                app,
                "main",
                WebviewUrl::External("https://music.youtube.com".parse()?),
            )
            .title("YouTube Music")
            .inner_size(NORMAL_SIZE.0, NORMAL_SIZE.1)
            .min_inner_size(700.0, 500.0)
            .center()
            .build()?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running Rustube");
}
