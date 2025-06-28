// Value nodes - nodes that evaluate to numeric values for calculations

// Trait for nodes that evaluate to numeric values
pub trait ValueNode: Send + Sync + std::fmt::Debug {
    fn evaluate(&self, character: &crate::Character, rng: &mut dyn rand::RngCore) -> i32;
}

impl ValueNode for Box<dyn ValueNode> {
    fn evaluate(&self, character: &crate::Character, rng: &mut dyn rand::RngCore) -> i32 {
        (**self).evaluate(character, rng)
    }
}

// Re-export individual value node modules
pub use crate::constant_value_node::ConstantValueNode;
pub use crate::character_hp_value_node::CharacterHpValueNode;
pub use crate::character_hp_from_node::CharacterHpFromNode;