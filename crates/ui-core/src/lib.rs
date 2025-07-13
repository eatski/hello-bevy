// UI Core crate - Bevy-independent UI logic and state management

pub mod game_state;
pub mod rule_management;
pub mod battle_logic;

#[cfg(test)]
mod integration_tests;

// Re-export public types
pub use game_state::{GameState, GameMode};
pub use rule_management::CurrentRules;
pub use token_input::{FlatTokenInput, convert_flat_rules_to_nodes};
pub use battle_logic::BattleOrchestrator;