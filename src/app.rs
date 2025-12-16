use std::{collections::HashMap, time::Duration};
use iced::Subscription;
use crate::utils::get_active_window;

#[derive(Default)]
pub struct AppState {
    pub programs: HashMap<String, u32>,
    pub total_time: u32,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Tick,
    Exit,
}

pub fn update(state: &mut AppState, message: Message) {
    match message {
        Message::Tick => {
            if let Some(app_name) = get_active_window() {
                *state.programs.entry(app_name).or_insert(0) += 1;
                state.total_time += 1;
            }
        }
        Message::Exit => {
            println!("Exit!")
        }
    }
}

pub fn state_loop() -> Subscription<Message> {
    iced::time::every(Duration::from_secs(1)).map(|_| Message::Tick)
}
