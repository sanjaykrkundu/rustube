#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

use iced::widget::{button, column, container, row, text};
use iced::{Alignment, Element, Length, Theme, Task};
use iced_wry::{webview, WebViewConfig, WebViewController};

#[derive(Debug, Clone)]
enum Message {
    ToggleWidget,
    WebViewCreated(Result<(), String>),
}

struct Player {
    controller: WebViewController,
    widget: bool,
    error: Option<String>,
}

fn new() -> (Player, Task<Message>) {
    let mut player = Player {
        controller: WebViewController::new(
            WebViewConfig::default().url("https://music.youtube.com"),
        ),
        widget: false,
        error: None,
    };
    let task = player.controller.create_task(iced::window::Id::MAIN, Message::WebViewCreated);
    (player, task)
}

fn update(player: &mut Player, message: Message) -> Task<Message> {
    match message {
        Message::WebViewCreated(Ok(())) => player.controller.take_staged(),
        Message::WebViewCreated(Err(error)) => player.error = Some(error),
        Message::ToggleWidget => {
            player.widget = !player.widget;
            let size = if player.widget { iced::Size::new(360.0, 180.0) } else { iced::Size::new(1100.0, 760.0) };
            let level = if player.widget { iced::window::Level::AlwaysOnTop } else { iced::window::Level::Normal };
            return iced::window::latest().then(move |id| match id {
                Some(id) => Task::batch([
                    iced::window::resize(id, size),
                    iced::window::set_level(id, level),
                ]),
                None => Task::none(),
            });
        }
    }
    Task::none()
}

fn view(player: &Player) -> Element<'_, Message> {
    let label = if player.widget { "Restore window" } else { "Widget mode" };
    column![
        container(
            row![
                text("♪").size(22),
                column![text("Rustube").size(16), text("YouTube Music").size(12)],
                button(label).on_press(Message::ToggleWidget),
            ]
            .align_y(Alignment::Center)
            .spacing(12)
        )
        .padding([10, 16])
        .width(Length::Fill),
        if let Some(error) = &player.error {
            container(text(format!("Could not load YouTube Music: {error}")))
                .width(Length::Fill)
                .height(Length::Fill)
                .into()
        } else {
            webview::<Message>(&player.controller)
                .width(Length::Fill)
                .height(Length::Fill)
                .into()
        }
    ]
    .spacing(0)
    .into()
}

fn main() -> iced::Result {
    iced::application(new, update, view)
        .theme(Theme::Dark)
        .window_size((1100.0, 760.0))
        .run()
}
