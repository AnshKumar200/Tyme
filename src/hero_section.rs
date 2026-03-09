use iced::{
    Alignment, Color, Element, Length, Radians, Rectangle, Renderer, Theme,
    alignment::Horizontal,
    mouse::Cursor,
    widget::{
        canvas::{self, Cache, Geometry, LineCap, Path, Stroke},
        column, container, stack, text,
    },
};
use std::{f32::consts::PI};

use crate::app::{AppState, Message};

struct PieChart<'a> {
    data: &'a AppState,
    colors: Vec<Color>,
    cache: Cache,
}

impl<'a> PieChart<'a> {
    fn new(state: &'a AppState) -> Self {
        Self {
            data: state,
            colors: vec![
                Color::from_rgb8(255, 0, 100),
                Color::from_rgb8(0, 200, 255),
                Color::from_rgb8(255, 200, 0),
                Color::from_rgb8(100, 100, 255),
            ],
            cache: Cache::default(),
        }
    }
}

impl<'a> canvas::Program<Message> for PieChart<'a> {
    type State = ();
    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<Geometry> {
        let geometry = self.cache.draw(renderer, bounds.size(), |frame| {
            let center = frame.center();
            let radius = 70.0;
            let stroke_width = 25.0;
            let desired_offset: f32 = ((stroke_width / 2.0) + 1.0) / radius;

            let total: f32 = self.data.programs.values().map(|&i| i as f32).sum();
            let mut current_angle = 0.0;

            for (i, value) in self.data.programs.iter().enumerate() {
                let slice_percentage = *value.1 as f32 / total;
                let slice_angle = slice_percentage * 2.0 * PI;

                let min_angle = desired_offset * 2.0 + 0.001;
                let effective_angle = slice_angle.max(min_angle);

                let start = current_angle + desired_offset;
                let end = current_angle + effective_angle - desired_offset;

                let color = self.colors[i % self.colors.len()];

                let arc = Path::new(|p| {
                    p.arc(canvas::path::Arc {
                        center,
                        radius: radius,
                        start_angle: Radians(start),
                        end_angle: Radians(end),
                    });
                });

                frame.stroke(
                    &arc,
                    Stroke::default()
                        .with_color(color)
                        .with_width(stroke_width)
                        .with_line_cap(LineCap::Round),
                );
                current_angle += effective_angle;
            }
        });

        vec![geometry]
    }
}

pub fn hero_section(state: &AppState) -> Element<'_, Message> {
    let pie_chart = container(
        canvas::Canvas::new(PieChart::new(state))
            .width(Length::Fixed(200.0))
            .height(Length::Fixed(200.0)),
    );

    let time = column![text("12h"), text("44min")]
        .spacing(10)
        .align_x(Alignment::Center);

    let content = stack![pie_chart, container(time).center_y(200.0).center_x(200.0)];

    container(content)
        .width(Length::Fill)
        .align_x(Horizontal::Center)
        .into()
}
