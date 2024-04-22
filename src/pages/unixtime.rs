use chrono::{DateTime, Utc};
use crossterm::event::KeyCode;
use itsuki::zero_indexed_enum;
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Padding, Paragraph},
    Frame,
};
use tui_input::{backend::crossterm::EventHandler, Input};

use crate::{
    key_code, key_code_char,
    msg::Msg,
    pages::{page::Page, util},
    widget::select::Select,
};

pub struct UnixTimePage {
    focused: bool,
    cur: CurrentStatus,
}

struct CurrentStatus {
    item: PageItems,
    input: Input,
    output: String,
    tz_sel: TimeZoneItemSelect,
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
                tz_sel: TimeZoneItemSelect::Utc,
                status: Status::None,
                edit: false,
            },
        }
    }
}

zero_indexed_enum! {
    PageItems => [Input, Output, TimeZone]
}

zero_indexed_enum! {
    TimeZoneItemSelect => [
        Utc,
        Local,
    ]
}

impl TimeZoneItemSelect {
    fn str(&self) -> &str {
        match self {
            TimeZoneItemSelect::Utc => "UTC",
            TimeZoneItemSelect::Local => "Local",
        }
    }

    fn strings_vec() -> Vec<String> {
        Self::vars_vec().iter().map(|s| s.str().into()).collect()
    }
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
            key_code_char!('l') | key_code!(KeyCode::Right) => {
                Some(Msg::UnixTimePageCurrentItemSelectNext)
            }
            key_code_char!('h') | key_code!(KeyCode::Left) => {
                Some(Msg::UnixTimePageCurrentItemSelectPrev)
            }
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
            Msg::UnixTimePageCurrentItemSelectNext => {
                self.current_item_select_next();
            }
            Msg::UnixTimePageCurrentItemSelectPrev => {
                self.current_item_select_prev();
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

    fn render(&self, f: &mut Frame, area: Rect) {
        let chunks = Layout::vertical([
            Constraint::Length(3),
            Constraint::Length(2),
            Constraint::Length(3),
            Constraint::Length(1),
            Constraint::Length(2),
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
        f.render_widget(input, chunks[0]);

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
            f.render_widget(status, chunks[1])
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
        f.render_widget(output, chunks[2]);

        let tz_sel = Select::new(
            TimeZoneItemSelect::strings_vec(),
            self.cur.tz_sel.val(),
            self.cur.item == PageItems::TimeZone,
            self.focused,
        );
        f.render_widget(tz_sel, chunks[4]);
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
            if matches!(self.cur.item, PageItems::TimeZone) {
                helps.push("<Left/Right> Select current item value");
            }
            if matches!(self.cur.item, PageItems::Input) {
                helps.push("<e> Edit");
            }
            if matches!(self.cur.item, PageItems::Input | PageItems::Output) {
                helps.push("<y> Copy to clipboard");
            }
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

    fn current_item_select_next(&mut self) {
        match self.cur.item {
            PageItems::TimeZone => {
                if self.cur.tz_sel.val() < TimeZoneItemSelect::len() - 1 {
                    self.cur.tz_sel = self.cur.tz_sel.next();
                }
                self.update_output();
            }
            PageItems::Input => {}
            PageItems::Output => {}
        }
    }

    fn current_item_select_prev(&mut self) {
        match self.cur.item {
            PageItems::TimeZone => {
                if self.cur.tz_sel.val() > 0 {
                    self.cur.tz_sel = self.cur.tz_sel.prev();
                }
                self.update_output();
            }
            PageItems::Input => {}
            PageItems::Output => {}
        }
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
        if !matches!(self.cur.item, PageItems::Input | PageItems::Output) {
            return None;
        }

        let text = match self.cur.item {
            PageItems::Input => self.cur.input.value(),
            PageItems::Output => self.cur.output.as_str(),
            _ => "",
        };
        util::copy_to_clipboard(text)
    }

    fn paste_from_clipboard(&mut self) {
        if !matches!(self.cur.item, PageItems::Input) {
            return;
        }

        let text = util::paste_from_clipboard().unwrap();
        self.cur.input = self.cur.input.clone().with_value(text);

        self.update_output();
    }

    fn update_output(&mut self) {
        let s = self.cur.input.value();
        if s.is_empty() {
            self.cur.output = String::new();
            self.cur.status = Status::None;
        } else if let Some(dt) = parse_as_unix_timestamp(s) {
            self.cur.output = match self.cur.tz_sel {
                TimeZoneItemSelect::Utc => dt.datetime.with_timezone(&Utc).to_string(),
                TimeZoneItemSelect::Local => dt.datetime.with_timezone(&chrono::Local).to_string(),
            };
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
