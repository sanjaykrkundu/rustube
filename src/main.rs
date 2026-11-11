#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager, Size, WebviewUrl, WebviewWindowBuilder,
};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};
use std::{fs, path::PathBuf};

const NORMAL_SIZE: (f64, f64) = (1100.0, 760.0);
const WIDGET_SIZE: (f64, f64) = (360.0, 180.0);

fn page_tweak_paths(app: &tauri::AppHandle) -> Vec<PathBuf> {
    let mut paths = Vec::new();

    // The editable sidecar beside the executable takes precedence.
    if let Ok(exe) = std::env::current_exe() {
        if let Some(directory) = exe.parent() {
            paths.push(directory.join("minimal.js"));
        }
    }

    // Packaged Tauri installs keep bundled resources in the app resource folder.
    if let Ok(directory) = app.path().resource_dir() {
        paths.push(directory.join("minimal.js"));
        paths.push(directory.join("web").join("minimal.js"));
    }

    // Development fallback when running from `cargo tauri dev`.
    paths.push(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("web/minimal.js"));
    paths
}

fn show_main_window(app: &tauri::AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.unminimize();
        let _ = window.show();
        let _ = window.set_focus();
    }
}

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
            let show_item = MenuItem::with_id(
                app,
                "show",
                "Open YouTube Music",
                true,
                None::<&str>,
            )?;
            let quit_item = MenuItem::with_id(app, "quit", "Quit Rustube", true, None::<&str>)?;
            let tray_menu = Menu::with_items(app, &[&show_item, &quit_item])?;
            let tray_icon = app
                .default_window_icon()
                .cloned()
                .ok_or("Rustube tray icon is missing from the app configuration")?;

            let _tray = TrayIconBuilder::new()
                .icon(tray_icon)
                .menu(&tray_menu)
                .show_menu_on_left_click(false)
                .tooltip("Rustube — YouTube Music")
                .on_menu_event(|app, event| match event.id().as_ref() {
                    "show" => show_main_window(app),
                    "quit" => app.exit(0),
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        show_main_window(tray.app_handle());
                    }
                })
                .build(app)?;

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
            let window = WebviewWindowBuilder::new(
                app,
                "main",
                WebviewUrl::External("https://music.youtube.com/library".parse()?),
            )
            .title("YouTube Music")
            .inner_size(NORMAL_SIZE.0, NORMAL_SIZE.1)
            .min_inner_size(700.0, 500.0)
            .center()
            .on_page_load({
                let script_paths = page_tweak_paths(app.handle());
                move |window, payload| {
                    if payload.event() != tauri::webview::PageLoadEvent::Finished
                        || payload.url().host_str() != Some("music.youtube.com")
                    {
                        return;
                    }

                    // Read on every top-level load so editing this sidecar and
                    // refreshing the page applies the latest changes.
                    if let Some(script) = script_paths
                        .iter()
                        .find_map(|path| fs::read_to_string(path).ok())
                    {
                        let _ = window.eval(script);
                    }
                }
            })
            .build()?;

            window.on_window_event(|window, event| match event {
                tauri::WindowEvent::CloseRequested { api, .. } => {
                    api.prevent_close();
                    let _ = window.hide();
                }
                tauri::WindowEvent::Resized(size) => {
                    // Windows may report a zero-sized client area during
                    // minimization before `is_minimized()` reflects the state.
                    if size.width == 0
                        || size.height == 0
                        || window.is_minimized().unwrap_or(false)
                    {
                        let _ = window.hide();
                    }
                }
                tauri::WindowEvent::Focused(false)
                    if window.is_minimized().unwrap_or(false) =>
                {
                    let _ = window.hide();
                }
                _ => {}
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running Rustube");
}
