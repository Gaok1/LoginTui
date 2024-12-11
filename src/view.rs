use std::{io::Stdout, time::Duration};

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    prelude::CrosstermBackend,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
};

use crate::screens::{home::Home, login::Login, screen::{Screen, ScreenType}};

pub struct View {
    terminal: ratatui::Terminal<CrosstermBackend<Stdout>>,
    screens: Vec<Box<dyn Screen>>,
    current_screen: ScreenType,
}

impl View {
    pub fn new() -> View {
        let terminal: ratatui::Terminal<CrosstermBackend<Stdout>> = ratatui::init();
        let login = Box::new(Login::new());
        let home = Box::new(Home::new());
        View {
            terminal,
            screens: vec![login,home],
            current_screen: ScreenType::Login,
        }
    }

    pub fn update(&mut self) {
        loop {
            self.terminal
                .draw(|f| {
                    self.screens[self.current_screen as usize].render(f);
                })
                .unwrap();
            if let Some(key) = self.get_input() {
                if let Some(next_screen) = self.screens[self.current_screen as usize].handle_input(key) {
                    self.current_screen = next_screen;
                }
            }
        }
    }

    pub fn get_input(&self) -> Option<KeyCode> {
        if event::poll(Duration::from_millis(0)).unwrap() {
            if let Event::Key(event) = event::read().unwrap() {
                if let KeyEventKind::Press = event.kind {
                    return Some(event.code);
                }
            }
        }
        None
    }
}

impl View {
    fn render_home(&mut self) {
        self.terminal
            .draw(|f| {
                let size = f.area();

                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Percentage(30), Constraint::Percentage(60)])
                    .split(size);

                let centered = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([
                        Constraint::Percentage(30),
                        Constraint::Percentage(40),
                        Constraint::Percentage(30),
                    ])
                    .split(chunks[1]);
                let login_box = Paragraph::new("Home")
                    .block(Block::default().borders(Borders::ALL))
                    .alignment(Alignment::Center);

                f.render_widget(login_box, centered[1]);
            })
            .unwrap();
    }
}
