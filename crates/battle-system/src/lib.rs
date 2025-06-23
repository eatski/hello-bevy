// Battle system crate - game logic and rule management

pub mod battle;
pub mod rule_loader;
pub mod rule_input_model;

// Re-export public types
pub use action_system::Character;
pub use battle::Battle;
pub use rule_loader::{load_rules_from_file, parse_rules_from_json, convert_to_token_rules};
pub use rule_input_model::{RuleSet, RuleChain, TokenConfig};