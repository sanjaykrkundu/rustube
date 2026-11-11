#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

use eframe::{egui, NativeOptions};
use wry::{
    dpi::{LogicalPosition, LogicalSize},
    Rect, WebView, WebViewBuilder,
};

const URL: &str = "https://music.youtube.com";

#[derive(Default)]
struct Player {
    webview: Option<WebView>,
    error: Option<String>,
    widget: bool,
}

impl eframe::App for Player {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        let content_origin = ui.min_rect().min;

        egui::Frame::new()
            .fill(egui::Color32::from_rgb(24, 24, 27))
            .inner_margin(egui::Margin::symmetric(16, 10))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("●").color(egui::Color32::from_rgb(255, 35, 67)));
                    ui.label(egui::RichText::new("Rustube").size(16.0).strong().color(egui::Color32::WHITE));
                    ui.separator();
                    ui.label(egui::RichText::new("YOUTUBE MUSIC").size(11.0).color(egui::Color32::from_gray(165)));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button(if self.widget { "↗ Restore" } else { "▣ Widget" }).clicked() {
                            self.widget = !self.widget;
                            let size = if self.widget { egui::vec2(380.0, 220.0) } else { egui::vec2(1100.0, 760.0) };
                            ui.ctx().send_viewport_cmd(egui::ViewportCommand::InnerSize(size));
                            ui.ctx().send_viewport_cmd(egui::ViewportCommand::WindowLevel(
                                if self.widget { egui::WindowLevel::AlwaysOnTop } else { egui::WindowLevel::Normal },
                            ));
                        }
                    });
                });
            });

        let view_rect = ui.available_rect_before_wrap();
        let (view_rect, _) = ui.allocate_exact_size(view_rect.size(), egui::Sense::hover());
        let local_min = view_rect.min - content_origin;
        let bounds = Rect {
            position: LogicalPosition::new(local_min.x as f64, local_min.y as f64).into(),
            size: LogicalSize::new(view_rect.width() as f64, view_rect.height() as f64).into(),
        };

        if self.webview.is_none() && self.error.is_none() {
            match WebViewBuilder::new()
                .with_url(URL)
                .with_bounds(bounds)
                .build_as_child(frame)
            {
                Ok(webview) => self.webview = Some(webview),
                Err(error) => self.error = Some(error.to_string()),
            }
        }

        if let Some(webview) = &self.webview {
            if let Err(error) = webview.set_bounds(bounds) {
                self.error = Some(error.to_string());
            }
        }

        if let Some(error) = &self.error {
            ui.label(format!("Could not load YouTube Music: {error}"));
        }
    }
}

fn main() -> eframe::Result {
    let options = NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1100.0, 760.0]),
        renderer: eframe::Renderer::Glow,
        ..Default::default()
    };
    eframe::run_native("Rustube · egui", options, Box::new(|_| Ok(Box::<Player>::default())))
}
