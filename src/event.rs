use std::{sync::mpsc, thread};

use ratatui::crossterm::event::Event;

pub fn new() -> (mpsc::Sender<Event>, mpsc::Receiver<Event>) {
    let (tx, rx) = mpsc::channel();

    let event_tx = tx.clone();
    thread::spawn(move || loop {
        let e = ratatui::crossterm::event::read().unwrap();
        event_tx.send(e).unwrap();
    });

    (tx, rx)
}
