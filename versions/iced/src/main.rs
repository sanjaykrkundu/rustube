use iced::widget::{button, column, text};
use iced::{Element, Theme};

#[derive(Debug, Clone)]
enum Message {
    OpenMusic,
    ToggleWidget,
}

struct Player {
    widget: bool,
}

impl Default for Player {
    fn default() -> Self {
        Self { widget: false }
    }
}

fn update(player: &mut Player, message: Message) -> iced::Task<Message> {
    match message {
        Message::OpenMusic => { let _ = webbrowser::open("https://music.youtube.com"); }
        Message::ToggleWidget => {
            player.widget = !player.widget;
            let size = if player.widget { iced::Size::new(360.0, 180.0) } else { iced::Size::new(1100.0, 760.0) };
            let level = if player.widget { iced::window::Level::AlwaysOnTop } else { iced::window::Level::Normal };
            return iced::window::latest().then(move |id| match id {
                Some(id) => iced::Task::batch([
                    iced::window::resize(id, size),
                    iced::window::set_level(id, level),
                ]),
                None => iced::Task::none(),
            });
        }
    }
    iced::Task::none()
}

fn view(player: &Player) -> Element<'_, Message> {
    let label = if player.widget { "Restore window" } else { "Widget mode" };
    column![
        text("Rustube · Iced edition"),
        text("Light native control panel. Playback runs in your default browser."),
        button("Open YouTube Music").on_press(Message::OpenMusic),
        button(label).on_press(Message::ToggleWidget),
    ]
    .spacing(8)
    .padding(10)
    .into()
}

fn main() -> iced::Result {
    iced::application(Player::default, update, view)
        .theme(Theme::Dark)
        .window_size((1100.0, 760.0))
        .run()
}
