use std::process::exit;

use ratatui::{
    style::{Color, Style},
    widgets::{
        canvas::{Canvas, Map, MapResolution, Points, Shape},
        Block, BorderType, Borders,
    },
};

use super::screen::Screen;

pub struct Home {}

impl Home {
    pub fn new() -> Self {
        Self {}
    }
}

impl Screen for Home {
    fn handle_input(
        &mut self,
        key: crossterm::event::KeyCode,
    ) -> Option<super::screen::ScreenType> {
        if (key == crossterm::event::KeyCode::Char('q')) {
            exit(0);
        }
        None
    }

    fn render(&mut self, f: &mut ratatui::Frame) {
        let size = f.area();

        let block = Block::default().title("Home - Map").borders(Borders::ALL);
        let dots = vec![
            (0.0, 0.0),
            (10.0, 10.0),
            (20.0, 20.0),
            (30.0, 30.0),
            (40.0, 40.0),
            (50.0, 50.0),
            (60.0, 60.0),
            (70.0, 70.0),
            (80.0, 80.0),
            (90.0, 90.0),
        ];
        let mut canvas = Canvas::default()
            .block(block)
            .paint(|ctx| {
                ctx.draw(&Map {
                    resolution: MapResolution::High, // Resolução do mapa
                    color: Color::LightGreen,        // Cor do mapa
                });
              

                let points = Points {
                    coords: &dots,
                    color: Color::LightRed,
                };

                ctx.draw(&points);
            })
            .x_bounds([-180.0, 180.0])
            .y_bounds([-90.0, 90.0]);

        f.render_widget(canvas, size);
    }
}
