use arboard::Clipboard;
use chrono::{DateTime, Utc};
use crossterm::event::KeyCode;
use itsuki::zero_indexed_enum;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Padding, Paragraph, Widget},
};
use tui_input::{backend::crossterm::EventHandler, Input};

use crate::{key_code, key_code_char, msg::Msg, pages::page::Page};

pub struct UnixTimePage {
    focused: bool,
    cur: CurrentStatus,
}

struct CurrentStatus {
    item: PageItems,
    input: Input,
    output: String,
    status: Status,
    edit: bool,
}

enum Status {
    None,
    Info(String),
    Warn(String),
}

impl Status {
    fn str(&self) -> &str {
        match self {
            Status::None => "",
            Status::Info(s) => s,
            Status::Warn(s) => s,
        }
    }
}

impl UnixTimePage {
    pub fn new(focused: bool) -> UnixTimePage {
        UnixTimePage {
            focused,
            cur: CurrentStatus {
                item: PageItems::Input,
                input: Input::default(),
                output: String::new(),
                status: Status::None,
                edit: false,
            },
        }
    }
}

zero_indexed_enum! {
    PageItems => [Input, Output]
}

impl Page for UnixTimePage {
    fn handle_key(&self, key: crossterm::event::KeyEvent) -> Option<Msg> {
        if self.cur.edit {
            return match key {
                key_code!(KeyCode::Esc) => Some(Msg::UnixTimePageEditEnd),
                _ => Some(Msg::UnixTimePageEditKeyEvent(key)),
            };
        }

        match key {
            key_code!(KeyCode::Esc) => Some(Msg::Quit),
            key_code_char!('n', Ctrl) => Some(Msg::UnixTimePageSelectNextItem),
            key_code_char!('p', Ctrl) => Some(Msg::UnixTimePageSelectPrevItem),
            key_code_char!('y') => Some(Msg::UnixTimePageCopy),
            key_code_char!('p') => Some(Msg::UnixTimePagePaste),
            key_code_char!('e') => Some(Msg::UnixTimePageEditStart),
            _ => None,
        }
    }

    fn update(&mut self, msg: Msg) -> Option<Msg> {
        match msg {
            Msg::UnixTimePageSelectNextItem => {
                self.select_next_item();
            }
            Msg::UnixTimePageSelectPrevItem => {
                self.select_prev_item();
            }
            Msg::UnixTimePageCopy => {
                return self.copy_to_clipboard();
            }
            Msg::UnixTimePagePaste => {
                self.paste_from_clipboard();
            }
            Msg::UnixTimePageEditStart => {
                self.edit_start();
            }
            Msg::UnixTimePageEditEnd => {
                self.edit_end();
            }
            Msg::UnixTimePageEditKeyEvent(key) => {
                self.edit(key);
            }
            _ => {}
        }
        None
    }

    fn render(&self, buf: &mut Buffer, area: Rect) {
        let chunks = Layout::vertical([
            Constraint::Length(3),
            Constraint::Length(2),
            Constraint::Length(3),
        ])
        .split(area);

        let input_style = if self.focused {
            if self.cur.item == PageItems::Input {
                Style::default().fg(Color::Blue)
            } else {
                Style::default().fg(Color::Reset)
            }
        } else {
            Style::default().fg(Color::DarkGray)
        };

        let input = Paragraph::new(self.cur.input.value()).block(
            Block::bordered()
                .style(input_style)
                .title("Input")
                .padding(Padding::horizontal(1)),
        );
        input.render(chunks[0], buf);

        if !matches!(self.cur.status, Status::None) {
            let status_style = match self.cur.status {
                Status::Info(_) => Style::default().fg(Color::Green),
                Status::Warn(_) => Style::default().fg(Color::Yellow),
                _ => Style::default(),
            };
            let status = Paragraph::new(self.cur.status.str().to_string()).block(
                Block::default()
                    .borders(Borders::empty())
                    .style(status_style)
                    .padding(Padding::horizontal(1)),
            );
            status.render(chunks[1], buf);
        }

        let output_style = if self.focused {
            if self.cur.item == PageItems::Output {
                Style::default().fg(Color::Blue)
            } else {
                Style::default().fg(Color::Reset)
            }
        } else {
            Style::default().fg(Color::DarkGray)
        };

        let output = Paragraph::new(self.cur.output.clone()).block(
            Block::bordered()
                .style(output_style)
                .title("Output")
                .padding(Padding::horizontal(1)),
        );
        output.render(chunks[2], buf);
    }

    fn focus(&mut self) {
        self.focused = true;
    }

    fn unfocus(&mut self) {
        self.focused = false;
    }

    fn helps(&self) -> Vec<&str> {
        let mut helps: Vec<&str> = Vec::new();
        if self.cur.edit {
            helps.push("<Esc> End edit");
        } else {
            helps.push("<C-n/C-p> Select item");
            if matches!(self.cur.item, PageItems::Input) {
                helps.push("<e> Edit");
            }
            helps.push("<y> Copy to clipboard");
            if matches!(self.cur.item, PageItems::Input) {
                helps.push("<p> Paste from clipboard");
            }
        }
        helps
    }
}

