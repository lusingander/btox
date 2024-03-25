use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Paragraph, Widget},
};

use crate::{msg::Msg, pages::page::Page};

pub struct UuidPage {}

impl UuidPage {
    pub fn new() -> UuidPage {
        UuidPage {}
    }
}

impl Page for UuidPage {
    fn handle_key(&self, key: crossterm::event::KeyEvent) -> Option<Msg> {
        let _ = key;
        None
    }

    fn update(&mut self, msg: Msg) {
        let _ = msg;
    }

    fn render(&self, buf: &mut Buffer, area: Rect) {
        let content = Paragraph::new("uuid page");
        content.render(area, buf);
    }
}
