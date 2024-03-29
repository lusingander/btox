use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::Widget,
};

pub struct Select {
    items: Vec<String>,
    current: usize,
    focused: bool,
    enabled: bool,
}

impl Select {
    pub fn new(items: Vec<String>, current: usize, focused: bool, enabled: bool) -> Select {
        Select {
            items,
            current,
            focused,
            enabled,
        }
    }
}

impl Widget for Select {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // let str_max_w = area.width - 4;
        let cur_item = self.items.get(self.current).unwrap();

        let left_style = self.parts_style(self.current == 0);
        let right_style = self.parts_style(self.current == self.items.len() - 1);
        let cur_item_style = self.parts_style(false);

        buf.set_string(area.left(), area.top(), "<", left_style);
        buf.set_string(area.left() + 2, area.top(), cur_item, cur_item_style);
        buf.set_string(area.right() - 1, area.top(), ">", right_style);
    }
}

impl Select {
    fn parts_style(&self, disabled_cond: bool) -> Style {
        if self.enabled {
            if disabled_cond {
                Style::default().fg(Color::DarkGray)
            } else if self.focused {
                Style::default().fg(Color::Blue)
            } else {
                Style::default().fg(Color::Reset)
            }
        } else {
            Style::default().fg(Color::DarkGray)
        }
    }
}