impl UnixTimePage {
    fn select_next_item(&mut self) {
        self.cur.item = self.cur.item.next();
    }

    fn select_prev_item(&mut self) {
        self.cur.item = self.cur.item.prev();
    }

    fn edit_start(&mut self) {
        self.cur.edit = true;
    }

    fn edit_end(&mut self) {
        self.cur.edit = false;
    }

    fn edit(&mut self, key: crossterm::event::KeyEvent) {
        let event = &crossterm::event::Event::Key(key);
        self.cur.input.handle_event(event);

        self.update_output();
    }

    fn copy_to_clipboard(&self) -> Option<Msg> {
        let text = match self.cur.item {
            PageItems::Input => self.cur.input.value(),
            PageItems::Output => self.cur.output.as_str(),
        };
        let result = Clipboard::new().and_then(|mut c| c.set_text(text));
        match result {
            Ok(_) => Some(Msg::NotifyInfo("Copy to clipboard succeeded".into())),
            Err(_) => Some(Msg::NotifyError("Copy to clipboard failed".into())),
        }
    }

    fn paste_from_clipboard(&mut self) {
        let text = Clipboard::new().and_then(|mut c| c.get_text()).unwrap();
        self.cur.input = self.cur.input.clone().with_value(text);

        self.update_output();
    }

    fn update_output(&mut self) {
        let s = self.cur.input.value();
        if s.is_empty() {
            self.cur.output = String::new();
            self.cur.status = Status::None;
        } else if let Some(dt) = parse_as_unix_timestamp(s) {
            self.cur.output = dt.datetime.to_string();
            let msg = format!("valid unix timestamp ({:?})", dt.resolution);
            self.cur.status = Status::Info(msg);
        } else if let Some(dt) = parse_as_datetime(s) {
            self.cur.output = dt.timestamp().to_string();
            self.cur.status = Status::Info("valid datetime".into());
        } else {
            self.cur.output = String::new();
            self.cur.status = Status::Warn("invalid input".into());
        }
    }
}

struct DateTimeWithResolution {
    datetime: DateTime<Utc>,
    resolution: Resolution,
}

#[derive(Debug)]
enum Resolution {
    Second,
    Milli,
    Micro,
    Nano,
}

fn parse_as_unix_timestamp(s: &str) -> Option<DateTimeWithResolution> {
    s.parse::<u128>().ok().and_then(to_timestamp)
}

fn to_timestamp(t: u128) -> Option<DateTimeWithResolution> {
    if t < 1_000_000_000_000 {
        // seconds
        DateTime::from_timestamp(t as i64, 0).map(|datetime| DateTimeWithResolution {
            datetime,
            resolution: Resolution::Second,
        })
    } else if t < 1_000_000_000_000_000 {
        // millis
        let sec = t / 1_000;
        let millis = t % 1_000;
        DateTime::from_timestamp(sec as i64, millis as u32 * 1_000_000).map(|datetime| {
            DateTimeWithResolution {
                datetime,
                resolution: Resolution::Milli,
            }
        })
    } else if t < 1_000_000_000_000_000_000 {
        // micros
        let sec = t / 1_000_000;
        let micros = t % 1_000_000;
        DateTime::from_timestamp(sec as i64, micros as u32 * 1_000).map(|datetime| {
            DateTimeWithResolution {
                datetime,
                resolution: Resolution::Micro,
            }
        })
    } else if t < 1_000_000_000_000_000_000_000 {
        // nanos
        let sec = t / 1_000_000_000;
        let nanos = t % 1_000_000_000;
        DateTime::from_timestamp(sec as i64, nanos as u32).map(|datetime| DateTimeWithResolution {
            datetime,
            resolution: Resolution::Nano,
        })
    } else {
        // too large
        None
    }
}

fn parse_as_datetime(s: &str) -> Option<DateTime<Utc>> {
    s.parse::<DateTime<Utc>>().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_timestamp() {
        assert_eq!(
            to_timestamp(1_634_567_890).map(|d| d.datetime),
            parse_from_rfc3339("2021-10-18T14:38:10Z")
        );
        assert_eq!(
            to_timestamp(1_634_567_890_123).map(|d| d.datetime),
            parse_from_rfc3339("2021-10-18T14:38:10.123Z")
        );
        assert_eq!(
            to_timestamp(1_634_567_890_123_456).map(|d| d.datetime),
            parse_from_rfc3339("2021-10-18T14:38:10.123456Z")
        );
        assert_eq!(
            to_timestamp(1_634_567_890_123_456_789).map(|d| d.datetime),
            parse_from_rfc3339("2021-10-18T14:38:10.123456789Z")
        );
        assert_eq!(
            to_timestamp(1_634_567).map(|d| d.datetime),
            parse_from_rfc3339("1970-01-19T22:02:47.000Z")
        );
        assert_eq!(
            to_timestamp(1_634_567_890_123_456_789_000).map(|d| d.datetime),
            None
        );
    }

    fn parse_from_rfc3339(s: &str) -> Option<DateTime<Utc>> {
        Some(DateTime::parse_from_rfc3339(s).unwrap().to_utc())
    }
}
