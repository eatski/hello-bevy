// Action system crate - node-based action resolution system

pub mod core;
pub mod nodes;
pub mod system;

// Re-export essential types only
pub use core::{Character, Team, TeamSide, ActionResolver, ActionType, RuleNode, NodeError, NodeResult};
pub use nodes::condition::{ConditionNode, ConditionCheckNode, RandomConditionNode, GreaterThanConditionNode};
pub use nodes::value::{ValueNode, ConstantValueNode};
pub use nodes::character::{BattleContext, CharacterNode, ActingCharacterNode, RandomCharacterNode, CharacterHpFromNode};
pub use nodes::action::{StrikeActionNode, HealActionNode};
pub use system::ActionCalculationSystem;

