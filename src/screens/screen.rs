use crossterm::event::KeyCode;
use ratatui::Frame;

pub trait Screen {
    fn handle_input(&mut self, key: KeyCode)-> Option<ScreenType>;
    fn render(&mut self, f: &mut Frame);
}


#[derive(Clone, Copy)]
pub enum ScreenType {
    Login = 0,
    Home = 1,
}