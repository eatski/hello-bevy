// Condition nodes - nodes that evaluate to true/false for decision making

// Trait for nodes that evaluate to boolean conditions
pub trait ConditionNode: Send + Sync + std::fmt::Debug {
    fn evaluate(&self, battle_context: &crate::BattleContext, rng: &mut dyn rand::RngCore) -> bool;
}

impl ConditionNode for Box<dyn ConditionNode> {
    fn evaluate(&self, battle_context: &crate::BattleContext, rng: &mut dyn rand::RngCore) -> bool {
        (**self).evaluate(battle_context, rng)
    }
}

// Re-export individual condition node modules
pub use super::random_condition_node::RandomConditionNode;
pub use super::greater_than_condition_node::GreaterThanConditionNode;