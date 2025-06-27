// Action system crate - node-based action resolution system

pub mod character;
pub mod core;
pub mod actions;
pub mod bool_nodes;
pub mod number_nodes;
pub mod system;

// Re-export public types
pub use character::Character;
pub use core::{ActionResolver, ActionType, RuleNode};
pub use actions::{CheckNode, StrikeAction, HealAction};
pub use bool_nodes::{BoolNode, TrueOrFalseRandomNode, GreaterThanNode};
pub use number_nodes::{NumberNode, ConstantNode, CharacterHPNode};
pub use system::ActionCalculationSystem;