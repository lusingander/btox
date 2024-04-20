use ratatui::{buffer::Buffer, layout::Rect};

use crate::msg::Msg;

pub trait Page {
    fn handle_key(&self, key: crossterm::event::KeyEvent) -> Option<Msg>;
    fn update(&mut self, msg: Msg) -> Option<Msg>;

    fn render(&self, buf: &mut Buffer, area: Rect);

    fn focus(&mut self);
    fn unfocus(&mut self);

    fn helps(&self) -> Vec<&str>;
}
