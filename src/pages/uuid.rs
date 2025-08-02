use itsuki::zero_indexed_enum;
use ratatui::{crossterm::event::KeyCode, layout::Rect, text::Line, Frame};
use ratatui_macros::vertical;
use uuid::Uuid;

use crate::{
    fn_next_prev_mut, fn_str_map, key_code, key_code_char,
    msg::{Msg, PageMsg, UuidMsg},
    pages::{page::Page, util},
    widget::{
        scroll::{ScrollOutput, ScrollOutputState},
        select::Select,
    },
};

const COUNT_MAX: usize = 100;

pub struct UuidPage {
    focused: bool,
    cur: CurrentStatus,

    ids: Vec<Uuid>,
}

#[derive(Default)]
struct CurrentStatus {
    item: PageItems,
    hyphen_sel: HyphenItemSelect,
    case_sel: CaseItemSelect,
    ver_sel: VersionItemSelect,
    count: usize,
    output_state: ScrollOutputState,
}

impl UuidPage {
    pub fn new(focused: bool) -> UuidPage {
        UuidPage {
            focused,
            cur: CurrentStatus {
                count: 1,
                ..Default::default()
            },
            ids: Vec::new(),
        }
    }
}

#[derive(Default)]
#[zero_indexed_enum]
enum PageItems {
    #[default]
    Hyphen,
    Case,
    Version,
    Count,
    Output,
}

#[derive(Default)]
#[zero_indexed_enum]
enum HyphenItemSelect {
    #[default]
    WithHyphen,
    WithoutHyphen,
}

impl HyphenItemSelect {
    fn_str_map! {
        HyphenItemSelect::WithHyphen => "With hyphen",
        HyphenItemSelect::WithoutHyphen => "Without hyphen",
    }

    fn_next_prev_mut! {}
}

#[derive(Default)]
#[zero_indexed_enum]
enum CaseItemSelect {
    #[default]
    Lowercase,
    Uppercase,
}

impl CaseItemSelect {
    fn_str_map! {
        CaseItemSelect::Lowercase => "Lowercase",
        CaseItemSelect::Uppercase => "Uppercase",
    }

    fn_next_prev_mut! {}
}

#[derive(Default)]
#[zero_indexed_enum]
enum VersionItemSelect {
    #[default]
    V4,
}

impl VersionItemSelect {
    fn_str_map! {
        VersionItemSelect::V4 => "Version 4",
    }

    fn_next_prev_mut! {}
}

impl Page for UuidPage {
    fn handle_key(&self, key: ratatui::crossterm::event::KeyEvent) -> Option<Msg> {
        let msg = match key {
            key_code_char!('j') | key_code!(KeyCode::Down) => UuidMsg::SelectNextItem,
            key_code_char!('k') | key_code!(KeyCode::Up) => UuidMsg::SelectPrevItem,
            key_code_char!('l') | key_code!(KeyCode::Right) => UuidMsg::CurrentItemSelectNext,
            key_code_char!('h') | key_code!(KeyCode::Left) => UuidMsg::CurrentItemSelectPrev,
            key_code_char!('e', Ctrl) => UuidMsg::ScrollDown,
            key_code_char!('y', Ctrl) => UuidMsg::ScrollUp,
            key_code_char!('y') => UuidMsg::Copy,
            key_code_char!('p') => UuidMsg::Paste,
            key_code!(KeyCode::Enter) => UuidMsg::Generate,
            _ => return None,
        };
        Some(Msg::Page(PageMsg::Uuid(msg)))
    }

    fn update(&mut self, msg: PageMsg) -> Option<Msg> {
        if let PageMsg::Uuid(msg) = msg {
            match msg {
                UuidMsg::SelectNextItem => {
                    self.select_next_item();
                }
                UuidMsg::SelectPrevItem => {
                    self.select_prev_item();
                }
                UuidMsg::CurrentItemSelectNext => {
                    self.current_item_select_next();
                }
                UuidMsg::CurrentItemSelectPrev => {
                    self.current_item_select_prev();
                }
                UuidMsg::ScrollDown => {
                    self.scroll_down();
                }
                UuidMsg::ScrollUp => {
                    self.scroll_up();
                }
                UuidMsg::Generate => {
                    self.generate_uuid();
                }
                UuidMsg::Copy => {
                    return self.copy_to_clipboard();
                }
                UuidMsg::Paste => {
                    return self.paste_from_clipboard();
                }
            }
        }
        None
    }

    fn render(&mut self, f: &mut Frame, area: Rect) {
        let chunks = vertical![==2, ==2, ==2, ==2, >=0].split(area);

        let hyphen_sel = Select::new(
            HyphenItemSelect::strings_vec(),
            self.cur.hyphen_sel.val(),
            self.cur.item == PageItems::Hyphen,
            self.focused,
        );
        f.render_widget(hyphen_sel, chunks[0]);

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
            (1..=COUNT_MAX).map(|i| format!("{i}")).collect(),
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
        helps.push("<j/k> Select item");
        if !matches!(self.cur.item, PageItems::Output) {
            helps.push("<h/l> Select current item value");
        }
        helps.push("<Enter> Generate uuid");
        if matches!(self.cur.item, PageItems::Output) {
            helps.push("<C-e/C-y> Scroll down/up");
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
            PageItems::Hyphen => {
                self.cur.hyphen_sel.next_mut();
            }
            PageItems::Case => {
                self.cur.case_sel.next_mut();
            }
            PageItems::Version => {
                self.cur.ver_sel.next_mut();
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
            PageItems::Hyphen => {
                self.cur.hyphen_sel.prev_mut();
            }
            PageItems::Case => {
                self.cur.case_sel.prev_mut();
            }
            PageItems::Version => {
                self.cur.ver_sel.prev_mut();
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
        if !matches!(self.cur.item, PageItems::Output) || self.ids.is_empty() {
            return;
        }
        self.cur.output_state.scroll_down();
    }

    fn scroll_up(&mut self) {
        if !matches!(self.cur.item, PageItems::Output) || self.ids.is_empty() {
            return;
        }
        self.cur.output_state.scroll_up();
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
            let msg = format!("Could not parse {failure_count} lines of string to UUID");
            Some(Msg::NotifyWarn(msg))
        } else {
            None
        }
    }

    fn format_uuid(&self, id: &Uuid) -> String {
        let mut buf = Uuid::encode_buffer();
        let s = match (self.cur.hyphen_sel, self.cur.case_sel) {
            (HyphenItemSelect::WithHyphen, CaseItemSelect::Lowercase) => {
                id.hyphenated().encode_lower(&mut buf)
            }
            (HyphenItemSelect::WithHyphen, CaseItemSelect::Uppercase) => {
                id.hyphenated().encode_upper(&mut buf)
            }
            (HyphenItemSelect::WithoutHyphen, CaseItemSelect::Lowercase) => {
                id.simple().encode_lower(&mut buf)
            }
            (HyphenItemSelect::WithoutHyphen, CaseItemSelect::Uppercase) => {
                id.simple().encode_upper(&mut buf)
            }
        };
        s.to_string()
    }

    fn render_output(&mut self, f: &mut Frame, area: Rect) {
        let lines: Vec<Line> = self
            .ids
            .iter()
            .map(|id| Line::raw(self.format_uuid(id)))
            .collect();
        let output = ScrollOutput::new(lines, self.focused, self.cur.item == PageItems::Output);
        f.render_stateful_widget(output, area, &mut self.cur.output_state);
    }
}
