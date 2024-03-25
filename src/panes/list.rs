use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, List, ListItem, Widget},
};

use crate::{key_code_char, msg::Msg, panes::pane::Pane};

#[derive(Debug, Clone, Copy)]
enum PageType {
    Uuid = 0,
    Foo,
    Bar,
}

impl PageType {
    fn next(&self) -> (PageType, Msg) {
        match self {
            PageType::Uuid => (PageType::Foo, Msg::ToolPaneSelectFooPage),
            PageType::Foo => (PageType::Bar, Msg::ToolPaneSelectBarPage),
            PageType::Bar => (PageType::Uuid, Msg::ToolPaneSelectUuidPage),
        }
    }

    fn prev(&self) -> (PageType, Msg) {
        match self {
            PageType::Uuid => (PageType::Bar, Msg::ToolPaneSelectBarPage),
            PageType::Foo => (PageType::Uuid, Msg::ToolPaneSelectUuidPage),
            PageType::Bar => (PageType::Foo, Msg::ToolPaneSelectFooPage),
        }
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
            key_code_char!('j') => Some(Msg::ListPaneSelectNext),
            key_code_char!('k') => Some(Msg::ListPaneSelectPrev),
            _ => None,
        }
    }

    fn update(&mut self, msg: Msg) -> Option<Msg> {
        match msg {
            Msg::ListPaneSelectNext => {
                let (s, m) = self.selected.next();
                self.selected = s;
                return Some(m);
            }
            Msg::ListPaneSelectPrev => {
                let (s, m) = self.selected.prev();
                self.selected = s;
                return Some(m);
            }
            _ => {}
        }
        None
    }

    fn render(&self, buf: &mut Buffer, area: Rect) {
        let items = vec!["uuid", "foo", "bar"]
            .into_iter()
            .enumerate()
            .map(|(i, label)| {
                let item = ListItem::new(label);
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

        list.render(area, buf);
    }

    fn focus(&mut self) {
        self.focused = true;
    }

    fn unfocus(&mut self) {
        self.focused = false;
    }
}
