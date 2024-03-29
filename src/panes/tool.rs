use ratatui::{
    buffer::Buffer,
    layout::{Margin, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Widget},
};

use crate::{
    msg::Msg,
    pages::{bar::BarPage, foo::FooPage, page::Page, uuid::UuidPage},
    panes::pane::Pane,
};

pub struct ToolPane {
    page: Box<dyn Page>,
    focused: bool,
}

impl ToolPane {
    pub fn new(focused: bool) -> ToolPane {
        ToolPane {
            page: Box::new(UuidPage::new(focused)),
            focused,
        }
    }
}

impl Pane for ToolPane {
    fn handle_key(&self, key: crossterm::event::KeyEvent) -> Option<Msg> {
        self.page.handle_key(key)
    }

    fn update(&mut self, msg: Msg) -> Option<Msg> {
        match msg {
            Msg::ToolPaneSelectUuidPage => {
                self.page = Box::new(UuidPage::new(self.focused));
            }
            Msg::ToolPaneSelectFooPage => {
                self.page = Box::new(FooPage::new(self.focused));
            }
            Msg::ToolPaneSelectBarPage => {
                self.page = Box::new(BarPage::new(self.focused));
            }
            _ => {
                return self.page.update(msg);
            }
        }
        None
    }

    fn render(&self, buf: &mut Buffer, area: Rect) {
        let (border_type, block_style) = if self.focused {
            (BorderType::Rounded, Style::default().fg(Color::Blue))
        } else {
            (BorderType::Rounded, Style::default().fg(Color::DarkGray))
        };
        let page_block = Block::default()
            .borders(Borders::ALL)
            .border_type(border_type)
            .style(block_style);

        page_block.render(area, buf);

        let page_content_area = area.inner(&Margin::new(2, 1));
        self.page.render(buf, page_content_area);
    }

    fn focus(&mut self) {
        self.focused = true;
        self.page.focus();
    }

    fn unfocus(&mut self) {
        self.focused = false;
        self.page.unfocus();
    }
}
