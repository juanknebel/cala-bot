use crate::tui_app::{
  app::{App, AppReturn},
  inputs::input::InputEvent,
  ui,
};
use eyre::Result;
use std::{cell::RefCell, io::stdout, rc::Rc, time::Duration};
use tui::{backend::CrosstermBackend, Terminal};

use crate::tui_app::inputs::events::Events;

pub async fn start_ui(app: Rc<RefCell<App>>) -> Result<()> {
  // Configure Crossterm backend for tui
  let stdout = stdout();
  crossterm::terminal::enable_raw_mode()?;
  let backend = CrosstermBackend::new(stdout);
  let mut terminal = Terminal::new(backend)?;
  terminal.clear()?;
  terminal.hide_cursor()?;

  // User event handler
  let tick_rate = Duration::from_millis(200);
  let events = Events::new(tick_rate);

  loop {
    let mut app = app.borrow_mut();

    // Render
    terminal.draw(|rect| ui::draw(rect, &app))?;

    // Handle inputs
    let result = match events.next()? {
      InputEvent::Input(key) => app.do_action(key).await,
      InputEvent::Tick => app.update_on_tick(),
    };
    // Check if we should exit
    if result == AppReturn::Exit {
      break;
    }
  }

  // Restore the terminal and close application
  terminal.clear()?;
  terminal.show_cursor()?;
  crossterm::terminal::disable_raw_mode()?;

  Ok(())
}
