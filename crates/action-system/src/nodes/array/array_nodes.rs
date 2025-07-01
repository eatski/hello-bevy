// Array node traits - nodes that evaluate to collections of characters

use crate::core::NodeResult;

// Trait for nodes that evaluate to arrays of characters
pub trait CharacterArrayNode: Send + Sync + std::fmt::Debug {
    fn evaluate(&self, eval_context: &crate::nodes::evaluation_context::EvaluationContext, rng: &mut dyn rand::RngCore) -> NodeResult<Vec<crate::Character>>;
}

impl CharacterArrayNode for Box<dyn CharacterArrayNode> {
    fn evaluate(&self, eval_context: &crate::nodes::evaluation_context::EvaluationContext, rng: &mut dyn rand::RngCore) -> NodeResult<Vec<crate::Character>> {
        (**self).evaluate(eval_context, rng)
    }
}