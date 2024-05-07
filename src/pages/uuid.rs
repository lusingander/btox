use crossterm::event::KeyCode;
use itsuki::zero_indexed_enum;
use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::Line,
    widgets::{Block, Padding, Paragraph},
    Frame,
};
use ratatui_macros::vertical;
use uuid::Uuid;

use crate::{
    key_code, key_code_char,
    msg::Msg,
    pages::{page::Page, util},
    widget::{scroll::ScrollBar, select::Select},
};

const COUNT_MAX: usize = 100;

pub struct UuidPage {
    focused: bool,
    cur: CurrentStatus,

    ids: Vec<Uuid>,
}

struct CurrentStatus {
    item: PageItems,
    dash_sel: DashItemSelect,
    case_sel: CaseItemSelect,
    ver_sel: VersionItemSelect,
    count: usize,
    output_offset: usize,
}

impl UuidPage {
    pub fn new(focused: bool) -> UuidPage {
        UuidPage {
            focused,
            cur: CurrentStatus {
                item: PageItems::Dash,
                dash_sel: DashItemSelect::WithDash,
                case_sel: CaseItemSelect::Lowercase,
                ver_sel: VersionItemSelect::V4,
                count: 1,
                output_offset: 0,
            },
            ids: Vec::new(),
        }
    }
}

zero_indexed_enum! {
    PageItems => [Dash, Case, Version, Count, Output]
}

zero_indexed_enum! {
    DashItemSelect => [WithDash, WithoutDash]
}

impl DashItemSelect {
    fn str(&self) -> &str {
        match self {
            DashItemSelect::WithDash => "With dash",
            DashItemSelect::WithoutDash => "Without dash",
        }
    }

