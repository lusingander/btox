use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Paragraph, Widget},
};

use crate::{msg::Msg, pages::page::Page};

pub struct FooPage {}

impl FooPage {
    pub fn new() -> FooPage {
        FooPage {}
    }
}

impl Page for FooPage {
    fn handle_key(&self, key: crossterm::event::KeyEvent) -> Option<Msg> {
        let _ = key;
        None
    }

    fn update(&mut self, msg: Msg) {
        let _ = msg;
    }

    fn render(&self, buf: &mut Buffer, area: Rect) {
        let content = Paragraph::new("foo page");
        content.render(area, buf);
    }
}
