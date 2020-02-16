use std::{
    io::{
        stdin,
    },
    thread,
    time::Duration,
    sync::mpsc,
};
use termion::{
    event::Key,
    input::TermRead,
};


pub enum Event<I> {
    Input(I),
    Tick,
}
pub struct Events {
    rx: mpsc::Receiver<Event<Key>>,
}

impl Default for Events {
    fn default() -> Self {
        let (tx, rx) = mpsc::channel();
        {
            let tx = tx.clone();
            thread::spawn(move || {
                let stdin = stdin();
                for evt in stdin.keys() {
                    if let Ok(key) = evt {
                        tx.send(Event::Input(key)).unwrap();
                    }
                }
            });
        };

        thread::spawn(move || {
            loop {
                tx.send(Event::Tick).unwrap();
                thread::sleep(Duration::from_millis(250));
            }
        });
        
        Events {
            rx,
        }
    }
}
impl Events {
    pub fn next(&self) -> Result<Event<Key>, mpsc::RecvError> {
        self.rx.recv()
    }
}
