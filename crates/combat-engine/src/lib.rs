// Action system crate - node-based action resolution system

pub mod character;
pub mod core;
pub mod condition_nodes;
pub mod value_nodes;
pub mod control_nodes;
pub mod action_nodes;
pub mod system;

// Re-export public types
pub use character::Character;
pub use core::{ActionResolver, ActionType, RuleNode};
pub use condition_nodes::{ConditionNode, RandomConditionNode, GreaterThanConditionNode};
pub use value_nodes::{ValueNode, ConstantValueNode, CharacterHpValueNode};
pub use control_nodes::ConditionCheckNode;
pub use action_nodes::{StrikeActionNode, HealActionNode};
pub use system::ActionCalculationSystem;