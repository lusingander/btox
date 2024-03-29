use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Paragraph, Widget},
};

use crate::{msg::Msg, pages::page::Page};

pub struct BarPage {
    focused: bool,
}

impl BarPage {
    pub fn new(focused: bool) -> BarPage {
        BarPage { focused }
    }
}

impl Page for BarPage {
    fn handle_key(&self, key: crossterm::event::KeyEvent) -> Option<Msg> {
        let _ = key;
        None
    }

    fn update(&mut self, msg: Msg) -> Option<Msg> {
        let _ = msg;
        None
    }

    fn render(&self, buf: &mut Buffer, area: Rect) {
        let content = Paragraph::new("bar page");
        content.render(area, buf);
    }

    fn focus(&mut self) {
        self.focused = true;
    }

    fn unfocus(&mut self) {
        self.focused = false;
    }
}
