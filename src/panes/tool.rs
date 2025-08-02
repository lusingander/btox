use ratatui::{
    layout::{Margin, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders},
    Frame,
};

use crate::{
    msg::Msg,
    pages::{
        hash::HashPage, number::NumberBasePage, page::Page, ulid::UlidPage, unixtime::UnixTimePage,
        uuid::UuidPage,
    },
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
    fn handle_key(&self, key: ratatui::crossterm::event::KeyEvent) -> Option<Msg> {
        self.page.handle_key(key)
    }

    fn update(&mut self, msg: Msg) -> Option<Msg> {
        match msg {
            Msg::ToolPaneSelectUuidPage => {
                self.page = Box::new(UuidPage::new(self.focused));
            }
            Msg::ToolPaneSelectUlidPage => {
                self.page = Box::new(UlidPage::new(self.focused));
            }
            Msg::ToolPaneSelectHashPage => {
                self.page = Box::new(HashPage::new(self.focused));
            }
            Msg::ToolPaneSelectUnixTimePage => {
                self.page = Box::new(UnixTimePage::new(self.focused));
            }
            Msg::ToolPaneSelectNumberBasePage => {
                self.page = Box::new(NumberBasePage::new(self.focused));
            }
            Msg::Page(page_msg) => {
                return self.page.update(&page_msg);
            }
            _ => {}
        }
        None
    }

    fn render(&mut self, f: &mut Frame, area: Rect) {
        let (border_type, block_style) = if self.focused {
            (BorderType::Rounded, Style::default().fg(Color::Blue))
        } else {
            (BorderType::Rounded, Style::default().fg(Color::DarkGray))
        };
        let page_block = Block::default()
            .borders(Borders::ALL)
            .border_type(border_type)
            .style(block_style);

        f.render_widget(page_block, area);

        let page_content_area = area.inner(Margin::new(2, 1));
        self.page.render(f, page_content_area);
    }

    fn focus(&mut self) {
        self.focused = true;
        self.page.focus();
    }

    fn unfocus(&mut self) {
        self.focused = false;
        self.page.unfocus();
    }

    fn helps(&self) -> Vec<&str> {
        self.page.helps()
    }
}
