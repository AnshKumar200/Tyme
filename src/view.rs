use iced::{Element, Length::Fill, widget::{Column, Row, Text, button, row, text}};

use crate::app::{AppState, Message};


pub fn view(state: &AppState) -> Element<'_, Message> {
    let mut column = Column::new().spacing(10);

    let nav_bar = row![
        text("Tyme").size(24).width(Fill),
        button(text("Exit").size(24)).on_press(Message::Exit),
    ].spacing(8);

    column = column.push(nav_bar);

    for (key, value) in &state.programs {
        let row = Row::new()
            .spacing(5)
            .push(Text::new(format!("Program Name: {}", key)))
            .push(Text::new(format!("Time: {}", value)));

        column = column.push(row)
    }

    column.into()
}
