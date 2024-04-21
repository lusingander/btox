use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Margin, Rect},
    style::{Color, Style},
    text::Line,
    widgets::{Block, BorderType, Borders, Paragraph, Widget},
};

use crate::{
    msg::Msg,
    pages::{bar::BarPage, foo::FooPage, page::Page, uuid::UuidPage},
    panes::pane::Pane,
    util::group_strs_to_fit_width,
};

pub struct ToolPane {
    page: Box<dyn Page>,
    focused: bool,
    help: bool,
}

impl ToolPane {
    pub fn new(focused: bool) -> ToolPane {
        ToolPane {
            page: Box::new(UuidPage::new(focused)),
            focused,
            help: false,
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
            Msg::ToggleHelp => {
                self.help = !self.help;
            }
            _ => {
                return self.page.update(msg);
            }
        }
        None
    }

    fn render(&self, buf: &mut Buffer, area: Rect) {
        let help_lines = self.help_lines(area.width - 2);

        let chunks = Layout::vertical([
            Constraint::Min(0),
            Constraint::Length(help_lines.len() as u16),
        ])
        .split(area);

        let (border_type, block_style) = if self.focused {
            (BorderType::Rounded, Style::default().fg(Color::Blue))
        } else {
            (BorderType::Rounded, Style::default().fg(Color::DarkGray))
        };
        let page_block = Block::default()
            .borders(Borders::ALL)
            .border_type(border_type)
            .style(block_style);

        page_block.render(chunks[0], buf);

        let page_content_area = chunks[0].inner(&Margin::new(2, 1));
        self.page.render(buf, page_content_area);

        if self.help {
            let help_area = chunks[1].inner(&Margin::new(1, 0));
            let help = Paragraph::new(help_lines);
            help.render(help_area, buf);
        }
    }

    fn focus(&mut self) {
        self.focused = true;
        self.page.focus();
    }

    fn unfocus(&mut self) {
        self.focused = false;
        self.page.unfocus();
        self.help = false;
    }
}

impl ToolPane {
    fn help_lines(&self, width: u16) -> Vec<Line> {
        if self.help && self.focused {
            let delimiter = ", ";
            group_strs_to_fit_width(&self.page.helps(), width as usize, delimiter)
                .iter()
                .map(|helps| {
                    Line::styled(helps.join(delimiter), Style::default().fg(Color::DarkGray))
                })
                .chain([Line::raw("")])
                .collect()
        } else {
            Vec::new()
        }
    }
}
