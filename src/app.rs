use std::sync::mpsc;

use crossterm::event::{Event, KeyCode};
use itsuki::zero_indexed_enum;
use ratatui::{
    backend::Backend,
    buffer::Buffer,
    layout::{Constraint, Layout, Margin, Rect},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::Widget,
    Frame, Terminal,
};

use crate::{
    key_code, key_code_char,
    msg::Msg,
    panes::{list::ListPane, pane::Pane, tool::ToolPane},
};

zero_indexed_enum! {
    PaneType => [
        List,
        Tool,
    ]
}

enum Notification {
    None,
    Info(String),
    Warn(String),
    Error(String),
}

pub struct App {
    quit: bool,
    focused: PaneType,
    notification: Notification,
    list_pane: ListPane,
    tool_pane: ToolPane,
}

impl App {
    pub fn new() -> App {
        App {
            quit: false,
            focused: PaneType::List,
            notification: Notification::None,
            list_pane: ListPane::new(true),
            tool_pane: ToolPane::new(false),
        }
    }

    pub fn start<B: Backend>(
        &mut self,
        terminal: &mut Terminal<B>,
        rx: mpsc::Receiver<Event>,
    ) -> std::io::Result<()> {
        while !self.quit {
            terminal.draw(|f| self.render(f))?;

            match rx.recv().unwrap() {
                Event::Key(key) => {
                    self.notification = Notification::None;

                    let mut current_msg = self.handle_key(key);
                    while let Some(msg) = current_msg {
                        current_msg = self.update(msg);
                    }
                }
                Event::Resize(w, h) => self.resize(w, h),
                _ => {}
            }
        }
        Ok(())
    }

    fn handle_key(&self, key: crossterm::event::KeyEvent) -> Option<Msg> {
        match key {
            key_code!(KeyCode::Esc) | key_code_char!('c', Ctrl) => Some(Msg::Quit),
            key_code!(KeyCode::Tab) => Some(Msg::SwitchPane),
            key_code_char!('?') => Some(Msg::ToggleHelp),
            _ => match self.focused {
                PaneType::List => self.list_pane.handle_key(key),
                PaneType::Tool => self.tool_pane.handle_key(key),
            },
        }
    }

    fn update(&mut self, msg: Msg) -> Option<Msg> {
        match msg {
            Msg::Quit => {
                self.quit_app();
            }
            Msg::SwitchPane => {
                self.switch_pane();
            }
            Msg::NotifyInfo(msg) => {
                self.notification = Notification::Info(msg);
            }
            Msg::NotifyWarn(msg) => {
                self.notification = Notification::Warn(msg);
            }
            Msg::NotifyError(msg) => {
                self.notification = Notification::Error(msg);
            }
            _ => {
                let list_msg = self.list_pane.update(msg.clone());
                let tool_msg = self.tool_pane.update(msg.clone());
                return first_some(vec![list_msg, tool_msg]);
            }
        }
        None
    }

    fn quit_app(&mut self) {
        self.quit = true;
    }

    fn switch_pane(&mut self) {
        self.list_pane.unfocus();
        self.tool_pane.unfocus();

        match self.focused {
            PaneType::List => {
                self.focused = PaneType::Tool;
                self.tool_pane.focus();
            }
            PaneType::Tool => {
                self.focused = PaneType::List;
                self.list_pane.focus();
            }
        }
    }

    fn render(&self, f: &mut Frame) {
        let chunks = Layout::vertical([Constraint::Min(0), Constraint::Length(1)]).split(f.size());

        self.render_panes(f.buffer_mut(), chunks[0]);
        self.render_notification(f.buffer_mut(), chunks[1]);
    }

    fn render_panes(&self, buf: &mut Buffer, area: Rect) {
        let chunks = Layout::horizontal([Constraint::Length(20), Constraint::Min(0)]).split(area);

        self.list_pane.render(buf, chunks[0]);
        self.tool_pane.render(buf, chunks[1]);
    }

    fn render_notification(&self, buf: &mut Buffer, area: Rect) {
        let area = area.inner(&Margin::new(1, 0));
        let style = Style::default().add_modifier(Modifier::BOLD);
        match &self.notification {
            Notification::Info(msg) => {
                Line::styled(msg, style.fg(Color::Green)).render(area, buf);
            }
            Notification::Warn(msg) => {
                Line::styled(msg, style.fg(Color::Yellow)).render(area, buf);
            }
            Notification::Error(msg) => {
                Line::styled(msg, style.fg(Color::Red)).render(area, buf);
            }
            Notification::None => {}
        };
    }

    fn resize(&mut self, w: u16, h: u16) {
        let _ = (w, h);
    }
}

fn first_some<T>(opts: Vec<Option<T>>) -> Option<T> {
    opts.into_iter().find(|opt| opt.is_some()).flatten()
}
