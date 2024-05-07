use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::Line,
    widgets::{Block, Padding, Paragraph, StatefulWidget, Widget},
};

pub struct ScrollBar {
    lines_len: usize,
    offset: usize,
    bar_char: char,
}

impl ScrollBar {
    pub fn new(lines_len: usize, offset: usize) -> ScrollBar {
        ScrollBar {
            lines_len,
            offset,
            bar_char: '│', // use '┃' or '║' instead...?
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
        let scrollbar_height = self.calc_scrollbar_height(area);
        let scrollbar_top = self.calc_scrollbar_top(area, scrollbar_height);

        let x = area.x;
        for h in 0..scrollbar_height {
            let y = scrollbar_top + h;
            buf.get_mut(x, y).set_char(self.bar_char);
        }
    }

    fn calc_scrollbar_height(&self, area: Rect) -> u16 {
        let area_h = area.height as f64;
        let lines_len = self.lines_len as f64;
        let height = area_h * (area_h / lines_len);
        (height as u16).max(1)
    }

    fn calc_scrollbar_top(&self, area: Rect, scrollbar_height: u16) -> u16 {
        let area_h = area.height as f64;
        let scrollbar_h = scrollbar_height as f64;
        let offset = self.offset as f64;
        let lines_len = self.lines_len as f64;
        let top = ((area_h - scrollbar_h) * offset) / (lines_len - area_h);
        area.y + (top as u16)
    }
}

pub struct ScrollOutput<'a> {
    lines: Vec<Line<'a>>,
    lines_len: usize,
    focused: bool,
    selected: bool,
}

impl<'a> ScrollOutput<'a> {
    pub fn new(lines: Vec<Line>, focused: bool, selected: bool) -> ScrollOutput {
        let lines_len = lines.len();
        ScrollOutput {
            lines,
            lines_len,
            focused,
            selected,
        }
    }
}

pub struct ScrollOutputState {
    pub offset: usize,
}

impl ScrollOutputState {
    pub fn new(offset: usize) -> ScrollOutputState {
        ScrollOutputState { offset }
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

        let max_offset = self.lines_len.saturating_sub(max_content_height);
        if state.offset > max_offset {
            state.offset = max_offset;
        }

        let content: Vec<Line> = self
            .lines
            .iter()
            .skip(state.offset)
            .take(max_content_height)
            .cloned()
            .collect();

        let output = Paragraph::new(content).block(
            Block::bordered()
                .style(style)
                .padding(Padding::horizontal(1)),
        );
        output.render(area, buf);

        if self.lines_len > max_content_height {
            let scrollbar_area = Rect::new(area.right() - 2, area.top() + 1, 1, area.height - 2);
            let scrollbar = ScrollBar::new(self.lines_len, state.offset);
            scrollbar.render(scrollbar_area, buf);
        }
    }
}
