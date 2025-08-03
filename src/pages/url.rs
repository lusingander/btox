use itsuki::zero_indexed_enum;
use percent_encoding::{percent_decode_str, utf8_percent_encode, AsciiSet, CONTROLS};
use ratatui::{
    crossterm::event::KeyCode,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Padding, Paragraph, Wrap},
    Frame,
};
use ratatui_macros::vertical;

use crate::{
    fn_next_prev_mut, fn_str_map, key_code, key_code_char,
    msg::{Msg, PageMsg, UrlMsg},
    pages::{page::Page, util},
    widget::{
        scroll::{ScrollOutput, ScrollOutputState},
        select::Select,
    },
};

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

pub struct UrlPage {
    focused: bool,
    cur: CurrentStatus,
}

#[derive(Default)]
struct CurrentStatus {
    item: PageItems,
    eod_sel: EncodeOrDecodeSelect,
    charset_sel: CharsetSelect,
    input: String,
    input_state: ScrollOutputState,
    output: String,
    status: InputStatus,
}

impl UrlPage {
    pub fn new(focused: bool) -> UrlPage {
        UrlPage {
            focused,
            cur: CurrentStatus::default(),
        }
    }
}

#[derive(Default)]
enum InputStatus {
    #[default]
    None,
    Warn(String),
}

#[derive(Default)]
#[zero_indexed_enum]
enum PageItems {
    #[default]
    EncodeOrDecode,
    Charset,
    Input,
    Output,
}

#[derive(Default)]
#[zero_indexed_enum]
enum EncodeOrDecodeSelect {
    #[default]
    Encode,
    Decode,
}

impl EncodeOrDecodeSelect {
    fn_str_map! {
        EncodeOrDecodeSelect::Encode => "Encode",
        EncodeOrDecodeSelect::Decode => "Decode",
    }

    fn_next_prev_mut! {}
}

#[derive(Default)]
#[zero_indexed_enum]
enum CharsetSelect {
    #[default]
    Utf8,
}

impl CharsetSelect {
    fn_str_map! {
        CharsetSelect::Utf8 => "UTF-8",
    }

    fn_next_prev_mut! {}
}

impl Page for UrlPage {
    fn handle_key(&self, key: ratatui::crossterm::event::KeyEvent) -> Option<Msg> {
        let msg = match key {
            key_code_char!('j') | key_code!(KeyCode::Down) => UrlMsg::SelectNextItem,
            key_code_char!('k') | key_code!(KeyCode::Up) => UrlMsg::SelectPrevItem,
            key_code_char!('l') | key_code!(KeyCode::Right) => UrlMsg::CurrentItemSelectNext,
            key_code_char!('h') | key_code!(KeyCode::Left) => UrlMsg::CurrentItemSelectPrev,
            key_code_char!('e', Ctrl) => UrlMsg::ScrollDown,
            key_code_char!('y', Ctrl) => UrlMsg::ScrollUp,
            key_code_char!('y') => UrlMsg::Copy,
            key_code_char!('p') => UrlMsg::Paste,
            _ => return None,
        };
        Some(Msg::Page(PageMsg::Url(msg)))
    }

    fn update(&mut self, msg: PageMsg) -> Option<Msg> {
        if let PageMsg::Url(msg) = msg {
            match msg {
                UrlMsg::SelectNextItem => {
                    self.select_next_item();
                }
                UrlMsg::SelectPrevItem => {
                    self.select_prev_item();
                }
                UrlMsg::CurrentItemSelectNext => {
                    self.current_item_select_next();
                }
                UrlMsg::CurrentItemSelectPrev => {
                    self.current_item_select_prev();
                }
                UrlMsg::ScrollDown => {
                    self.scroll_down();
                }
                UrlMsg::ScrollUp => {
                    self.scroll_up();
                }
                UrlMsg::Copy => {
                    return self.copy_to_clipboard();
                }
                UrlMsg::Paste => {
                    self.paste_from_clipboard();
                }
            }
        }
        None
    }