    fn strings_vec() -> Vec<String> {
        Self::vars_vec().iter().map(|s| s.str().into()).collect()
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

zero_indexed_enum! {
    VersionItemSelect => [V4]
}

impl VersionItemSelect {
    fn str(&self) -> &str {
        match self {
            VersionItemSelect::V4 => "Version 4",
        }
    }

    fn strings_vec() -> Vec<String> {
        Self::vars_vec().iter().map(|s| s.str().into()).collect()
    }
}

impl Page for UuidPage {
    fn handle_key(&self, key: crossterm::event::KeyEvent) -> Option<Msg> {
        match key {
            key_code!(KeyCode::Esc) => Some(Msg::Quit),
            key_code_char!('n', Ctrl) => Some(Msg::UuidPageSelectNextItem),
            key_code_char!('p', Ctrl) => Some(Msg::UuidPageSelectPrevItem),
            key_code_char!('l') | key_code!(KeyCode::Right) => {
                Some(Msg::UuidPageCurrentItemSelectNext)
            }
            key_code_char!('h') | key_code!(KeyCode::Left) => {
                Some(Msg::UuidPageCurrentItemSelectPrev)
            }
            key_code_char!('j') => Some(Msg::UuidPageScrollDown),
            key_code_char!('k') => Some(Msg::UuidPageScrollUp),
            key_code_char!('y') => Some(Msg::UuidPageCopy),
            key_code_char!('p') => Some(Msg::UuidPagePaste),
            key_code!(KeyCode::Enter) => Some(Msg::UuidPageGenerate),
            _ => None,
        }
    }

    fn update(&mut self, msg: Msg) -> Option<Msg> {
        match msg {
            Msg::UuidPageSelectNextItem => {
                self.select_next_item();
            }
            Msg::UuidPageSelectPrevItem => {
                self.select_prev_item();
            }
            Msg::UuidPageCurrentItemSelectNext => {
                self.current_item_select_next();
            }
            Msg::UuidPageCurrentItemSelectPrev => {
                self.current_item_select_prev();
            }
            Msg::UuidPageScrollDown => {
                self.scroll_down();
            }
            Msg::UuidPageScrollUp => {
                self.scroll_up();
            }
            Msg::UuidPageGenerate => {
                self.generate_uuid();
            }
            Msg::UuidPageCopy => {
                return self.copy_to_clipboard();
            }
            Msg::UuidPagePaste => {
                return self.paste_from_clipboard();
            }
            _ => {}
        }
        None
    }

    fn render(&mut self, f: &mut Frame, area: Rect) {
        let chunks = vertical![==2, ==2, ==2, ==2, >=0].split(area);

        let dash_sel = Select::new(
            DashItemSelect::strings_vec(),
            self.cur.dash_sel.val(),
            self.cur.item == PageItems::Dash,
            self.focused,
        );
        f.render_widget(dash_sel, chunks[0]);

        let case_sel = Select::new(
            CaseItemSelect::strings_vec(),
            self.cur.case_sel.val(),
            self.cur.item == PageItems::Case,
            self.focused,
        );
        f.render_widget(case_sel, chunks[1]);

        let version_sel = Select::new(
            VersionItemSelect::strings_vec(),
            self.cur.ver_sel.val(),
            self.cur.item == PageItems::Version,
            self.focused,
        );
        f.render_widget(version_sel, chunks[2]);

        let count_sel = Select::new(
            (1..=COUNT_MAX).map(|i| format!("{}", i)).collect(),
            self.cur.count - 1,
            self.cur.item == PageItems::Count,
            self.focused,
        );
        f.render_widget(count_sel, chunks[3]);

        self.render_output(f, chunks[4]);
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
        if !matches!(self.cur.item, PageItems::Output) {
            helps.push("<Left/Right> Select current item value");
        }
        helps.push("<Enter> Generate uuid");
        if matches!(self.cur.item, PageItems::Output) {
            helps.push("<y> Copy to clipboard");
            helps.push("<p> Paste from clipboard");
        }
        helps
    }
}

impl UuidPage {
    fn select_next_item(&mut self) {
        self.cur.item = self.cur.item.next();
    }

    fn select_prev_item(&mut self) {
        self.cur.item = self.cur.item.prev();
    }

    fn current_item_select_next(&mut self) {
        match self.cur.item {
            PageItems::Dash => {
                if self.cur.dash_sel.val() < DashItemSelect::len() - 1 {
                    self.cur.dash_sel = self.cur.dash_sel.next();
                }
            }
            PageItems::Case => {
                if self.cur.case_sel.val() < CaseItemSelect::len() - 1 {
                    self.cur.case_sel = self.cur.case_sel.next();
                }
            }
            PageItems::Version => {
                if self.cur.ver_sel.val() < VersionItemSelect::len() - 1 {
                    self.cur.ver_sel = self.cur.ver_sel.next();
                }
            }
            PageItems::Count => {
                if self.cur.count < COUNT_MAX {
                    self.cur.count += 1;
                }
            }
            PageItems::Output => {}
        }
    }

    fn current_item_select_prev(&mut self) {
        match self.cur.item {
            PageItems::Dash => {
                if self.cur.dash_sel.val() > 0 {
                    self.cur.dash_sel = self.cur.dash_sel.prev();
                }
            }
            PageItems::Case => {
                if self.cur.case_sel.val() > 0 {
                    self.cur.case_sel = self.cur.case_sel.prev();
                }
            }
            PageItems::Version => {
                if self.cur.ver_sel.val() > 0 {
                    self.cur.ver_sel = self.cur.ver_sel.prev();
                }
            }
            PageItems::Count => {
                if self.cur.count > 1 {
                    self.cur.count -= 1;
                }
            }
            PageItems::Output => {}
        }
    }

    fn scroll_down(&mut self) {
        if self.cur.output_offset < self.ids.len() - 1 {
            self.cur.output_offset += 1;
        }
    }

    fn scroll_up(&mut self) {
        if self.cur.output_offset > 0 {
            self.cur.output_offset -= 1;
        }
    }

    fn generate_uuid(&mut self) {
        self.ids = (0..self.cur.count).map(|_| Uuid::new_v4()).collect();
    }

    fn copy_to_clipboard(&self) -> Option<Msg> {
        if !matches!(self.cur.item, PageItems::Output) {
            return None;
        }

        let ids: Vec<String> = self.ids.iter().map(|id| self.format_uuid(id)).collect();
        let text = ids.join("\n");
        util::copy_to_clipboard(&text)
    }

    fn paste_from_clipboard(&mut self) -> Option<Msg> {
        if !matches!(self.cur.item, PageItems::Output) {
            return None;
        }

        let text = util::paste_from_clipboard().unwrap();
        let mut ids: Vec<Uuid> = Vec::new();
        let mut failure_count = 0;
        for s in text.lines() {
            if let Ok(id) = Uuid::parse_str(s) {
                ids.push(id);
            } else {
                failure_count += 1;
            }
        }
        self.ids = ids;

        if failure_count > 0 {
            let msg = format!("Could not parse {} lines of string to UUID", failure_count);
            Some(Msg::NotifyWarn(msg))
        } else {
            None
        }
    }

    fn format_uuid(&self, id: &Uuid) -> String {
        let mut buf = Uuid::encode_buffer();
        let s = match (self.cur.dash_sel, self.cur.case_sel) {
            (DashItemSelect::WithDash, CaseItemSelect::Lowercase) => {
                id.hyphenated().encode_lower(&mut buf)
            }
            (DashItemSelect::WithDash, CaseItemSelect::Uppercase) => {
                id.hyphenated().encode_upper(&mut buf)
            }
            (DashItemSelect::WithoutDash, CaseItemSelect::Lowercase) => {
                id.simple().encode_lower(&mut buf)
            }
            (DashItemSelect::WithoutDash, CaseItemSelect::Uppercase) => {
                id.simple().encode_upper(&mut buf)
            }
        };
        s.to_string()
    }

    fn render_output(&self, f: &mut Frame, area: Rect) {
        let output_style = if self.focused {
            if self.cur.item == PageItems::Output {
                Style::default().fg(Color::Blue)
            } else {
                Style::default().fg(Color::Reset)
            }
        } else {
            Style::default().fg(Color::DarkGray)
        };

        let max_output_count = area.height as usize - 2;
        let uuids: Vec<Line<'_>> = self
            .ids
            .iter()
            .skip(self.cur.output_offset)
            .take(max_output_count)
            .map(|id| Line::raw(self.format_uuid(id)))
            .collect();
        let output = Paragraph::new(uuids).block(
            Block::bordered()
                .style(output_style)
                .padding(Padding::horizontal(1)),
        );
        f.render_widget(output, area);

        if self.ids.len() > max_output_count {
            let scrollbar_area = Rect::new(area.right() - 2, area.top() + 1, 1, area.height - 2);
            let scrollbar = ScrollBar::new(self.ids.len(), self.cur.output_offset);
            f.render_widget(scrollbar, scrollbar_area);
        }
    }
}
