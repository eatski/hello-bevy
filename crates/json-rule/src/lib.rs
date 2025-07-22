// Rule system crate - JSON rule loading and conversion

pub mod rule_loader;

// Re-export public types  
pub use token_input::{RuleSet, StructuredTokenInput};
pub use rule_loader::{load_rules_from_file, parse_rules_from_json};