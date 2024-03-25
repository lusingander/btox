use ratatui::{buffer::Buffer, layout::Rect};

pub trait Page {
    fn update(&mut self);
    fn render(&self, buf: &mut Buffer, area: Rect);
}
