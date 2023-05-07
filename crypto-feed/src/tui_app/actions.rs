use std::{
  collections::HashMap,
  fmt::{self, Display},
  slice::Iter,
};

use crate::tui_app::inputs::key::Key;

/// We define all available action
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Action {
  ListSymbols,
  StartStream,
  Quit,
}

impl Action {
  /// All available actions
  pub fn iterator() -> Iter<'static, Action> {
    static ACTIONS: [Action; 3] =
      [Action::Quit, Action::ListSymbols, Action::StartStream];
    ACTIONS.iter()
  }

  /// List of key associated to action
  pub fn keys(&self) -> &[Key] {
    match self {
      Action::Quit => &[Key::Ctrl('c'), Key::Char('q')],
      Action::ListSymbols => &[Key::Char('1')],
      Action::StartStream => &[Key::Char('2')],
    }
  }
}

/// Could display a user friendly short description of action
impl Display for Action {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let str = match self {
      Action::Quit => "Quit",
      Action::ListSymbols => "List symbols",
      Action::StartStream => "Start stream",
    };
    write!(f, "{}", str)
  }
}

/// The application should have some contextual actions.
#[derive(Default, Debug, Clone)]
pub struct Actions(Vec<Action>);

impl Actions {
  /// Given a key, find the corresponding action
  pub fn find(&self, key: Key) -> Option<&Action> {
    // for a in Action::iterator() {
    //  dbg!(a);
    //}

    Action::iterator()
      .filter(|action| self.0.contains(action))
      .find(|action| action.keys().contains(&key))
  }

  /// Get contextual actions.
  /// (just for building a help view)
  pub fn actions(&self) -> &[Action] {
    self.0.as_slice()
  }
}

impl From<Vec<Action>> for Actions {
  /// Build contextual action
  ///
  /// # Panics
  ///
  /// If two actions have same key
  fn from(actions: Vec<Action>) -> Self {
    // Check key unicity
    let mut map: HashMap<Key, Vec<Action>> = HashMap::new();
    for action in actions.iter() {
      for key in action.keys().iter() {
        match map.get_mut(key) {
          Some(vec) => vec.push(*action),
          None => {
            map.insert(*key, vec![*action]);
          },
        }
      }
    }
    let errors = map
      .iter()
      .filter(|(_, actions)| actions.len() > 1) // at least two actions share same shortcut
      .map(|(key, actions)| {
        let actions = actions
          .iter()
          .map(Action::to_string)
          .collect::<Vec<_>>()
          .join(", ");
        format!("Conflict key {} with actions {}", key, actions)
      })
      .collect::<Vec<_>>();
    if !errors.is_empty() {
      panic!("{}", errors.join("; "))
    }

    // Ok, we can create contextual actions
    Self(actions)
  }
}
