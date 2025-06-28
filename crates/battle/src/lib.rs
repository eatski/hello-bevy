// Battle core crate - battle logic and management

pub mod battle;

// Re-export public types
pub use action_system::{Character, RuleNode};
pub use battle::Battle;