// UI Core crate - Bevy-independent UI logic and state management

pub mod game_state;
pub mod rule_management;
pub mod token_converter;

#[cfg(test)]
mod integration_tests;

// Re-export public types
pub use game_state::{GameState, GameMode};
pub use rule_management::CurrentRules;
pub use token_converter::{UITokenType, convert_ui_rules_to_nodes};