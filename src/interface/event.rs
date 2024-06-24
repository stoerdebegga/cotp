use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

use crossterm::event::{self, Event as CrosstermEvent, KeyEvent, KeyEventKind};

use crate::interface::app::AppResult;

/// Terminal events.
#[derive(Clone, Debug)]
pub enum Event {
    /// Terminal tick.
    Tick,
    /// Key press.
    Key(KeyEvent),
    /// Mouse click/scroll.
    Mouse(()),
    /// Terminal resize.
    Resize((), ()),
    /// Focus gained
    FocusGained(),
    /// Focus lost
    FocusLost(),
    /// Paste text
    Paste(()),
}

/// Terminal event handler.
#[derive(Debug)]
pub struct EventHandler {
    // Event sender channel.
    //sender: mpsc::Sender<Event>,
    /// Event receiver channel.
    receiver: mpsc::Receiver<Event>,
    // Event handler thread.
    //handler: thread::JoinHandle<()>,
}

impl EventHandler {
    /// Constructs a new instance of [`EventHandler`].
    pub fn new(tick_rate: u64) -> Self {
        let tick_rate = Duration::from_millis(tick_rate);
        let (sender, receiver) = mpsc::channel();
        thread::spawn(move || {
            let mut last_tick = Instant::now();
            loop {
                let timeout = tick_rate
                    .checked_sub(last_tick.elapsed())
                    .unwrap_or(tick_rate);

                if event::poll(timeout).expect("no events available") {
                    match event::read().expect("unable to read event") {
                        CrosstermEvent::Key(e) => {
                            // Workaround to fix double input on Windows
                            // Please check https://github.com/crossterm-rs/crossterm/issues/752
                            if e.kind == KeyEventKind::Press {
                                sender.send(Event::Key(e))
                            } else {
                                Ok(())
                            }
                        }
                        CrosstermEvent::Mouse(_e) => sender.send(Event::Mouse(())),
                        CrosstermEvent::Resize(_w, _h) => sender.send(Event::Resize((), ())),
                        CrosstermEvent::FocusGained => sender.send(Event::FocusGained()),
                        CrosstermEvent::FocusLost => sender.send(Event::FocusLost()),
                        CrosstermEvent::Paste(_e) => sender.send(Event::Paste(())),
                    }
                    .expect("failed to send terminal event")
                }

                if last_tick.elapsed() >= tick_rate {
                    sender.send(Event::Tick).expect("failed to send tick event");
                    last_tick = Instant::now();
                }
            }
        });

        Self { receiver }
    }

    /// Receive the next event from the handler thread.
    ///
    /// This function will always block the current thread if
    /// there is no data available and it's possible for more data to be sent.
    pub fn next(&self) -> AppResult<Event> {
        Ok(self.receiver.recv()?)
    }
}
