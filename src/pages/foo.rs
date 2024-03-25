use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Paragraph, Widget},
};

use crate::pages::page::Page;

pub struct FooPage {}

impl FooPage {
    pub fn new() -> FooPage {
        FooPage {}
    }
}

impl Page for FooPage {
    fn update(&mut self) {
        todo!()
    }

    fn render(&self, buf: &mut Buffer, area: Rect) {
        let content = Paragraph::new("foo page");
        content.render(area, buf);
    }
}
