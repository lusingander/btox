use std::sync::mpsc;

use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    Frame, Terminal,
};

use crate::{
    key_code, key_code_char,
    msg::Msg,
    panes::{list::ListPane, pane::Pane, tool::ToolPane},
};

#[derive(Debug, Clone, Copy)]
enum PaneType {
    List,
    Tool,
}

pub struct App {
    quit: bool,
    focused: PaneType,
    list_pane: ListPane,
    tool_pane: ToolPane,
}

impl App {
    pub fn new() -> App {
        App {
            quit: false,
            focused: PaneType::List,
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
            _ => {
                let list_msg = self.list_pane.update(msg);
                let tool_msg = self.tool_pane.update(msg);
                return first_some(&[list_msg, tool_msg]);
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
        let chunks = Layout::new(
            Direction::Horizontal,
            [Constraint::Length(20), Constraint::Min(0)],
        )
        .split(f.size());

        self.list_pane.render(f.buffer_mut(), chunks[0]);
        self.tool_pane.render(f.buffer_mut(), chunks[1]);
    }

    fn resize(&mut self, w: u16, h: u16) {
        let _ = (w, h);
    }
}

fn first_some<T: Copy>(opts: &[Option<T>]) -> Option<T> {
    opts.iter().copied().flatten().next()
}
