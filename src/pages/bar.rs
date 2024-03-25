use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Paragraph, Widget},
};

use crate::pages::page::Page;

pub struct BarPage {}

impl BarPage {
    pub fn new() -> BarPage {
        BarPage {}
    }
}

impl Page for BarPage {
    fn update(&mut self) {
        todo!()
    }

    fn render(&self, buf: &mut Buffer, area: Rect) {
        let content = Paragraph::new("bar page");
        content.render(area, buf);
    }
}
