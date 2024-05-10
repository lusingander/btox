use crossterm::event::KeyCode;
use itsuki::zero_indexed_enum;
use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Padding, Paragraph},
    Frame,
};
use ratatui_macros::vertical;
use tui_input::{backend::crossterm::EventHandler, Input};

use crate::{
    key_code, key_code_char,
    msg::Msg,
    pages::{page::Page, util},
    widget::select::Select,
};

pub struct NumberBasePage {
    focused: bool,
    cur: CurrentStatus,
}

struct CurrentStatus {
    item: PageItems,
    binary_input: Input,
    octal_input: Input,
    decimal_input: Input,
    hex_input: Input,
    binary_status: String,
    octal_status: String,
    decimal_status: String,
    hex_status: String,
    case_sel: CaseItemSelect,
    edit: bool,
}

impl NumberBasePage {
    pub fn new(focused: bool) -> NumberBasePage {
        NumberBasePage {
            focused,
            cur: CurrentStatus {
                item: PageItems::Binary,
                binary_input: Input::default(),
                octal_input: Input::default(),
                decimal_input: Input::default(),
                hex_input: Input::default(),
                binary_status: String::new(),
                octal_status: String::new(),
                decimal_status: String::new(),
                hex_status: String::new(),
                case_sel: CaseItemSelect::Lowercase,
                edit: false,
            },
        }
    }
}

zero_indexed_enum! {
    PageItems => [Binary, Octal, Decimal, Hexadecimal, Case]
}

impl PageItems {
    fn str(&self) -> &str {
        match self {
            PageItems::Binary => "Binary",
            PageItems::Octal => "Octal",
            PageItems::Decimal => "Decimal",
            PageItems::Hexadecimal => "Hexadecimal",
            PageItems::Case => "", // not used
        }
    }
}

zero_indexed_enum! {
    CaseItemSelect => [Lowercase, Uppercase]
}

impl CaseItemSelect {
    fn str(&self) -> &str {
        match self {
            CaseItemSelect::Lowercase => "Lowercase",
            CaseItemSelect::Uppercase => "Uppercase",
        }
    }

    fn strings_vec() -> Vec<String> {
        Self::vars_vec().iter().map(|s| s.str().into()).collect()
    }
}

impl Page for NumberBasePage {
    fn handle_key(&self, key: crossterm::event::KeyEvent) -> Option<Msg> {
        if self.cur.edit {
            return match key {
                key_code!(KeyCode::Esc) => Some(Msg::NumberBasePageEditEnd),
                _ => Some(Msg::NumberBasePageEditKeyEvent(key)),
            };
        }

        match key {
            key_code!(KeyCode::Esc) => Some(Msg::Quit),
            key_code_char!('n', Ctrl) => Some(Msg::NumberBasePageSelectNextItem),
            key_code_char!('p', Ctrl) => Some(Msg::NumberBasePageSelectPrevItem),
            key_code_char!('l') | key_code!(KeyCode::Right) => {
                Some(Msg::NumberBasePageCurrentItemSelectNext)
            }
            key_code_char!('h') | key_code!(KeyCode::Left) => {
                Some(Msg::NumberBasePageCurrentItemSelectPrev)
            }
            key_code_char!('y') => Some(Msg::NumberBasePageCopy),
            key_code_char!('p') => Some(Msg::NumberBasePagePaste),
            key_code_char!('e') => Some(Msg::NumberBasePageEditStart),
            _ => None,
        }
    }

    fn update(&mut self, msg: Msg) -> Option<Msg> {
        match msg {
            Msg::NumberBasePageSelectNextItem => {
                self.select_next_item();
            }
            Msg::NumberBasePageSelectPrevItem => {
                self.select_prev_item();
            }
            Msg::NumberBasePageCurrentItemSelectNext => {
                self.current_item_select_next();
            }
            Msg::NumberBasePageCurrentItemSelectPrev => {
                self.current_item_select_prev();
            }
            Msg::NumberBasePageCopy => {
                return self.copy_to_clipboard();
            }
            Msg::NumberBasePagePaste => {
                self.paste_from_clipboard();
            }
            Msg::NumberBasePageEditStart => {
                self.edit_start();
            }
            Msg::NumberBasePageEditEnd => {
                self.edit_end();
            }
            Msg::NumberBasePageEditKeyEvent(key) => {
                self.edit(key);
            }
            _ => {}
        }
        None
    }

