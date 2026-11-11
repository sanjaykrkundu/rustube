# Rustube

A lightweight desktop app for YouTube Music, built with Rust and Tauri. It loads
the official YouTube Music website in the operating system's WebView, so sign-in,
playlists, and playback use the site's own features.

The app opens directly to your library. Minimal view starts on and hides the site
logo, navigation rail, player UI, and video surface while keeping search,
sign-in/account controls, and your library visible. Use the **Minimal view: On/Off**
button in the lower-right corner to toggle it; your choice is saved in the app's
YouTube Music WebView. Hiding the video surface does not guarantee that YouTube
stops loading or decoding the media stream. These tweaks apply only on
`music.youtube.com` inside this app.

Rustube stays available from the system tray. Left-click the tray icon to reopen
the app; right-click for **Open YouTube Music** or **Quit Rustube**. Closing or
minimizing the window sends it to the tray.

## Edit the page tweaks without rebuilding

The app reads `minimal.js` beside `rustube.exe` each time YouTube Music finishes
loading. Edit that file, then refresh the page (Ctrl+R) to try the change. The
GitHub Actions workflow provides a portable ZIP containing both files. The
installer also includes a bundled fallback copy when no sidecar is present.

## Widget mode

Press **Ctrl+Shift+M** (or **Cmd+Shift+M** on macOS) to switch between the normal
window and a compact, always-on-top 360 × 180 widget. The same page remains open
so playback continues while switching modes.

## Build

Install Rust and the [Tauri v2 prerequisites](https://v2.tauri.app/start/prerequisites/),
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

## Build on GitHub

The workflow at `.github/workflows/build-windows.yml` builds the Tauri Windows
installer and a portable ZIP with `rustube.exe` and its editable `minimal.js`.
Push this project to GitHub, then run **Actions → Build Rustube (Windows)**.
