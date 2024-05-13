use std::ops::Range;

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Text},
    widgets::{Block, Padding, Paragraph, StatefulWidget, Widget},
};

struct BarCharSet {
    full: char,
    upper_half: char,
    lower_half: char,
}

impl BarCharSet {
    fn normal() -> BarCharSet {
        BarCharSet {
            full: '│',
            upper_half: '╵',
            lower_half: '╷',
        }
    }

    #[allow(unused)]
    fn heavy() -> BarCharSet {
        BarCharSet {
            full: '┃',
            upper_half: '╹',
            lower_half: '╻',
        }
    }
}

pub struct ScrollBar {
    lines_len: usize,
    offset: usize,
    bar_char_set: BarCharSet,
}

impl ScrollBar {
    pub fn new(lines_len: usize, offset: usize) -> ScrollBar {
        ScrollBar {
            lines_len,
            offset,
            bar_char_set: BarCharSet::normal(),
        }
    }
}

impl Default for ScrollBar {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

impl Widget for ScrollBar {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.render_scroll_bar(area, buf);
    }
}

impl ScrollBar {
    fn render_scroll_bar(&self, area: Rect, buf: &mut Buffer) {
        let scrollbar_height = self.calc_virtual_scrollbar_height(area);
        let scrollbar_range = self.calc_virtual_scrollbar_range(area, scrollbar_height);

        let r = scrollbar_range;
        let x = area.x;
        for i in 0..area.height {
            let y = area.y + i;
            let upper_half = r.contains(&(area.y + i * 2));
            let lower_half = r.contains(&(area.y + i * 2 + 1));
            if upper_half && lower_half {
                buf.get_mut(x, y).set_char(self.bar_char_set.full);
            } else if upper_half {
                buf.get_mut(x, y).set_char(self.bar_char_set.upper_half);
            } else if lower_half {
                buf.get_mut(x, y).set_char(self.bar_char_set.lower_half);
            }
        }
    }

    fn calc_virtual_scrollbar_height(&self, area: Rect) -> u16 {
        let area_h = (area.height as f64) * 2.0;
        let lines_len = (self.lines_len as f64) * 2.0;
        let height = area_h * (area_h / lines_len);
        (height as u16).max(1)
    }

    fn calc_virtual_scrollbar_range(&self, area: Rect, scrollbar_height: u16) -> Range<u16> {
        let area_h = (area.height as f64) * 2.0;
        let scrollbar_h = scrollbar_height as f64;
        let offset = (self.offset as f64) * 2.0;
        let lines_len = (self.lines_len as f64) * 2.0;
        let top_offset = ((area_h - scrollbar_h) * offset) / (lines_len - area_h);

        let top = top_offset as u16 + area.y;
        top..(top + scrollbar_height)
    }
}

pub struct ScrollOutput<'a> {
    text: Text<'a>,
    focused: bool,
    selected: bool,
    title: &'a str,
}

impl<'a> ScrollOutput<'a> {
    pub fn new<T>(text: T, focused: bool, selected: bool) -> ScrollOutput<'a>
    where
        T: Into<Text<'a>>,
    {
        ScrollOutput {
            text: text.into(),
            focused,
            selected,
            title: "",
        }
    }

    pub fn title(mut self, title: &'a str) -> Self {
        self.title = title;
        self
    }
}

pub struct ScrollOutputState {
    pub offset: usize,
}

impl ScrollOutputState {
    pub fn new(offset: usize) -> ScrollOutputState {
        ScrollOutputState { offset }
    }

    pub fn scroll_down(&mut self) {
        // no need to check max offset because it's already handled in render
        self.offset = self.offset.saturating_add(1);
    }

    pub fn scroll_up(&mut self) {
        self.offset = self.offset.saturating_sub(1);
    }
}

impl Default for ScrollOutputState {
    fn default() -> Self {
        ScrollOutputState::new(0)
    }
}

impl<'a> StatefulWidget for ScrollOutput<'a> {
    type State = ScrollOutputState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let style = if self.focused {
            if self.selected {
                Style::default().fg(Color::Blue)
            } else {
                Style::default().fg(Color::Reset)
            }
        } else {
            Style::default().fg(Color::DarkGray)
        };

        let max_content_height = area.height as usize - 2;

        let max_offset = self.text.height().saturating_sub(max_content_height);
        if state.offset > max_offset {
            state.offset = max_offset;
        }

        let content: Vec<Line> = self
            .text
            .iter()
            .skip(state.offset)
            .take(max_content_height)
            .cloned()
            .collect();

        let output = Paragraph::new(content).block(
            Block::bordered()
                .style(style)
                .title(self.title)
                .padding(Padding::horizontal(1)),
        );
        output.render(area, buf);

        if self.text.height() > max_content_height {
            let scrollbar_area = Rect::new(area.right() - 2, area.top() + 1, 1, area.height - 2);
            let scrollbar = ScrollBar::new(self.text.height(), state.offset);
            scrollbar.render(scrollbar_area, buf);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::assert_buffer_eq;
    use rstest::*;

    #[rstest]
    #[case(10, 20, 0, vec!["│", "│", "│", "│", "│", " ", " ", " ", " ", " "])]
    #[case(10, 20, 1, vec!["╷", "│", "│", "│", "│", "╵", " ", " ", " ", " "])]
    #[case(10, 20, 2, vec![" ", "│", "│", "│", "│", "│", " ", " ", " ", " "])]
    #[case(10, 20, 3, vec![" ", "╷", "│", "│", "│", "│", "╵", " ", " ", " "])]
    #[case(10, 20, 9, vec![" ", " ", " ", " ", "╷", "│", "│", "│", "│", "╵"])]
    #[case(10, 20, 10, vec![" ", " ", " ", " ", " ", "│", "│", "│", "│", "│"])]
    fn test_scroll_bar(
        #[case] area_height: u16,
        #[case] lines_len: usize,
        #[case] offset: usize,
        #[case] expected: Vec<&'static str>,
    ) {
        let area = Rect::new(0, 0, 1, area_height);
        let mut buf = Buffer::empty(area);

        let scroll_bar = ScrollBar::new(lines_len, offset);
        scroll_bar.render(area, &mut buf);

        let expected = Buffer::with_lines(expected);
        assert_buffer_eq!(buf, expected);
    }
}
