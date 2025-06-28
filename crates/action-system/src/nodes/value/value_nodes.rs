// Value nodes - nodes that evaluate to numeric values for calculations

// Trait for nodes that evaluate to numeric values
pub trait ValueNode: Send + Sync + std::fmt::Debug {
    fn evaluate(&self, battle_context: &crate::BattleContext, rng: &mut dyn rand::RngCore) -> i32;
}

impl ValueNode for Box<dyn ValueNode> {
    fn evaluate(&self, battle_context: &crate::BattleContext, rng: &mut dyn rand::RngCore) -> i32 {
        (**self).evaluate(battle_context, rng)
    }
}

// Re-export individual value node modules
pub use super::constant_value_node::ConstantValueNode;
pub use crate::nodes::character::character_hp_from_node::CharacterHpFromNode;