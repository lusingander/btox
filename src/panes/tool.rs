use ratatui::{
    layout::{Margin, Rect},
    style::{Color, Style},
    text::Line,
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};
use ratatui_macros::vertical;

use crate::{
    msg::Msg,
    pages::{
        hash::HashPage, number::NumberBasePage, page::Page, unixtime::UnixTimePage, uuid::UuidPage,
    },
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
            Msg::ToolPaneSelectHashPage => {
                self.page = Box::new(HashPage::new(self.focused));
            }
            Msg::ToolPaneSelectUnixTimePage => {
                self.page = Box::new(UnixTimePage::new(self.focused));
            }
            Msg::ToolPaneSelectNumberBasePage => {
                self.page = Box::new(NumberBasePage::new(self.focused));
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

    fn render(&self, f: &mut Frame, area: Rect) {
        let help_lines = self.help_lines(area.width - 2);

        let help_lines_len = help_lines.len() as u16;
        let chunks = vertical![>=0, ==help_lines_len].split(area);

        let (border_type, block_style) = if self.focused {
            (BorderType::Rounded, Style::default().fg(Color::Blue))
        } else {
            (BorderType::Rounded, Style::default().fg(Color::DarkGray))
        };
        let page_block = Block::default()
            .borders(Borders::ALL)
            .border_type(border_type)
            .style(block_style);

        f.render_widget(page_block, chunks[0]);

        let page_content_area = chunks[0].inner(&Margin::new(2, 1));
        self.page.render(f, page_content_area);

        if self.help {
            let help_area = chunks[1].inner(&Margin::new(1, 0));
            let help = Paragraph::new(help_lines);
            f.render_widget(help, help_area);
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
