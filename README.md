# Rustube

A small native desktop window for YouTube Music, built with Rust and Tauri.
The app opens the official YouTube Music site in a native WebView; sign-in,
playlists, and playback are provided by YouTube Music itself.

## Widget mode

Press **Ctrl+Shift+M** (or **Cmd+Shift+M** on macOS) to toggle a compact,
always-on-top 360 × 180 window. Press the shortcut again to restore the full
window. The same WebView stays open so playback continues through the resize.

## Build

Install the Rust toolchain and the [Tauri v2 prerequisites](https://tauri.app/start/prerequisites/),
then run:

```sh
cargo run
```

To create a distributable build:

```sh
cargo tauri build
```

The app needs an internet connection. YouTube account features and availability
are subject to Google's terms and regional support.

## Framework alternatives

- This root project is the **Tauri** version.
- [`versions/egui`](versions/egui) embeds YouTube Music using Wry.
- [`versions/iced`](versions/iced) embeds YouTube Music using `iced_wry`.

All three versions load YouTube Music inside their app window using the system
WebView. The page itself is the main runtime cost. Windows builds launch as GUI
apps without opening a console window.

## Build on GitHub

The workflow at `.github/workflows/build-windows.yml` builds all three Windows
versions and uploads their outputs as downloadable workflow artifacts. Push
this project to a GitHub repository, then run **Actions → Build Windows apps**.
