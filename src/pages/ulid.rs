use itsuki::zero_indexed_enum;
use ratatui::{crossterm::event::KeyCode, layout::Rect, text::Line, Frame};
use ratatui_macros::vertical;
use ulid::Ulid;

use crate::{
    fn_next_prev_mut, fn_str_map, key_code, key_code_char,
    msg::{Msg, PageMsg, UlidMsg},
    pages::{page::Page, util},
    widget::{
        scroll::{ScrollOutput, ScrollOutputState},
        select::Select,
    },
};

const COUNT_MAX: usize = 100;

pub struct UlidPage {
    focused: bool,
    cur: CurrentStatus,
    ids: Vec<Ulid>,
}

#[derive(Default)]
struct CurrentStatus {
    item: PageItems,
    case_sel: CaseItemSelect,
    count: usize,
    output_state: ScrollOutputState,
}

impl UlidPage {
    pub fn new(focused: bool) -> UlidPage {
        UlidPage {
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
    Case,
    Count,
    Output,
}

#[derive(Default)]
#[zero_indexed_enum]
enum CaseItemSelect {
    #[default]
    Uppercase,
    Lowercase,
}

impl CaseItemSelect {
    fn_str_map! {
        CaseItemSelect::Uppercase => "Uppercase",
        CaseItemSelect::Lowercase => "Lowercase",
    }
    fn_next_prev_mut! {}
}

impl Page for UlidPage {
    fn handle_key(&self, key: ratatui::crossterm::event::KeyEvent) -> Option<Msg> {
        let msg = match key {
            key_code_char!('j') | key_code!(KeyCode::Down) => UlidMsg::SelectNextItem,
            key_code_char!('k') | key_code!(KeyCode::Up) => UlidMsg::SelectPrevItem,
            key_code_char!('l') | key_code!(KeyCode::Right) => UlidMsg::CurrentItemSelectNext,
            key_code_char!('h') | key_code!(KeyCode::Left) => UlidMsg::CurrentItemSelectPrev,
            key_code_char!('e', Ctrl) => UlidMsg::ScrollDown,
            key_code_char!('y', Ctrl) => UlidMsg::ScrollUp,
            key_code_char!('y') => UlidMsg::Copy,
            key_code_char!('p') => UlidMsg::Paste,
            key_code!(KeyCode::Enter) => UlidMsg::Generate,
            _ => return None,
        };
        Some(Msg::Page(PageMsg::Ulid(msg)))
    }

    fn update(&mut self, msg: &PageMsg) -> Option<Msg> {
        if let PageMsg::Ulid(msg) = msg {
            match msg {
                UlidMsg::SelectNextItem => {
                    self.select_next_item();
                }
                UlidMsg::SelectPrevItem => {
                    self.select_prev_item();
                }
                UlidMsg::CurrentItemSelectNext => {
                    self.current_item_select_next();
                }
                UlidMsg::CurrentItemSelectPrev => {
                    self.current_item_select_prev();
                }
                UlidMsg::ScrollDown => {
                    self.scroll_down();
                }
                UlidMsg::ScrollUp => {
                    self.scroll_up();
                }
                UlidMsg::Generate => {
                    self.generate_ulid();
                }
                UlidMsg::Copy => {
                    return self.copy_to_clipboard();
                }
                UlidMsg::Paste => {
                    return self.paste_from_clipboard();
                }
            }
        }
        None
    }

    fn render(&mut self, f: &mut Frame, area: Rect) {
        let chunks = vertical![==2, ==2, >=0].split(area);

        let case_sel = Select::new(
            CaseItemSelect::strings_vec(),
            self.cur.case_sel.val(),
            self.cur.item == PageItems::Case,
            self.focused,
        );
        f.render_widget(case_sel, chunks[0]);

        let count_sel = Select::new(
            (1..=COUNT_MAX).map(|i| format!("{i}")).collect(),
            self.cur.count - 1,
            self.cur.item == PageItems::Count,
            self.focused,
        );
        f.render_widget(count_sel, chunks[1]);

        self.render_output(f, chunks[2]);
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
        helps.push("<Enter> Generate ulid");
        if matches!(self.cur.item, PageItems::Output) {
            helps.push("<C-e/C-y> Scroll down/up");
            helps.push("<y> Copy to clipboard");
            helps.push("<p> Paste from clipboard");
        }
        helps
    }
}

impl UlidPage {
    fn select_next_item(&mut self) {
        self.cur.item = self.cur.item.next();
    }

    fn select_prev_item(&mut self) {
        self.cur.item = self.cur.item.prev();
    }

    fn current_item_select_next(&mut self) {
        match self.cur.item {
            PageItems::Case => {
                self.cur.case_sel.next_mut();
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
            PageItems::Case => {
                self.cur.case_sel.prev_mut();
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

    fn generate_ulid(&mut self) {
        self.ids = (0..self.cur.count).map(|_| Ulid::new()).collect();
    }

    fn copy_to_clipboard(&self) -> Option<Msg> {
        if !matches!(self.cur.item, PageItems::Output) {
            return None;
        }

        let ids: Vec<String> = self.ids.iter().map(|id| self.format_ulid(id)).collect();
        let text = ids.join("\n");
        util::copy_to_clipboard(&text)
    }

    fn paste_from_clipboard(&mut self) -> Option<Msg> {
        if !matches!(self.cur.item, PageItems::Output) {
            return None;
        }

        let text = util::paste_from_clipboard().unwrap();
        let mut ids: Vec<Ulid> = Vec::new();
        let mut failure_count = 0;
        for s in text.lines() {
            if let Ok(id) = Ulid::from_string(s) {
                ids.push(id);
            } else {
                failure_count += 1;
            }
        }
        self.ids = ids;

        if failure_count > 0 {
            let msg = format!("Could not parse {failure_count} lines of string to ULID");
            Some(Msg::NotifyWarn(msg))
        } else {
            None
        }
    }

    fn format_ulid(&self, id: &Ulid) -> String {
        match self.cur.case_sel {
            CaseItemSelect::Uppercase => id.to_string(),
            CaseItemSelect::Lowercase => id.to_string().to_lowercase(),
        }
    }

    fn render_output(&mut self, f: &mut Frame, area: Rect) {
        let lines: Vec<Line> = self
            .ids
            .iter()
            .map(|id| Line::raw(self.format_ulid(id)))
            .collect();
        let output = ScrollOutput::new(lines, self.focused, self.cur.item == PageItems::Output);
        f.render_stateful_widget(output, area, &mut self.cur.output_state);
    }
}
