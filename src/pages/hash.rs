use itsuki::zero_indexed_enum;
use md5::{Digest, Md5};
use ratatui::{
    crossterm::event::KeyCode,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Padding, Paragraph, Wrap},
    Frame,
};
use ratatui_macros::vertical;
use sha1::Sha1;
use sha2::{Sha224, Sha256, Sha384, Sha512, Sha512_224, Sha512_256};

use crate::{
    fn_next_prev_mut, fn_str_map, key_code, key_code_char,
    msg::{HashMsg, Msg, PageMsg},
    pages::{page::Page, util},
    widget::{
        scroll::{ScrollOutput, ScrollOutputState},
        select::Select,
    },
};

pub struct HashPage {
    focused: bool,
    cur: CurrentStatus,
}

#[derive(Default)]
struct CurrentStatus {
    item: PageItems,
    algo_sel: AlgoItemSelect,
    enc_sel: EncodeItemSelect,
    input: String,
    input_state: ScrollOutputState,
    output: String,
}

impl HashPage {
    pub fn new(focused: bool) -> HashPage {
        let algo_sel = AlgoItemSelect::default();
        let input = String::new();
        let output = calculate_hash(&input, algo_sel);
        HashPage {
            focused,
            cur: CurrentStatus {
                algo_sel,
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
    Algo,
    Encode,
    Input,
    Output,
}

#[derive(Default)]
#[zero_indexed_enum]
enum AlgoItemSelect {
    #[default]
    Md5,
    Sha1,
    Sha224,
    Sha256,
    Sha384,
    Sha512_224,
    Sha512_256,
    Sha512,
}

impl AlgoItemSelect {
    fn_str_map! {
        AlgoItemSelect::Md5 => "MD5",
        AlgoItemSelect::Sha1 => "SHA-1",
        AlgoItemSelect::Sha224 => "SHA-224",
        AlgoItemSelect::Sha256 => "SHA-256",
        AlgoItemSelect::Sha384 => "SHA-384",
        AlgoItemSelect::Sha512_224 => "SHA-512/224",
        AlgoItemSelect::Sha512_256 => "SHA-512/256",
        AlgoItemSelect::Sha512 => "SHA-512",
    }

    fn_next_prev_mut! {}
}

#[derive(Default)]
#[zero_indexed_enum]
enum EncodeItemSelect {
    #[default]
    Utf8,
}

impl EncodeItemSelect {
    fn_str_map! {
        EncodeItemSelect::Utf8 => "UTF-8",
    }

    fn_next_prev_mut! {}
}

impl Page for HashPage {
    fn handle_key(&self, key: ratatui::crossterm::event::KeyEvent) -> Option<Msg> {
        let msg = match key {
            key_code_char!('j') | key_code!(KeyCode::Down) => HashMsg::SelectNextItem,
            key_code_char!('k') | key_code!(KeyCode::Up) => HashMsg::SelectPrevItem,
            key_code_char!('l') | key_code!(KeyCode::Right) => HashMsg::CurrentItemSelectNext,
            key_code_char!('h') | key_code!(KeyCode::Left) => HashMsg::CurrentItemSelectPrev,
            key_code_char!('e', Ctrl) => HashMsg::ScrollDown,
            key_code_char!('y', Ctrl) => HashMsg::ScrollUp,
            key_code_char!('y') => HashMsg::Copy,
            key_code_char!('p') => HashMsg::Paste,
            _ => return None,
        };
        Some(Msg::Page(PageMsg::Hash(msg)))
    }

    fn update(&mut self, msg: PageMsg) -> Option<Msg> {
        if let PageMsg::Hash(msg) = msg {
            match msg {
                HashMsg::SelectNextItem => {
                    self.select_next_item();
                }
                HashMsg::SelectPrevItem => {
                    self.select_prev_item();
                }
                HashMsg::CurrentItemSelectNext => {
                    self.current_item_select_next();
                }
                HashMsg::CurrentItemSelectPrev => {
                    self.current_item_select_prev();
                }
                HashMsg::ScrollDown => {
                    self.scroll_down();
                }
                HashMsg::ScrollUp => {
                    self.scroll_up();
                }
                HashMsg::Copy => {
                    return self.copy_to_clipboard();
                }
                HashMsg::Paste => {
                    self.paste_from_clipboard();
                }
            }
        }
        None
    }

    fn render(&mut self, f: &mut Frame, area: Rect) {
        let chunks = vertical![==2, ==2, >=0, ==5].split(area);

        let algo_sel = Select::new(
            AlgoItemSelect::strings_vec(),
            self.cur.algo_sel.val(),
            self.cur.item == PageItems::Algo,
            self.focused,
        );
        f.render_widget(algo_sel, chunks[0]);

        let enc_sel = Select::new(
            EncodeItemSelect::strings_vec(),
            self.cur.enc_sel.val(),
            self.cur.item == PageItems::Encode,
            self.focused,
        );
        f.render_widget(enc_sel, chunks[1]);

        let input_text = self.cur.input.clone();
        let input = ScrollOutput::new(input_text, self.focused, self.cur.item == PageItems::Input)
            .title("Input");
        f.render_stateful_widget(input, chunks[2], &mut self.cur.input_state);

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
        f.render_widget(output, chunks[3]);
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
        if matches!(self.cur.item, PageItems::Algo | PageItems::Encode) {
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

impl HashPage {
    fn select_next_item(&mut self) {
        self.cur.item = self.cur.item.next();
    }

    fn select_prev_item(&mut self) {
        self.cur.item = self.cur.item.prev();
    }

    fn current_item_select_next(&mut self) {
        match self.cur.item {
            PageItems::Algo => {
                self.cur.algo_sel.next_mut();
                self.update_hash();
            }
            PageItems::Encode => {
                self.cur.enc_sel.next_mut();
                self.update_hash();
            }
            PageItems::Input => {}
            PageItems::Output => {}
        }
    }

    fn current_item_select_prev(&mut self) {
        match self.cur.item {
            PageItems::Algo => {
                self.cur.algo_sel.prev_mut();
                self.update_hash();
            }
            PageItems::Encode => {
                self.cur.enc_sel.prev_mut();
                self.update_hash();
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

            self.update_hash();
        }
    }

    fn update_hash(&mut self) {
        self.cur.output = calculate_hash(&self.cur.input, self.cur.algo_sel);
    }
}

fn calculate_hash(input: &str, algo_sel: AlgoItemSelect) -> String {
    let input_bytes = input.as_bytes();
    match algo_sel {
        AlgoItemSelect::Md5 => hash_to_str(&Md5::digest(input_bytes)),
        AlgoItemSelect::Sha1 => hash_to_str(&Sha1::digest(input_bytes)),
        AlgoItemSelect::Sha224 => hash_to_str(&Sha224::digest(input_bytes)),
        AlgoItemSelect::Sha256 => hash_to_str(&Sha256::digest(input_bytes)),
        AlgoItemSelect::Sha384 => hash_to_str(&Sha384::digest(input_bytes)),
        AlgoItemSelect::Sha512_224 => hash_to_str(&Sha512_224::digest(input_bytes)),
        AlgoItemSelect::Sha512_256 => hash_to_str(&Sha512_256::digest(input_bytes)),
        AlgoItemSelect::Sha512 => hash_to_str(&Sha512::digest(input_bytes)),
    }
}

fn hash_to_str(hash: &[u8]) -> String {
    let mut buf = [0u8; 128];
    base16ct::lower::encode_str(hash, &mut buf)
        .unwrap()
        .to_string()
}
