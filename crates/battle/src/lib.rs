// Battle core crate - battle logic and management

pub mod battle;
pub mod battle_events;

// Re-export public types
pub use action_system::{Character, RuleNode};
pub use battle::Battle;
pub use battle_events::BattleEvent;