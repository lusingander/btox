use base64::{engine::general_purpose, Engine as _};
use itsuki::zero_indexed_enum;
use ratatui::{
    crossterm::event::KeyCode,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Padding, Paragraph, Wrap},
    Frame,
};
use ratatui_macros::vertical;

use crate::{
    fn_next_prev_mut, fn_str_map, key_code, key_code_char,
    msg::{Base64Msg, Msg, PageMsg},
    pages::{page::Page, util},
    widget::{
        scroll::{ScrollOutput, ScrollOutputState},
        select::Select,
    },
};

pub struct Base64Page {
    focused: bool,
    cur: CurrentStatus,
}

#[derive(Default)]
struct CurrentStatus {
    item: PageItems,
    eod_sel: EncodeOrDecodeSelect,
    input: String,
    input_state: ScrollOutputState,
    output: String,
}

impl Base64Page {
    pub fn new(focused: bool) -> Base64Page {
        let eod_sel = EncodeOrDecodeSelect::default();
        let input = String::new();
        let output = calculate_base64(&input, eod_sel);
        Base64Page {
            focused,
            cur: CurrentStatus {
                eod_sel,
                input,
                output,
                ..Default::default()
            },
        }
    }
}

#[derive(Default)]
#[zero_indexed_enum]
enum PageItems {
    #[default]
    EncodeOrDecode,
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

impl Page for Base64Page {
    fn handle_key(&self, key: ratatui::crossterm::event::KeyEvent) -> Option<Msg> {
        let msg = match key {
            key_code_char!('j') | key_code!(KeyCode::Down) => Base64Msg::SelectNextItem,
            key_code_char!('k') | key_code!(KeyCode::Up) => Base64Msg::SelectPrevItem,
            key_code_char!('l') | key_code!(KeyCode::Right) => Base64Msg::CurrentItemSelectNext,
            key_code_char!('h') | key_code!(KeyCode::Left) => Base64Msg::CurrentItemSelectPrev,
            key_code_char!('e', Ctrl) => Base64Msg::ScrollDown,
            key_code_char!('y', Ctrl) => Base64Msg::ScrollUp,
            key_code_char!('y') => Base64Msg::Copy,
            key_code_char!('p') => Base64Msg::Paste,
            _ => return None,
        };
        Some(Msg::Page(PageMsg::Base64(msg)))
    }

    fn update(&mut self, msg: PageMsg) -> Option<Msg> {
        if let PageMsg::Base64(msg) = msg {
            match msg {
                Base64Msg::SelectNextItem => {
                    self.select_next_item();
                }
                Base64Msg::SelectPrevItem => {
                    self.select_prev_item();
                }
                Base64Msg::CurrentItemSelectNext => {
                    self.current_item_select_next();
                }
                Base64Msg::CurrentItemSelectPrev => {
                    self.current_item_select_prev();
                }
                Base64Msg::ScrollDown => {
                    self.scroll_down();
                }
                Base64Msg::ScrollUp => {
                    self.scroll_up();
                }
                Base64Msg::Copy => {
                    return self.copy_to_clipboard();
                }
                Base64Msg::Paste => {
                    self.paste_from_clipboard();
                }
            }
        }
        None
    }

    fn render(&mut self, f: &mut Frame, area: Rect) {
        let chunks = vertical![==2, >=0, ==5].split(area);

        let eod_sel = Select::new(
            EncodeOrDecodeSelect::strings_vec(),
            self.cur.eod_sel.val(),
            self.cur.item == PageItems::EncodeOrDecode,
            self.focused,
        );
        f.render_widget(eod_sel, chunks[0]);

        let input_text = self.cur.input.clone();
        let input = ScrollOutput::new(input_text, self.focused, self.cur.item == PageItems::Input)
            .title("Input");
        f.render_stateful_widget(input, chunks[1], &mut self.cur.input_state);

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
        f.render_widget(output, chunks[2]);
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
        if matches!(self.cur.item, PageItems::EncodeOrDecode) {
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

impl Base64Page {
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
        self.cur.output = calculate_base64(&self.cur.input, self.cur.eod_sel);
    }
}

fn calculate_base64(input: &str, eod_sel: EncodeOrDecodeSelect) -> String {
    match eod_sel {
        EncodeOrDecodeSelect::Encode => general_purpose::STANDARD.encode(input),
        EncodeOrDecodeSelect::Decode => {
            if let Ok(decoded) = general_purpose::STANDARD.decode(input) {
                String::from_utf8_lossy(&decoded).to_string()
            } else {
                "Invalid Base64".to_string()
            }
        }
    }
}
