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
- [`versions/egui`](versions/egui) is an **egui** widget shell that opens YouTube Music in your browser.
- [`versions/iced`](versions/iced) is an **Iced** widget shell that opens YouTube Music in your browser.

The egui and Iced shells stay lighter because playback runs in your existing
browser. The Tauri version embeds YouTube Music in the app window. The YouTube
Music page itself remains the largest runtime cost in that version.

## Build on GitHub

The workflow at `.github/workflows/build-windows.yml` builds all three Windows
versions and uploads their outputs as downloadable workflow artifacts. Push
this project to a GitHub repository, then run **Actions → Build Windows apps**.
