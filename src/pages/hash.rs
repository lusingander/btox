use arboard::Clipboard;
use crossterm::event::KeyCode;
use itsuki::zero_indexed_enum;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Padding, Paragraph, Widget},
};

use crate::{key_code, key_code_char, msg::Msg, pages::page::Page, widget::select::Select};

pub struct HashPage {
    focused: bool,
    cur: CurrentStatus,
}

struct CurrentStatus {
    item: PageItems,
    algo_sel: AlgoItemSelect,
    enc_sel: EncodeItemSelect,
}

impl HashPage {
    pub fn new(focused: bool) -> HashPage {
        HashPage {
            focused,
            cur: CurrentStatus {
                item: PageItems::Algo,
                algo_sel: AlgoItemSelect::Md5,
                enc_sel: EncodeItemSelect::Utf8,
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
            key_code_char!('j') => Some(Msg::HashPageSelectNextItem),
            key_code_char!('k') => Some(Msg::HashPageSelectPrevItem),
            key_code_char!('l') => Some(Msg::HashPageCurrentItemSelectNext),
            key_code_char!('h') => Some(Msg::HashPageCurrentItemSelectPrev),
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
            Msg::HashPageCopy => {
                return self.copy_to_clipboard();
            }
            Msg::HashPagePaste => {
                return self.paste_from_clipboard();
            }
            _ => {}
        }
        None
    }

    fn render(&self, buf: &mut Buffer, area: Rect) {
        let chunks = Layout::vertical([
            Constraint::Length(2),
            Constraint::Length(2),
            Constraint::Min(0),
            Constraint::Length(5),
        ])
        .split(area);

        let algo_sel = Select::new(
            AlgoItemSelect::strings_vec(),
            self.cur.algo_sel.val(),
            self.cur.item == PageItems::Algo,
            self.focused,
        );
        algo_sel.render(chunks[0], buf);

        let enc_sel = Select::new(
            EncodeItemSelect::strings_vec(),
            self.cur.enc_sel.val(),
            self.cur.item == PageItems::Encode,
            self.focused,
        );
        enc_sel.render(chunks[1], buf);

        let input_style = if self.focused {
            if self.cur.item == PageItems::Input {
                Style::default().fg(Color::Blue)
            } else {
                Style::default().fg(Color::Reset)
            }
        } else {
            Style::default().fg(Color::DarkGray)
        };

        let input = Paragraph::new("").block(
            Block::bordered()
                .style(input_style)
                .title("Input")
                .padding(Padding::horizontal(1)),
        );
        input.render(chunks[2], buf);

        let output_style = if self.focused {
            if self.cur.item == PageItems::Output {
                Style::default().fg(Color::Blue)
            } else {
                Style::default().fg(Color::Reset)
            }
        } else {
            Style::default().fg(Color::DarkGray)
        };

        let output = Paragraph::new("").block(
            Block::bordered()
                .style(output_style)
                .title("Output")
                .padding(Padding::horizontal(1)),
        );
        output.render(chunks[3], buf);
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
            }
            PageItems::Encode => {
                if self.cur.enc_sel.val() < EncodeItemSelect::len() - 1 {
                    self.cur.enc_sel = self.cur.enc_sel.next();
                }
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
            }
            PageItems::Encode => {
                if self.cur.enc_sel.val() > 0 {
                    self.cur.enc_sel = self.cur.enc_sel.prev();
                }
            }
            PageItems::Input => {}
            PageItems::Output => {}
        }
    }

    fn copy_to_clipboard(&self) -> Option<Msg> {
        if !matches!(self.cur.item, PageItems::Output) {
            return None;
        }

        let text = "";
        let result = Clipboard::new().and_then(|mut c| c.set_text(text));
        match result {
            Ok(_) => Some(Msg::NotifyInfo("Copy to clipboard succeeded".into())),
            Err(_) => Some(Msg::NotifyError("Copy to clipboard failed".into())),
        }
    }

    fn paste_from_clipboard(&mut self) -> Option<Msg> {
        if !matches!(self.cur.item, PageItems::Input) {
            return None;
        }

        let _ = Clipboard::new().and_then(|mut c| c.get_text()).unwrap();
        None
    }
}
