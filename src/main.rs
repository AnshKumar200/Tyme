use iced::Theme;

use crate::{app::{AppState, state_loop, update}, view::view};

mod utils;
mod app;
mod view;

fn main() -> iced::Result {
    iced::application(AppState::default, update, view)
        .title("Tyme")
        .theme(Theme::KanagawaDragon)
        .subscription(|_s| state_loop())
        .run()
}
