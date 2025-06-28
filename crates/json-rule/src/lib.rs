// Rule system crate - JSON rule loading and conversion

pub mod rule_input_model;
pub mod rule_loader;

// Re-export public types
pub use rule_input_model::{RuleSet, RuleChain, JsonTokenInput};
pub use rule_loader::{load_rules_from_file, parse_rules_from_json, convert_to_node_rules};