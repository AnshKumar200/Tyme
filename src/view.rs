use iced::{
    Border, Element, Length::Fill, widget::{Column, button, column, row, text}
};

use crate::{app::{AppState, Message}, hero_section::hero_section};

pub fn view(state: &AppState) -> Element<'_, Message> {
    let mut column = Column::new().spacing(10).padding(10);

    let nav_bar = row![
        text("Tyme").size(24).width(Fill),
        button(text("Exit").size(24))
            .on_press(Message::Exit)
            .style(|theme, status| {
                let default = button::primary(theme, status);
                button::Style {
                    border: Border {
                        radius: 6.0.into(),
                        ..default.border
                    },
                    ..default
                }
            }),
    ]
    .spacing(8);
    
    column = column.push(nav_bar);
    column = column.push(hero_section(state));

    for (key, value) in &state.programs {
        let rr = row![
            column![text(key).size(20), text("bar or something")].width(Fill),
            text(value).size(20),
        ];

        column = column.push(rr)
    }

    column.into()
}
