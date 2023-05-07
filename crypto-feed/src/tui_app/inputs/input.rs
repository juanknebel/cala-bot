use crate::tui_app::inputs::key::Key;

pub enum InputEvent {
  /// An input event occurred.
  Input(Key),
  /// An tick event occurred.
  Tick,
}
