use crossterm::event::KeyCode;
use itsuki::zero_indexed_enum;
use md5::{Digest, Md5};
use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Padding, Paragraph, Wrap},
    Frame,
};
use ratatui_macros::vertical;
use sha1::Sha1;
use sha2::{Sha224, Sha256, Sha384, Sha512, Sha512_224, Sha512_256};

use crate::{
    key_code, key_code_char,
    msg::Msg,
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
        let algo_sel = AlgoItemSelect::Md5;
        let input = String::new();
        let output = calculate_hash(&input, algo_sel);
        HashPage {
            focused,
            cur: CurrentStatus {
                item: PageItems::Algo,
                algo_sel,
                enc_sel: EncodeItemSelect::Utf8,
                input,
                input_state: ScrollOutputState::default(),
                output,
            },
        }
    }
}

zero_indexed_enum! {
    PageItems => [Algo, Encode, Input, Output]
}

zero_indexed_enum! {
    AlgoItemSelect => [
        Md5,
        Sha1,
        Sha224,
        Sha256,
        Sha384,
        Sha512_224,
        Sha512_256,
        Sha512,
    ]
}

impl AlgoItemSelect {
    fn str(&self) -> &str {
        match self {
            AlgoItemSelect::Md5 => "MD5",
            AlgoItemSelect::Sha1 => "SHA-1",
            AlgoItemSelect::Sha224 => "SHA-224",
            AlgoItemSelect::Sha256 => "SHA-256",
            AlgoItemSelect::Sha384 => "SHA-384",
            AlgoItemSelect::Sha512_224 => "SHA-512/224",
            AlgoItemSelect::Sha512_256 => "SHA-512/256",
            AlgoItemSelect::Sha512 => "SHA-512",
        }
    }

    fn strings_vec() -> Vec<String> {
        Self::vars_vec().iter().map(|s| s.str().into()).collect()
    }
}

zero_indexed_enum! {
    EncodeItemSelect => [
        Utf8,
    ]
}

impl EncodeItemSelect {
    fn str(&self) -> &str {
        match self {
            EncodeItemSelect::Utf8 => "UTF-8",
        }
    }

    fn strings_vec() -> Vec<String> {
        Self::vars_vec().iter().map(|s| s.str().into()).collect()
    }
}

impl Page for HashPage {
    fn handle_key(&self, key: crossterm::event::KeyEvent) -> Option<Msg> {
        match key {
            key_code!(KeyCode::Esc) => Some(Msg::Quit),
            key_code_char!('n', Ctrl) => Some(Msg::HashPageSelectNextItem),
            key_code_char!('p', Ctrl) => Some(Msg::HashPageSelectPrevItem),
            key_code_char!('l') | key_code!(KeyCode::Right) => {
                Some(Msg::HashPageCurrentItemSelectNext)
            }
            key_code_char!('h') | key_code!(KeyCode::Left) => {
                Some(Msg::HashPageCurrentItemSelectPrev)
            }
            key_code_char!('j') | key_code!(KeyCode::Down) => Some(Msg::HashPageScrollDown),
            key_code_char!('k') | key_code!(KeyCode::Up) => Some(Msg::HashPageScrollUp),
            key_code_char!('y') => Some(Msg::HashPageCopy),
            key_code_char!('p') => Some(Msg::HashPagePaste),
            _ => None,
        }
    }

    fn update(&mut self, msg: Msg) -> Option<Msg> {
        match msg {
            Msg::HashPageSelectNextItem => {
                self.select_next_item();
            }
            Msg::HashPageSelectPrevItem => {
                self.select_prev_item();
            }
            Msg::HashPageCurrentItemSelectNext => {
                self.current_item_select_next();
            }
            Msg::HashPageCurrentItemSelectPrev => {
                self.current_item_select_prev();
            }
            Msg::HashPageScrollDown => {
                self.scroll_down();
            }
            Msg::HashPageScrollUp => {
                self.scroll_up();
            }
            Msg::HashPageCopy => {
                return self.copy_to_clipboard();
            }
            Msg::HashPagePaste => {
                self.paste_from_clipboard();
            }
            _ => {}
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
        let input = ScrollOutput::new(input_text, self.focused, self.cur.item == PageItems::Input);
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
        helps.push("<C-n/C-p> Select item");
        if matches!(self.cur.item, PageItems::Algo | PageItems::Encode) {
            helps.push("<Left/Right> Select current item value");
        }
        if matches!(self.cur.item, PageItems::Output) {
            helps.push("<y> Copy to clipboard");
        }
        if matches!(self.cur.item, PageItems::Input) {
            helps.push("<Down/Up> Scroll down/up");
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
                if self.cur.algo_sel.val() < AlgoItemSelect::len() - 1 {
                    self.cur.algo_sel = self.cur.algo_sel.next();
                }
                self.update_hash();
            }
            PageItems::Encode => {
                if self.cur.enc_sel.val() < EncodeItemSelect::len() - 1 {
                    self.cur.enc_sel = self.cur.enc_sel.next();
                }
                self.update_hash();
            }
            PageItems::Input => {}
            PageItems::Output => {}
        }
    }

    fn current_item_select_prev(&mut self) {
        match self.cur.item {
            PageItems::Algo => {
                if self.cur.algo_sel.val() > 0 {
                    self.cur.algo_sel = self.cur.algo_sel.prev();
                }
                self.update_hash();
            }
            PageItems::Encode => {
                if self.cur.enc_sel.val() > 0 {
                    self.cur.enc_sel = self.cur.enc_sel.prev();
                }
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
        if self.cur.input_state.offset < self.cur.input.lines().count() - 1 {
            self.cur.input_state.offset += 1;
        }
    }

    fn scroll_up(&mut self) {
        if !matches!(self.cur.item, PageItems::Input) || self.cur.input.is_empty() {
            return;
        }
        if self.cur.input_state.offset > 0 {
            self.cur.input_state.offset -= 1;
        }
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
