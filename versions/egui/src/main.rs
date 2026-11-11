use eframe::{egui, NativeOptions};
const URL: &str = "https://music.youtube.com";

#[derive(Default)]
struct Player {
    widget: bool,
}

impl eframe::App for Player {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Rustube · egui edition");
            ui.label("Light native control panel. Playback runs in your default browser.");
            if ui.button("Open YouTube Music").clicked() {
                let _ = webbrowser::open(URL);
            }
            if ui.button(if self.widget { "Restore window" } else { "Widget mode" }).clicked() {
                self.widget = !self.widget;
                let size = if self.widget { egui::vec2(360.0, 180.0) } else { egui::vec2(1100.0, 760.0) };
                ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(size));
                ctx.send_viewport_cmd(egui::ViewportCommand::WindowLevel(if self.widget { egui::WindowLevel::AlwaysOnTop } else { egui::WindowLevel::Normal }));
            }
            ui.label("Widget mode keeps this small player panel above other windows.");
        });
        let _ = frame;
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