    fn render(&mut self, f: &mut Frame, area: Rect) {
        let chunks = vertical![==2, ==2, >=0, ==1, >=0].split(area);

        let eod_sel = Select::new(
            EncodeOrDecodeSelect::strings_vec(),
            self.cur.eod_sel.val(),
            self.cur.item == PageItems::EncodeOrDecode,
            self.focused,
        );
        f.render_widget(eod_sel, chunks[0]);

        let charset_sel = Select::new(
            CharsetSelect::strings_vec(),
            self.cur.charset_sel.val(),
            self.cur.item == PageItems::Charset,
            self.focused,
        );
        f.render_widget(charset_sel, chunks[1]);

        let input_text = self.cur.input.clone();
        let input = ScrollOutput::new(input_text, self.focused, self.cur.item == PageItems::Input)
            .title("Input");
        f.render_stateful_widget(input, chunks[2], &mut self.cur.input_state);

        if let InputStatus::Warn(status) = &self.cur.status {
            let status_style = Style::default().fg(Color::Red);
            let status = Paragraph::new(status.as_str()).block(
                Block::default()
                    .borders(Borders::empty())
                    .style(status_style)
                    .padding(Padding::horizontal(1)),
            );
            f.render_widget(status, chunks[3]);
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

        let output_text = self.cur.output.clone();
        let output = Paragraph::new(output_text)
            .block(
                Block::bordered()
                    .style(output_style)
                    .title("Output")
                    .padding(Padding::horizontal(1)),
            )
            .wrap(Wrap { trim: false });
        f.render_widget(output, chunks[4]);
    }

    fn focus(&mut self) {
        self.focused = true;
    }

    fn unfocus(&mut self) {
        self.focused = false;
    }

    fn helps(&self) -> Vec<&str> {
        let mut helps: Vec<&str> = Vec::new();
        helps.push("<j/k> Select item");
        if matches!(
            self.cur.item,
            PageItems::EncodeOrDecode | PageItems::Charset
        ) {
            helps.push("<h/l> Select current item value");
        }
        if matches!(self.cur.item, PageItems::Output) {
            helps.push("<y> Copy to clipboard");
        }
        if matches!(self.cur.item, PageItems::Input) {
            helps.push("<C-e/C-y> Scroll down/up");
            helps.push("<p> Paste from clipboard");
        }
        helps
    }
}

impl UrlPage {
    fn select_next_item(&mut self) {
        self.cur.item = self.cur.item.next();
    }

    fn select_prev_item(&mut self) {
        self.cur.item = self.cur.item.prev();
    }

    fn current_item_select_next(&mut self) {
        match self.cur.item {
            PageItems::EncodeOrDecode => {
                self.cur.eod_sel.next_mut();
                self.update_output();
            }
            PageItems::Charset => {
                self.cur.charset_sel.next_mut();
                self.update_output();
            }
            PageItems::Input => {}
            PageItems::Output => {}
        }
    }

    fn current_item_select_prev(&mut self) {
        match self.cur.item {
            PageItems::EncodeOrDecode => {
                self.cur.eod_sel.prev_mut();
                self.update_output();
            }
            PageItems::Charset => {
                self.cur.charset_sel.prev_mut();
                self.update_output();
            }
            PageItems::Input => {}
            PageItems::Output => {}
        }
    }

    fn scroll_down(&mut self) {
        if !matches!(self.cur.item, PageItems::Input) || self.cur.input.is_empty() {
            return;
        }
        self.cur.input_state.scroll_down();
    }

    fn scroll_up(&mut self) {
        if !matches!(self.cur.item, PageItems::Input) || self.cur.input.is_empty() {
            return;
        }
        self.cur.input_state.scroll_up();
    }

    fn copy_to_clipboard(&self) -> Option<Msg> {
        if !matches!(self.cur.item, PageItems::Output) {
            return None;
        }

        let text = &self.cur.output;
        util::copy_to_clipboard(text)
    }

    fn paste_from_clipboard(&mut self) {
        if matches!(self.cur.item, PageItems::Input) {
            self.cur.input = util::paste_from_clipboard().unwrap();
            self.update_output();
        }
    }

    fn update_output(&mut self) {
        (self.cur.output, self.cur.status) =
            calculate_url(&self.cur.input, self.cur.eod_sel, self.cur.charset_sel);
    }
}

fn calculate_url(
    input: &str,
    eod_sel: EncodeOrDecodeSelect,
    charset_sel: CharsetSelect,
) -> (String, InputStatus) {
    match eod_sel {
        EncodeOrDecodeSelect::Encode => {
            let output = match charset_sel {
                CharsetSelect::Utf8 => utf8_percent_encode(input, FRAGMENT).to_string(),
            };
            (output, InputStatus::None)
        }
        EncodeOrDecodeSelect::Decode => {
            let output = percent_decode_str(input).decode_utf8();
            match output {
                Ok(decoded) => (decoded.to_string(), InputStatus::None),
                Err(_) => (
                    String::new(),
                    InputStatus::Warn("invalid UTF-8 sequence".into()),
                ),
            }
        }
    }
}
