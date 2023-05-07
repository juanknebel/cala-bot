use std::{
  sync::mpsc::{channel, Receiver, RecvError, Sender},
  thread,
  time::Duration,
};

use crate::tui_app::inputs::{input::InputEvent, key::Key};

/// A small event handler that wrap crossterm input and tick event. Each event
/// type is handled in its own thread and returned to a common `Receiver`
pub struct Events {
  rx: Receiver<InputEvent>,
  // Need to be kept around to prevent disposing the sender side.
  _tx: Sender<InputEvent>,
}

impl Events {
  /// Constructs an new instance of `Events` with the default config.
  pub fn new(tick_rate: Duration) -> Events {
    let (tx, rx) = channel();

    let event_tx = tx.clone();
    thread::spawn(move || {
      loop {
        // poll for tick rate duration, if no event, sent tick event.
        if crossterm::event::poll(tick_rate).unwrap() {
          if let crossterm::event::Event::Key(key) =
            crossterm::event::read().unwrap()
          {
            let key = Key::from(key);
            event_tx.send(InputEvent::Input(key)).unwrap();
          }
        }
        event_tx.send(InputEvent::Tick).unwrap();
      }
    });

    Events {
      rx,
      _tx: tx,
    }
  }

  /// Attempts to read an event.
  /// This function will block the current thread.
  pub fn next(&self) -> Result<InputEvent, RecvError> {
    self.rx.recv()
  }
}
