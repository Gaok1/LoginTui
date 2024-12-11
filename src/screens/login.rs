// render Login

use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    widgets::{Block, Borders, Paragraph},
};

use super::screen::{Screen, ScreenType};

pub struct Login {
    pub Email_content: String,
    pub Password_content: String,
    pub field: Field,
}

impl Login {
    pub fn new() -> Self {
        Self {
            Email_content: String::new(),
            Password_content: String::new(),
            field: Field::Email,
        }
    }
}

impl Screen for Login {
    fn render(&mut self, f: &mut ratatui::Frame) {
        let size = f.area();
        let block = Block::bordered().bg(Color::Rgb(32, 32, 32));
        f.render_widget(block, size);

        let block = Block::default()
            .title_bottom(" Use ↑/↓ to move, enter to submit, q to quit ")
            .borders(Borders::BOTTOM)
            .title_alignment(Alignment::Center);
        f.render_widget(block, size);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(60)])
            .split(size);

        let centered = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(25),
                Constraint::Percentage(50),
                Constraint::Percentage(25),
            ])
            .split(chunks[1]);
        let login_box = Paragraph::new("Login")
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center);

        f.render_widget(login_box, centered[1]);

        let login_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(3),    //title
                Constraint::Length(3), //email
                Constraint::Min(2),
                Constraint::Length(3), //password
                Constraint::Min(3),
            ])
            .split(centered[1]);
        let input_box_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Min(2),
                Constraint::Percentage(70),
                Constraint::Min(2),
            ]);
        //email
        let mut input_box = Paragraph::new(self.Email_content.as_str())
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("email")
                    .title_alignment(Alignment::Center),
            )
            .alignment(Alignment::Center);
        if let Field::Email = self.field {
            input_box = input_box.style(Style::default().fg(Color::Yellow));
        }
        let input_chunks = input_box_layout.split(login_chunks[1]);
        f.render_widget(input_box, input_chunks[1]);
        let mut input_box = Paragraph::new(self.Password_content.as_str())
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("password")
                    .title_alignment(Alignment::Center),
            )
            .alignment(Alignment::Center);
        if let Field::Password = self.field {
            input_box = input_box.style(Style::default().fg(Color::Yellow));
        }
        let input_chunks = input_box_layout.split(login_chunks[3]);

        f.render_widget(input_box, input_chunks[1]);
    }

    fn handle_input(&mut self, key: KeyCode) -> Option<ScreenType> {
        match key {
            KeyCode::Char('q') => {
                std::process::exit(0);
            }
            KeyCode::Char(c) => match self.field {
                Field::Email => {
                    self.Email_content.push(c);
                }
                Field::Password => {
                    self.Password_content.push(c);
                }
            },
            KeyCode::Enter => {
                let mut login_valid = true;

                // Validação do Email
                if self.Email_content.trim().is_empty() {
                    self.Email_content = String::from("Email is required");
                    login_valid = false;
                } else if self.Email_content.contains(' ') || !self.Email_content.contains('@') {
                    self.Email_content = String::from("Invalid email format");
                    login_valid = false;
                }

                // Validação da Senha
                if self.Password_content.trim().is_empty() {
                    self.Password_content = String::from("Password is required");
                    login_valid = false;
                }

                // Verifica se as validações foram passadas
                if login_valid {
                    return Some(ScreenType::Home); // Login válido, vai para a tela principal
                } else {
                    return Some(ScreenType::Login); // Login inválido, mantém na tela de login
                }
            }

            KeyCode::Backspace => match self.field {
                Field::Email => {
                    self.Email_content.pop();
                }
                Field::Password => {
                    self.Password_content.pop();
                }
            },
            KeyCode::Down => {
                self.field = Field::Password;
            }
            KeyCode::Up => {
                self.field = Field::Email;
            }
            _ => {}
        }
        None
    }
}

enum Field {
    Email,
    Password,
}
