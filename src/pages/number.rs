use crossterm::event::KeyCode;
use itsuki::zero_indexed_enum;
use ratatui::{layout::Rect, widgets::Paragraph, Frame};

use crate::{key_code, msg::Msg, pages::page::Page};

pub struct NumberBasePage {
    focused: bool,
    cur: CurrentStatus,
}

struct CurrentStatus {
    item: PageItems,
}

impl NumberBasePage {
    pub fn new(focused: bool) -> NumberBasePage {
        NumberBasePage {
            focused,
            cur: CurrentStatus {
                item: PageItems::Binary,
            },
        }
    }
}

zero_indexed_enum! {
    PageItems => [Binary, Octal, Decimal, Hexadecimal]
}

impl Page for NumberBasePage {
    fn handle_key(&self, key: crossterm::event::KeyEvent) -> Option<Msg> {
        match key {
            key_code!(KeyCode::Esc) => Some(Msg::Quit),
            _ => None,
        }
    }

    fn update(&mut self, msg: Msg) -> Option<Msg> {
        match msg {
            _ => {}
        }
        None
    }

    fn render(&self, f: &mut Frame, area: Rect) {
        let content = Paragraph::new("NumberBase page");
        f.render_widget(content, area);
    }

    fn focus(&mut self) {
        self.focused = true;
    }

    fn unfocus(&mut self) {
        self.focused = false;
    }

    fn helps(&self) -> Vec<&str> {
        let mut helps: Vec<&str> = Vec::new();
        helps
    }
}
