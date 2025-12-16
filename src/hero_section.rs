use iced::{
    Color, Element, Length, Radians, Rectangle, Renderer, Theme,
    alignment::Horizontal,
    mouse::Cursor,
    widget::{
        canvas,
        canvas::{Cache, Geometry, Path, Stroke},
        container, stack, text,
    },
};
use std::f32::consts::PI;

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
            let radius = 90.0;

            let total: f32 = self.data.programs.values().map(|&i| i as f32).sum();
            let mut current_angle = 0.0;

            for (i, value) in self.data.programs.iter().enumerate() {
                let slice_angle = (*value.1 as f32 / total) * 2.0 * PI;
                let end_angle = current_angle + slice_angle;

                let color = self.colors[i % self.colors.len()];

                let arc = Path::new(|p| {
                    p.move_to(center);
                    p.arc(canvas::path::Arc {
                        center,
                        radius: radius,
                        start_angle: Radians(current_angle),
                        end_angle: Radians(end_angle),
                    });
                });

                frame.stroke(&arc, Stroke::default().with_color(color).with_width(20.0));

                current_angle = end_angle;
            }
        });

        vec![geometry]
    }
}

pub fn hero_section(state: &AppState) -> Element<'_, Message> {
    let pie_chart = container(
        canvas(PieChart::new(state))
            .width(Length::Fixed(200.0))
            .height(Length::Fixed(200.0)),
    );

    let time = container(text("12h 44min")).center_x(200).center_y(200);

    let content = stack![pie_chart, time];

    container(content)
        .width(Length::Fill)
        .align_x(Horizontal::Center)
        .into()
}