    fn render(&mut self, f: &mut Frame, area: Rect) {
        let chunks = vertical![==3, ==1, ==3, ==1, ==3, ==1, ==3, ==2, ==1].split(area);

        self.render_input(f, chunks[0], &self.cur.binary_input, PageItems::Binary);

        if !self.cur.binary_status.is_empty() {
            self.render_status(f, chunks[1], self.cur.binary_status.as_str());
        }

        self.render_input(f, chunks[2], &self.cur.octal_input, PageItems::Octal);

        if !self.cur.octal_status.is_empty() {
            self.render_status(f, chunks[3], self.cur.octal_status.as_str());
        }

        self.render_input(f, chunks[4], &self.cur.decimal_input, PageItems::Decimal);

        if !self.cur.decimal_status.is_empty() {
            self.render_status(f, chunks[5], self.cur.decimal_status.as_str());
        }

        self.render_input(f, chunks[6], &self.cur.hex_input, PageItems::Hexadecimal);

        if !self.cur.hex_status.is_empty() {
            self.render_status(f, chunks[7], self.cur.hex_status.as_str());
        }

        let case_sel = Select::new(
            CaseItemSelect::strings_vec(),
            self.cur.case_sel.val(),
            self.cur.item == PageItems::Case,
            self.focused,
        );
        f.render_widget(case_sel, chunks[8]);
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
            helps.push("<e> Edit");
            helps.push("<C-n/C-p> Select item");
            helps.push("<y> Copy to clipboard");
            helps.push("<p> Paste from clipboard");
        }
        helps
    }
}

impl NumberBasePage {
    fn select_next_item(&mut self) {
        self.cur.item = self.cur.item.next();
    }

    fn select_prev_item(&mut self) {
        self.cur.item = self.cur.item.prev();
    }

    fn current_item_select_next(&mut self) {
        if let PageItems::Case = self.cur.item {
            if self.cur.case_sel.val() < CaseItemSelect::len() - 1 {
                self.cur.case_sel = self.cur.case_sel.next();
            }
            self.update_hex_case();
        }
    }

    fn current_item_select_prev(&mut self) {
        if let PageItems::Case = self.cur.item {
            if self.cur.case_sel.val() > 0 {
                self.cur.case_sel = self.cur.case_sel.prev();
            }
            self.update_hex_case();
        }
    }
    fn edit_start(&mut self) {
        if matches!(self.cur.item, PageItems::Case) {
            return;
        }
        self.cur.edit = true;
    }

    fn edit_end(&mut self) {
        if matches!(self.cur.item, PageItems::Case) {
            return;
        }
        self.cur.edit = false;
    }

    fn edit(&mut self, key: crossterm::event::KeyEvent) {
        let event = &crossterm::event::Event::Key(key);
        match self.cur.item {
            PageItems::Binary => {
                self.cur.binary_input.handle_event(event);
            }
            PageItems::Octal => {
                self.cur.octal_input.handle_event(event);
            }
            PageItems::Decimal => {
                self.cur.decimal_input.handle_event(event);
            }
            PageItems::Hexadecimal => {
                self.cur.hex_input.handle_event(event);
            }
            PageItems::Case => {
                return;
            }
        }

        self.update_numbers(self.cur.item);
    }

    fn copy_to_clipboard(&self) -> Option<Msg> {
        let text = match self.cur.item {
            PageItems::Binary => self.cur.binary_input.value(),
            PageItems::Octal => self.cur.octal_input.value(),
            PageItems::Decimal => self.cur.decimal_input.value(),
            PageItems::Hexadecimal => self.cur.hex_input.value(),
            PageItems::Case => {
                return None;
            }
        };
        util::copy_to_clipboard(text)
    }

