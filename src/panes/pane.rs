use ratatui::{layout::Rect, Frame};

use crate::msg::Msg;

pub trait Pane {
    fn handle_key(&self, key: ratatui::crossterm::event::KeyEvent) -> Option<Msg>;
    fn update(&mut self, msg: Msg) -> Option<Msg>;

    fn render(&mut self, f: &mut Frame, area: Rect);

    fn focus(&mut self);
    fn unfocus(&mut self);

    fn helps(&self) -> Vec<&str>;
}
