// Element character node - returns the current character being processed in array operations
use crate::core::NodeResult;
use crate::nodes::unified_node::Node;
use crate::nodes::evaluation_context::EvaluationContext;

/// Node that returns the current character being processed in array operations
/// This is typically used within FilterList conditions to reference the element being evaluated
#[derive(Debug)]
pub struct ElementCharacterNode;

impl ElementCharacterNode {
    pub fn new() -> Self {
        Self
    }
}

impl Default for ElementCharacterNode {
    fn default() -> Self {
        Self::new()
    }
}

impl Node<crate::Character> for ElementCharacterNode {
    fn evaluate(&self, context: &EvaluationContext, _rng: &mut dyn rand::RngCore) -> NodeResult<crate::Character> {
        Ok(context.get_current_character().clone())
    }
}