    fn paste_from_clipboard(&mut self) {
        let text = util::paste_from_clipboard().unwrap();
        match self.cur.item {
            PageItems::Binary => {
                self.update_binary_input(text);
            }
            PageItems::Octal => {
                self.update_octal_input(text);
            }
            PageItems::Decimal => {
                self.update_decimal_input(text);
            }
            PageItems::Hexadecimal => {
                self.update_hex_input(text);
            }
            PageItems::Case => {
                return;
            }
        }

        self.update_numbers(self.cur.item);
    }

    fn update_numbers(&mut self, updated_item: PageItems) {
        #[allow(clippy::from_str_radix_10)]
        let updated_value = match updated_item {
            PageItems::Binary => u128::from_str_radix(self.cur.binary_input.value(), 2),
            PageItems::Octal => u128::from_str_radix(self.cur.octal_input.value(), 8),
            PageItems::Decimal => u128::from_str_radix(self.cur.decimal_input.value(), 10),
            PageItems::Hexadecimal => u128::from_str_radix(self.cur.hex_input.value(), 16),
            PageItems::Case => {
                return;
            }
        };
        match updated_value {
            Ok(value) => {
                self.update_binary_input(format!("{:b}", value));
                self.update_octal_input(format!("{:o}", value));
                self.update_decimal_input(format!("{}", value));
                self.update_hex_input(format!("{:x}", value));
                self.update_hex_case();
                self.cur.binary_status = String::new();
                self.cur.octal_status = String::new();
                self.cur.decimal_status = String::new();
                self.cur.hex_status = String::new();
            }
            Err(_) => match updated_item {
                PageItems::Binary => {
                    self.cur.binary_status = "Invalid binary number".into();
                }
                PageItems::Octal => {
                    self.cur.octal_status = "Invalid octal number".into();
                }
                PageItems::Decimal => {
                    self.cur.decimal_status = "Invalid decimal number".into();
                }
                PageItems::Hexadecimal => {
                    self.cur.hex_status = "Invalid hexadecimal number".into();
                }
                PageItems::Case => {}
            },
        }
    }

    fn update_binary_input(&mut self, value: String) {
        self.cur.binary_input = self.cur.binary_input.clone().with_value(value);
    }

    fn update_octal_input(&mut self, value: String) {
        self.cur.octal_input = self.cur.octal_input.clone().with_value(value);
    }

    fn update_decimal_input(&mut self, value: String) {
        self.cur.decimal_input = self.cur.decimal_input.clone().with_value(value);
    }

    fn update_hex_input(&mut self, value: String) {
        self.cur.hex_input = self.cur.hex_input.clone().with_value(value);
    }

    fn update_hex_case(&mut self) {
        let value = self.cur.hex_input.value();
        match self.cur.case_sel {
            CaseItemSelect::Lowercase => {
                self.cur.hex_input = self.cur.hex_input.clone().with_value(value.to_lowercase());
            }
            CaseItemSelect::Uppercase => {
                self.cur.hex_input = self.cur.hex_input.clone().with_value(value.to_uppercase());
            }
        }
    }

    fn render_input(&self, f: &mut Frame, area: Rect, input: &Input, item: PageItems) {
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
                .title(item.str())
                .padding(Padding::horizontal(1)),
        );
        f.render_widget(input_widget, area);

        if self.cur.edit && self.cur.item == item {
            let visual_cursor = input.visual_cursor() as u16;
            let x = area.x + 2 + visual_cursor.min(input_max_width);
            let y = area.y + 1;
            f.set_cursor(x, y);
        }
    }

    fn render_status(&self, f: &mut Frame, area: Rect, status: &str) {
        let status_style = Style::default().fg(Color::Red);
        let status = Paragraph::new(status).block(
            Block::default()
                .borders(Borders::empty())
                .style(status_style)
                .padding(Padding::horizontal(1)),
        );
        f.render_widget(status, area);
    }
}
