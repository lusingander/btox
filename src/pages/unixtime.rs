use itsuki::zero_indexed_enum;
use jiff::{fmt::strtime, tz::TimeZone, Timestamp};
use ratatui::{
    crossterm::event::KeyCode,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Padding, Paragraph},
    Frame,
};
use ratatui_macros::vertical;
use tui_input::{backend::crossterm::EventHandler, Input};

use crate::{
    fn_next_prev_mut, fn_str_map, key_code, key_code_char,
    msg::{Msg, PageMsg, UnixTimeMsg},
    pages::{page::Page, util},
    widget::select::Select,
};

const DEFAULT_OUTPUT_FORMAT: &str = "%Y-%m-%dT%H:%M:%S%.f%:z";

pub struct UnixTimePage {
    focused: bool,
    cur: CurrentStatus,
}

#[derive(Default)]
struct CurrentStatus {
    item: PageItems,
    input: Input,
    output: String,
    tz_sel: TimeZoneItemSelect,
    output_format: Input,
    input_status: Status,
    output_format_status: Status,
    edit: bool,
}

#[derive(Default)]
enum Status {
    #[default]
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
        let output_format = Input::new(DEFAULT_OUTPUT_FORMAT.to_string());
        UnixTimePage {
            focused,
            cur: CurrentStatus {
                output_format,
                ..Default::default()
            },
        }
    }
}

#[derive(Default)]
#[zero_indexed_enum]
enum PageItems {
    #[default]
    Input,
    Output,
    TimeZone,
    OutputFormat,
}

#[derive(Default)]
#[zero_indexed_enum]
enum TimeZoneItemSelect {
    #[default]
    Utc,
    Local,
}

impl TimeZoneItemSelect {
    fn_str_map! {
        TimeZoneItemSelect::Utc => "UTC",
        TimeZoneItemSelect::Local => "Local",
    }

    fn_next_prev_mut! {}
}

impl Page for UnixTimePage {
    fn handle_key(&self, key: ratatui::crossterm::event::KeyEvent) -> Option<Msg> {
        let msg = if self.cur.edit {
            match key {
                key_code!(KeyCode::Esc) => UnixTimeMsg::EditEnd,
                _ => UnixTimeMsg::EditKeyEvent(key),
            }
        } else {
            match key {
                key_code_char!('j') | key_code!(KeyCode::Down) => UnixTimeMsg::SelectNextItem,
                key_code_char!('k') | key_code!(KeyCode::Up) => UnixTimeMsg::SelectPrevItem,
                key_code_char!('l') | key_code!(KeyCode::Right) => {
                    UnixTimeMsg::CurrentItemSelectNext
                }
                key_code_char!('h') | key_code!(KeyCode::Left) => {
                    UnixTimeMsg::CurrentItemSelectPrev
                }
                key_code_char!('y') => UnixTimeMsg::Copy,
                key_code_char!('p') => UnixTimeMsg::Paste,
                key_code_char!('e') => UnixTimeMsg::EditStart,
                _ => return None,
            }
        };
        Some(Msg::Page(PageMsg::UnixTime(msg)))
    }

    fn update(&mut self, msg: PageMsg) -> Option<Msg> {
        if let PageMsg::UnixTime(msg) = msg {
            match msg {
                UnixTimeMsg::SelectNextItem => {
                    self.select_next_item();
                }
                UnixTimeMsg::SelectPrevItem => {
                    self.select_prev_item();
                }
                UnixTimeMsg::CurrentItemSelectNext => {
                    self.current_item_select_next();
                }
                UnixTimeMsg::CurrentItemSelectPrev => {
                    self.current_item_select_prev();
                }
                UnixTimeMsg::Copy => {
                    return self.copy_to_clipboard();
                }
                UnixTimeMsg::Paste => {
                    self.paste_from_clipboard();
                }
                UnixTimeMsg::EditStart => {
                    self.edit_start();
                }
                UnixTimeMsg::EditEnd => {
                    self.edit_end();
                }
                UnixTimeMsg::EditKeyEvent(key) => {
                    self.edit(key);
                }
            }
        }
        None
    }

