// UI crate - Bevy UI components and systems

pub mod ui;

// Re-export public types
pub use ui::{GameFont, GameBattle, load_font, setup_ui, handle_battle_input, update_battle_ui, update_log_ui, update_latest_log_ui};