use ratatui::{buffer::Buffer, layout::Rect};

use crate::msg::Msg;

pub trait Page {
    fn handle_key(&self, key: crossterm::event::KeyEvent) -> Option<Msg>;
    fn update(&mut self, msg: Msg);

    fn render(&self, buf: &mut Buffer, area: Rect);
}
