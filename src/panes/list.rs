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
    Foo = 0,
    Bar,
}

pub struct ListPane {
    selected: PageType,
    focused: bool,
}

impl ListPane {
    pub fn new(focused: bool) -> ListPane {
        ListPane {
            selected: PageType::Foo,
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
            Msg::ListPaneSelectNext | Msg::ListPaneSelectPrev => match self.selected {
                PageType::Foo => {
                    self.selected = PageType::Bar;
                    return Some(Msg::ToolPaneSelectBarPage);
                }
                PageType::Bar => {
                    self.selected = PageType::Foo;
                    return Some(Msg::ToolPaneSelectFooPage);
                }
            },
            _ => {}
        }
        None
    }

    fn render(&self, buf: &mut Buffer, area: Rect) {
        let items = vec!["foo", "bar"]
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
            (BorderType::Plain, Style::default().fg(Color::Blue))
        } else {
            (BorderType::Plain, Style::default().fg(Color::DarkGray))
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
