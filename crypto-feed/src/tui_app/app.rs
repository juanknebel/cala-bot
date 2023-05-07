use std::collections::HashMap;

use log::{debug, warn};

use crate::{
  binance::binance::Binance,
  tui_app::{
    actions::{Action, Actions},
    inputs::key::Key,
    state::AppState,
  },
};

pub struct App {
  /// Contextual actions
  actions: Actions,
  /// State
  state: AppState,
  /// Binane exchange
  binance: Binance,
  /// Content of every action
  content: HashMap<Action, String>,
  line_separator: String,
}

impl App {
  #[allow(clippy::new_without_default)]
  pub fn new() -> Self {
    let actions =
      vec![Action::ListSymbols, Action::StartStream, Action::Quit].into();
    let state = AppState::initialized();

    Self {
      actions,
      state,
      binance: Binance::new(),
      content: HashMap::new(),
      line_separator: "|".to_string(),
    }
  }

  /// Handle a user action
  pub async fn do_action(&mut self, key: Key) -> AppReturn {
    if let Some(action) = self.actions.find(key) {
      debug!("Run action [{:?}]", action);
      match action {
        Action::Quit => AppReturn::Exit,
        Action::ListSymbols => {
          let request_content = self
            .binance
            .list_symbols()
            .await
            .join(self.line_separator.as_str())
            .to_string();
          self.content.insert(Action::ListSymbols, request_content);
          AppReturn::Continue
        },
        Action::StartStream => AppReturn::Continue,
      }
    } else {
      warn!("No action accociated to {}", key);
      AppReturn::Continue
    }
  }

  pub fn request_content(&self) -> String {
    match self.content.get(&Action::ListSymbols) {
      Some(s) => s.to_string(),
      None => String::default(),
    }
  }

  pub fn line_separator(&self) -> String {
    self.line_separator.to_string()
  }

  /// We could update the app or dispatch event on tick
  pub fn update_on_tick(&mut self) -> AppReturn {
    // here we just increment a counter
    self.state.incr_tick();
    AppReturn::Continue
  }

  pub fn actions(&self) -> &Actions {
    &self.actions
  }

  pub fn state(&self) -> &AppState {
    &self.state
  }
}

#[derive(Debug, PartialEq, Eq)]
pub enum AppReturn {
  Exit,
  Continue,
}
