use crossterm::event::KeyCode;
use itsuki::zero_indexed_enum;
use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, List, ListItem},
    Frame,
};

use crate::{key_code, key_code_char, msg::Msg, panes::pane::Pane};

#[zero_indexed_enum]
enum PageType {
    Uuid,
    Hash,
    UnixTime,
    NumberBase,
}

impl PageType {
    fn select_msg(&self) -> Msg {
        match self {
            PageType::Uuid => Msg::ToolPaneSelectUuidPage,
            PageType::Hash => Msg::ToolPaneSelectHashPage,
            PageType::UnixTime => Msg::ToolPaneSelectUnixTimePage,
            PageType::NumberBase => Msg::ToolPaneSelectNumberBasePage,
        }
    }

    fn str(&self) -> &str {
        match self {
            PageType::Uuid => "UUID",
            PageType::Hash => "Hash",
            PageType::UnixTime => "Unix time",
            PageType::NumberBase => "Number base",
        }
    }

    fn strings_vec() -> Vec<String> {
        Self::vars_vec().iter().map(|s| s.str().into()).collect()
    }
}

pub struct ListPane {
    selected: PageType,
    focused: bool,
}

impl ListPane {
    pub fn new(focused: bool) -> ListPane {
        ListPane {
            selected: PageType::Uuid,
            focused,
        }
    }
}

impl Pane for ListPane {
    fn handle_key(&self, key: crossterm::event::KeyEvent) -> Option<Msg> {
        match key {
            key_code!(KeyCode::Esc) => Some(Msg::Quit),
            key_code_char!('n', Ctrl) => Some(Msg::ListPaneSelectNext),
            key_code_char!('p', Ctrl) => Some(Msg::ListPaneSelectPrev),
            _ => None,
        }
    }

    fn update(&mut self, msg: Msg) -> Option<Msg> {
        match msg {
            Msg::ListPaneSelectNext => {
                self.selected = self.selected.next();
                return Some(self.selected.select_msg());
            }
            Msg::ListPaneSelectPrev => {
                self.selected = self.selected.prev();
                return Some(self.selected.select_msg());
            }
            _ => {}
        }
        None
    }

    fn render(&mut self, f: &mut Frame, area: Rect) {
        let items = PageType::strings_vec()
            .into_iter()
            .enumerate()
            .map(|(i, label)| {
                let item = ListItem::new(format!(" {} ", label));
                if i == self.selected as usize {
                    let selected_color = if self.focused {
                        Color::Blue
                    } else {
                        Color::DarkGray
                    };
                    item.style(Style::default().fg(Color::Reset).bg(selected_color))
                } else {
                    item.style(Style::default().fg(Color::Reset))
                }
            });

        let (border_type, block_style) = if self.focused {
            (BorderType::Rounded, Style::default().fg(Color::Blue))
        } else {
            (BorderType::Rounded, Style::default().fg(Color::DarkGray))
        };
        let list = List::new(items).block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(border_type)
                .style(block_style),
        );

        f.render_widget(list, area);
    }

    fn focus(&mut self) {
        self.focused = true;
    }

    fn unfocus(&mut self) {
        self.focused = false;
    }
}
