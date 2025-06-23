// Action system module - token-based action resolution system

pub mod core;
pub mod actions;
pub mod bool_tokens;
pub mod number_tokens;
pub mod system;

// Re-export public types for backward compatibility
pub use core::{ActionResolver, ActionType, RuleToken};
pub use actions::{CheckToken, StrikeAction, HealAction};
pub use bool_tokens::{BoolToken, TrueOrFalseRandomToken, GreaterThanToken};
pub use number_tokens::{NumberToken, ConstantToken, CharacterHPToken};
pub use system::ActionCalculationSystem;