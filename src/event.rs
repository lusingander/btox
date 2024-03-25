use std::{sync::mpsc, thread};

pub enum Event {
    Key(crossterm::event::KeyEvent),
    Resize(usize, usize),
    Error(std::io::Error),
}

pub fn new() -> (mpsc::Sender<Event>, mpsc::Receiver<Event>) {
    let (tx, rx) = mpsc::channel();

    let event_tx = tx.clone();
    thread::spawn(move || loop {
        match crossterm::event::read() {
            Ok(e) => match e {
                crossterm::event::Event::Key(key) => {
                    event_tx.send(Event::Key(key)).unwrap();
                }
                crossterm::event::Event::Resize(w, h) => {
                    let w = w as usize;
                    let h = h as usize;
                    event_tx.send(Event::Resize(w, h)).unwrap();
                }
                _ => {}
            },
            Err(e) => {
                event_tx.send(Event::Error(e)).unwrap();
            }
        }
    });

    (tx, rx)
}
