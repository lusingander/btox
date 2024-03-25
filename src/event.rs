use std::{sync::mpsc, thread};

use crossterm::event::Event;

pub fn new() -> (mpsc::Sender<Event>, mpsc::Receiver<Event>) {
    let (tx, rx) = mpsc::channel();

    let event_tx = tx.clone();
    thread::spawn(move || loop {
        let e = crossterm::event::read().unwrap();
        event_tx.send(e).unwrap();
    });

    (tx, rx)
}
