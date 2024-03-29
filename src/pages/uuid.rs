use crossterm::event::{KeyCode, KeyEvent};
use itsuki::zero_indexed_enum;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Paragraph, Widget},
};

use crate::{key_code_char, msg::Msg, pages::page::Page, widget::select::Select};

const COUNT_MAX: usize = 100;

pub struct UuidPage {
    focused: bool,
    cur: CurrentStatus,
}

struct CurrentStatus {
    item: PageItems,
    dash_sel: DashItemSelect,
    case_sel: CaseItemSelect,
    ver_sel: VersionItemSelect,
    count: usize,
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
            },
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
            key_code_char!('j') => Some(Msg::UuidPageSelectNextItem),
            key_code_char!('k') => Some(Msg::UuidPageSelectPrevItem),
            key_code_char!('l') => Some(Msg::UuidPageCurrentItemSelectNext),
            key_code_char!('h') => Some(Msg::UuidPageCurrentItemSelectPrev),
            _ => None,
        }
    }

    fn update(&mut self, msg: Msg) -> Option<Msg> {
        match msg {
            Msg::UuidPageSelectNextItem => {
                self.cur.item = self.cur.item.next();
            }
            Msg::UuidPageSelectPrevItem => {
                self.cur.item = self.cur.item.prev();
            }
            Msg::UuidPageCurrentItemSelectNext => match self.cur.item {
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
            },
            Msg::UuidPageCurrentItemSelectPrev => match self.cur.item {
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
            },
            _ => {}
        }
        None
    }

    fn render(&self, buf: &mut Buffer, area: Rect) {
        let chunks = Layout::new(
            Direction::Vertical,
            [
                Constraint::Length(2),
                Constraint::Length(2),
                Constraint::Length(2),
                Constraint::Length(2),
                Constraint::Min(0),
            ],
        )
        .split(area);

        let dash_sel = Select::new(
            DashItemSelect::strings_vec(),
            self.cur.dash_sel.val(),
            self.cur.item == PageItems::Dash,
            self.focused,
        );
        dash_sel.render(chunks[0], buf);

        let case_sel = Select::new(
            CaseItemSelect::strings_vec(),
            self.cur.case_sel.val(),
            self.cur.item == PageItems::Case,
            self.focused,
        );
        case_sel.render(chunks[1], buf);

        let version_sel = Select::new(
            VersionItemSelect::strings_vec(),
            self.cur.ver_sel.val(),
            self.cur.item == PageItems::Version,
            self.focused,
        );
        version_sel.render(chunks[2], buf);

        let count_sel = Select::new(
            (1..=COUNT_MAX).map(|i| format!("{}", i)).collect(),
            self.cur.count - 1,
            self.cur.item == PageItems::Count,
            self.focused,
        );
        count_sel.render(chunks[3], buf);

        let output_style = if self.focused {
            if self.cur.item == PageItems::Output {
                Style::default().fg(Color::Blue)
            } else {
                Style::default().fg(Color::Reset)
            }
        } else {
            Style::default().fg(Color::DarkGray)
        };

        let uuids = "";
        let output = Paragraph::new(uuids).block(Block::bordered().style(output_style));
        output.render(chunks[4], buf);
    }

    fn focus(&mut self) {
        self.focused = true;
    }

    fn unfocus(&mut self) {
        self.focused = false;
    }
}