    fn render(&mut self, f: &mut Frame, area: Rect) {
        let chunks = vertical![==3, ==2, ==3, ==1, ==2, ==3, ==1].split(area);

        self.render_input(f, chunks[0], &self.cur.input, PageItems::Input, "Input");

        if !matches!(self.cur.input_status, Status::None) {
            self.render_status(f, chunks[1], &self.cur.input_status);
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

        self.render_input(
            f,
            chunks[5],
            &self.cur.output_format,
            PageItems::OutputFormat,
            "Output Format",
        );

        if !matches!(self.cur.output_format_status, Status::None) {
            self.render_status(f, chunks[6], &self.cur.output_format_status);
        }
    }

    fn focus(&mut self) {
        self.focused = true;
    }

    fn unfocus(&mut self) {
        self.focused = false;
    }

    fn helps(&self) -> Vec<&str> {
        use PageItems::*;
        let mut helps: Vec<&str> = Vec::new();
        if self.cur.edit {
            helps.push("<Esc> End edit");
        } else {
            helps.push("<j/k> Select item");
            if matches!(self.cur.item, TimeZone) {
                helps.push("<h/l> Select current item value");
            }
            if matches!(self.cur.item, Input | OutputFormat) {
                helps.push("<e> Edit");
            }
            if matches!(self.cur.item, Input | Output | OutputFormat) {
                helps.push("<y> Copy to clipboard");
            }
            if matches!(self.cur.item, Input | OutputFormat) {
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
                self.cur.tz_sel.next_mut();
                self.update_output();
            }
            PageItems::Input => {}
            PageItems::Output => {}
            PageItems::OutputFormat => {}
        }
    }

    fn current_item_select_prev(&mut self) {
        match self.cur.item {
            PageItems::TimeZone => {
                self.cur.tz_sel.prev_mut();
                self.update_output();
            }
            PageItems::Input => {}
            PageItems::Output => {}
            PageItems::OutputFormat => {}
        }
    }

    fn edit_start(&mut self) {
        use PageItems::*;
        if !matches!(self.cur.item, Input | OutputFormat) {
            return;
        }
        self.cur.edit = true;
    }

    fn edit_end(&mut self) {
        use PageItems::*;
        if !matches!(self.cur.item, Input | OutputFormat) {
            return;
        }
        self.cur.edit = false;
    }

    fn edit(&mut self, key: ratatui::crossterm::event::KeyEvent) {
        let event = &ratatui::crossterm::event::Event::Key(key);

        match self.cur.item {
            PageItems::Input => {
                self.cur.input.handle_event(event);
            }
            PageItems::OutputFormat => {
                self.cur.output_format.handle_event(event);
            }
            _ => {}
        };

        self.update_output();
    }

    fn copy_to_clipboard(&self) -> Option<Msg> {
        use PageItems::*;
        if !matches!(self.cur.item, Input | Output | OutputFormat) {
            return None;
        }

        let text = match self.cur.item {
            Input => self.cur.input.value(),
            Output => self.cur.output.as_str(),
            OutputFormat => self.cur.output_format.value(),
            _ => "",
        };
        util::copy_to_clipboard(text)
    }

    fn paste_from_clipboard(&mut self) {
        use PageItems::*;
        if !matches!(self.cur.item, Input | OutputFormat) {
            return;
        }

        let text = util::paste_from_clipboard().unwrap();
        match self.cur.item {
            Input => self.cur.input = self.cur.input.clone().with_value(text),
            OutputFormat => {
                self.cur.output_format = self.cur.output_format.clone().with_value(text)
            }
            _ => {}
        }

        self.update_output();
    }

    fn update_output(&mut self) {
        self.cur.output.clear();
        self.cur.input_status = Status::None;
        self.cur.output_format_status = Status::None;

        let s = self.cur.input.value();
        if s.is_empty() {
            return;
        }

        if let Some(dt) = parse_as_unix_timestamp(s) {
            let msg = format!("valid unix timestamp ({:?})", dt.resolution);
            self.cur.input_status = Status::Info(msg);
            let time_zone = match self.cur.tz_sel {
                TimeZoneItemSelect::Utc => TimeZone::UTC,
                TimeZoneItemSelect::Local => match TimeZone::try_system() {
                    Ok(time_zone) => time_zone,
                    Err(_) => {
                        self.cur.input_status = Status::Warn("local time zone unavailable".into());
                        return;
                    }
                },
            };
            match format_datetime(dt.timestamp, self.cur.output_format.value(), time_zone) {
                Ok(output) => self.cur.output = output,
                Err(_) => self.cur.output_format_status = Status::Warn("invalid format".into()),
            }
        } else if let Some(dt) = parse_as_datetime(s) {
            self.cur.output = to_unix_seconds(dt).to_string();
            self.cur.input_status = Status::Info("valid datetime".into());
        } else {
            self.cur.input_status = Status::Warn("invalid input".into());
        }
    }

    fn render_input(&self, f: &mut Frame, area: Rect, input: &Input, item: PageItems, title: &str) {
        let input_style = if self.focused {
            if self.cur.item == item {
                Style::default().fg(Color::Blue)
            } else {
                Style::default().fg(Color::Reset)
            }
        } else {
            Style::default().fg(Color::DarkGray)
        };

        let input_max_width = area.width - 4;
        let input_value = input.value();
        let input_start = input_value.len().saturating_sub(input_max_width as usize);
        let input_content = &input_value[input_start..];
        let input_widget = Paragraph::new(input_content).block(
            Block::bordered()
                .style(input_style)
                .title(title)
                .padding(Padding::horizontal(1)),
        );
        f.render_widget(input_widget, area);

        if self.cur.edit && self.cur.item == item {
            let visual_cursor = input.visual_cursor() as u16;
            let x = area.x + 2 + visual_cursor.min(input_max_width);
            let y = area.y + 1;
            f.set_cursor_position((x, y));
        }
    }

    fn render_status(&self, f: &mut Frame, area: Rect, status: &Status) {
        let status_style = match status {
            Status::Info(_) => Style::default().fg(Color::Green),
            Status::Warn(_) => Style::default().fg(Color::Yellow),
            _ => Style::default(),
        };
        let status = Paragraph::new(status.str().to_string()).block(
            Block::default()
                .borders(Borders::empty())
                .style(status_style)
                .padding(Padding::horizontal(1)),
        );
        f.render_widget(status, area)
    }
}

struct TimestampWithResolution {
    timestamp: Timestamp,
    resolution: Resolution,
}

#[derive(Debug, PartialEq, Eq)]
enum Resolution {
    Second,
    Milli,
    Micro,
    Nano,
}

fn parse_as_unix_timestamp(s: &str) -> Option<TimestampWithResolution> {
    s.parse::<u128>().ok().and_then(to_timestamp)
}

fn to_timestamp(t: u128) -> Option<TimestampWithResolution> {
    let (units_per_second, resolution) = if t < 1_000_000_000_000 {
        (1, Resolution::Second)
    } else if t < 1_000_000_000_000_000 {
        (1_000, Resolution::Milli)
    } else if t < 1_000_000_000_000_000_000 {
        (1_000_000, Resolution::Micro)
    } else if t < 1_000_000_000_000_000_000_000 {
        (1_000_000_000, Resolution::Nano)
    } else {
        return None;
    };

    let second = (t / units_per_second) as i64;
    let nanosecond = ((t % units_per_second) * (1_000_000_000 / units_per_second)) as i32;
    Timestamp::new(second, nanosecond)
        .ok()
        .map(|timestamp| TimestampWithResolution {
            timestamp,
            resolution,
        })
}

fn parse_as_datetime(s: &str) -> Option<Timestamp> {
    s.trim().parse().ok()
}

fn to_unix_seconds(timestamp: Timestamp) -> i128 {
    // Preserve floor rounding for instants before the Unix epoch.
    timestamp.as_nanosecond().div_euclid(1_000_000_000)
}

fn format_datetime(
    timestamp: Timestamp,
    format: &str,
    time_zone: TimeZone,
) -> Result<String, jiff::Error> {
    strtime::format(format, &timestamp.to_zoned_datetime(time_zone))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(0, "1970-01-01T00:00:00Z", Resolution::Second)]
    #[case(1_634_567_890, "2021-10-18T14:38:10Z", Resolution::Second)]
    #[case(1_634_567_890_123, "2021-10-18T14:38:10.123Z", Resolution::Milli)]
    #[case(
        1_634_567_890_123_456,
        "2021-10-18T14:38:10.123456Z",
        Resolution::Micro
    )]
    #[case(
        1_634_567_890_123_456_789,
        "2021-10-18T14:38:10.123456789Z",
        Resolution::Nano
    )]
    #[case(1_634_567, "1970-01-19T22:02:47Z", Resolution::Second)]
    #[case(1_000_000_000_000, "2001-09-09T01:46:40Z", Resolution::Milli)]
    #[case(1_000_000_000_000_000, "2001-09-09T01:46:40Z", Resolution::Micro)]
    #[case(1_000_000_000_000_000_000, "2001-09-09T01:46:40Z", Resolution::Nano)]
    #[case(
        253_402_207_200_999_999_999,
        "9999-12-30T22:00:00.999999999Z",
        Resolution::Nano
    )]
    fn test_to_timestamp(
        #[case] input: u128,
        #[case] expected: &str,
        #[case] resolution: Resolution,
    ) {
        let actual = to_timestamp(input).unwrap();
        assert_eq!(actual.timestamp, expected.parse::<Timestamp>().unwrap());
        assert_eq!(actual.resolution, resolution);
    }

    #[rstest]
    #[case("-1")]
    #[case("1.5")]
    #[case(" 123 ")]
    #[case("253402207201")]
    #[case("999999999999")]
    #[case("999999999999999")]
    #[case("999999999999999999")]
    #[case("253402207201000000000")]
    #[case("1634567890123456789000")]
    #[case("340282366920938463463374607431768211455")]
    fn test_parse_as_unix_timestamp_rejects_invalid_input(#[case] input: &str) {
        assert!(parse_as_unix_timestamp(input).is_none());
    }

    #[rstest]
    #[case("2021-10-18T14:38:10.123456789Z", 1_634_567_890)]
    #[case("2021-10-18T23:38:10+09:00", 1_634_567_890)]
    #[case(" 2021-10-18 14:38:10Z \n", 1_634_567_890)]
    #[case("1970-01-01T00:00:00Z", 0)]
    #[case("1969-12-31T23:59:59.5Z", -1)]
    #[case("1969-12-31T23:59:59Z", -1)]
    #[case("1969-12-31T23:59:58.999999999Z", -2)]
    fn test_datetime_to_unix_seconds(#[case] input: &str, #[case] expected: i128) {
        assert_eq!(to_unix_seconds(parse_as_datetime(input).unwrap()), expected);
    }

    #[rstest]
    #[case("2021-10-18T14:38:10")]
    #[case("2021-02-29T00:00:00Z")]
    #[case("2021-10-18T14:38:10Z trailing")]
    #[case("+010000-01-01T00:00:00Z")]
    fn test_parse_as_datetime_rejects_invalid_input(#[case] input: &str) {
        assert!(parse_as_datetime(input).is_none());
    }

    #[rstest]
    #[case("2021-10-18T14:38:10Z", "2021-10-18T14:38:10+00:00")]
    #[case("2021-10-18T14:38:10.100Z", "2021-10-18T14:38:10.1+00:00")]
    #[case("2021-10-18T14:38:10.123456Z", "2021-10-18T14:38:10.123456+00:00")]
    #[case(
        "2021-10-18T14:38:10.123456789Z",
        "2021-10-18T14:38:10.123456789+00:00"
    )]
    fn test_format_datetime(#[case] input: &str, #[case] expected: &str) {
        let timestamp = input.parse().unwrap();
        assert_eq!(
            format_datetime(timestamp, DEFAULT_OUTPUT_FORMAT, TimeZone::UTC).unwrap(),
            expected
        );
    }

    #[rstest]
    #[case("%Y-%m-%d", "1970-01-01")]
    #[case("%9f", "000000000")]
    #[case("%%", "%")]
    #[case("", "")]
    fn test_format_datetime_custom_format(#[case] format: &str, #[case] expected: &str) {
        assert_eq!(
            format_datetime(Timestamp::UNIX_EPOCH, format, TimeZone::UTC).unwrap(),
            expected
        );
    }

    #[rstest]
    #[case("%")]
    #[case("%Y-%m-%dT%H:%M:%S%.f%:z%")]
    #[case("%J")]
    #[case("%+")]
    fn test_format_datetime_rejects_invalid_format(#[case] format: &str) {
        assert!(format_datetime(Timestamp::UNIX_EPOCH, format, TimeZone::UTC).is_err());
    }

    #[rstest]
    #[case("Asia/Tokyo", "2024-01-01T00:00:00Z", "2024-01-01T09:00:00+09:00")]
    #[case(
        "America/New_York",
        "2024-01-01T00:00:00Z",
        "2023-12-31T19:00:00-05:00"
    )]
    #[case(
        "America/New_York",
        "2024-07-01T00:00:00Z",
        "2024-06-30T20:00:00-04:00"
    )]
    fn test_format_datetime_in_time_zone(
        #[case] time_zone: &str,
        #[case] input: &str,
        #[case] expected: &str,
    ) {
        assert_eq!(
            format_datetime(
                input.parse().unwrap(),
                DEFAULT_OUTPUT_FORMAT,
                TimeZone::get(time_zone).unwrap(),
            )
            .unwrap(),
            expected
        );
    }

    #[test]
    fn test_update_output_recovers_from_errors() {
        let mut page = UnixTimePage::new(true);
        page.cur.input = Input::new("1634567890".into());
        page.update_output();
        assert_eq!(page.cur.output, "2021-10-18T14:38:10+00:00");

        page.cur.output_format = Input::new("%".into());
        page.update_output();
        assert!(page.cur.output.is_empty());
        assert!(matches!(page.cur.output_format_status, Status::Warn(_)));

        page.cur.output_format = Input::new("%Y-%m-%d".into());
        page.update_output();
        assert_eq!(page.cur.output, "2021-10-18");
        assert!(matches!(page.cur.output_format_status, Status::None));

        page.cur.input = Input::new("invalid".into());
        page.update_output();
        assert!(page.cur.output.is_empty());
        assert!(matches!(page.cur.input_status, Status::Warn(_)));

        page.cur.input = Input::new("1969-12-31T23:59:59.5Z".into());
        page.update_output();
        assert_eq!(page.cur.output, "-1");
        assert!(matches!(page.cur.input_status, Status::Info(_)));
    }

    #[rstest]
    #[case("", "")]
    #[case("invalid", "")]
    #[case("2021-10-18T14:38:10Z", "1634567890")]
    fn test_update_output_clears_stale_format_error(#[case] input: &str, #[case] expected: &str) {
        let mut page = UnixTimePage::new(true);
        page.cur.input = Input::new("1634567890".into());
        page.cur.output_format = Input::new("%".into());
        page.update_output();
        assert!(matches!(page.cur.output_format_status, Status::Warn(_)));

        page.cur.input = Input::new(input.into());
        page.update_output();
        assert_eq!(page.cur.output, expected);
        assert!(matches!(page.cur.output_format_status, Status::None));
        if input.is_empty() {
            assert!(matches!(page.cur.input_status, Status::None));
        }
    }
}
