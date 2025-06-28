// Condition nodes - nodes that evaluate to true/false for decision making

// Trait for nodes that evaluate to boolean conditions
pub trait ConditionNode: Send + Sync + std::fmt::Debug {
    fn evaluate(&self, character: &crate::Character, rng: &mut dyn rand::RngCore) -> bool;
}

impl ConditionNode for Box<dyn ConditionNode> {
    fn evaluate(&self, character: &crate::Character, rng: &mut dyn rand::RngCore) -> bool {
        (**self).evaluate(character, rng)
    }
}

// Re-export individual condition node modules
pub use crate::random_condition_node::RandomConditionNode;
pub use crate::greater_than_condition_node::GreaterThanConditionNode;