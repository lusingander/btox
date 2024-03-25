use std::sync::mpsc;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    backend::Backend,
    widgets::{Block, Paragraph},
    Frame, Terminal,
};

use crate::{event::Event, key_code, key_code_char};

pub struct App {}

impl App {
    pub fn new() -> App {
        App {}
    }

    pub fn start<B: Backend>(
        &self,
        terminal: &mut Terminal<B>,
        rx: mpsc::Receiver<Event>,
    ) -> std::io::Result<()> {
        loop {
            terminal.draw(|f| self.render(f))?;

            match rx.recv().unwrap() {
                Event::Key(key) => {
                    if matches!(key, key_code!(KeyCode::Esc) | key_code_char!('c', Ctrl)) {
                        return Ok(());
                    }
                }
                Event::Resize(_, _) => todo!(),
                Event::Error(_) => todo!(),
            }
        }
    }

    fn render(&self, f: &mut Frame) {
        let p = Paragraph::new("Hello").block(Block::bordered().title("btox"));
        f.render_widget(p, f.size());
    }
}
