// Action system crate - node-based action resolution system

pub mod character;
pub mod core;
pub mod condition_nodes;
pub mod value_nodes;
pub mod character_nodes;
pub mod control_nodes;
pub mod action_nodes;
pub mod actions;
pub mod system;

// Individual struct modules
pub mod strike_action_node;
pub mod heal_action_node;
pub mod random_condition_node;
pub mod greater_than_condition_node;
pub mod condition_check_node;
pub mod constant_value_node;
pub mod character_hp_value_node;
pub mod acting_character_node;
pub mod random_character_node;
pub mod character_hp_from_node;

// Re-export public types
pub use character::Character;
pub use core::{ActionResolver, ActionType, RuleNode};
pub use condition_nodes::{ConditionNode, RandomConditionNode, GreaterThanConditionNode};
pub use value_nodes::{ValueNode, ConstantValueNode, CharacterHpValueNode};
pub use character_hp_from_node::CharacterHpFromNode;
pub use character_nodes::{CharacterNode, BattleContext, ActingCharacterNode, RandomCharacterNode};
pub use control_nodes::ConditionCheckNode;
pub use action_nodes::{StrikeActionNode, HealActionNode};
pub use actions::{StrikeActionNode as StrikeAction, HealActionNode as HealAction};
pub use system::ActionCalculationSystem;

