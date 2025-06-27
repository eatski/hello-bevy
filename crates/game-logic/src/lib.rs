// Battle core crate - battle logic and management

pub mod battle;

// Re-export public types
pub use combat_engine::{Character, RuleNode};
pub use battle::Battle;