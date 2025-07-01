// Condition nodes - nodes that evaluate to true/false for decision making

// Trait for nodes that evaluate to boolean conditions
pub trait ConditionNode: Send + Sync + std::fmt::Debug {
    fn evaluate(&self, eval_context: &crate::nodes::evaluation_context::EvaluationContext, rng: &mut dyn rand::RngCore) -> crate::core::NodeResult<bool>;
}

impl ConditionNode for Box<dyn ConditionNode> {
    fn evaluate(&self, eval_context: &crate::nodes::evaluation_context::EvaluationContext, rng: &mut dyn rand::RngCore) -> crate::core::NodeResult<bool> {
        (**self).evaluate(eval_context, rng)
    }
}

// Re-export individual condition node modules
pub use super::random_condition_node::RandomConditionNode;
pub use super::greater_than_condition_node::GreaterThanConditionNode;