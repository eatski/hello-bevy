// CountArrayNode - counts elements in character arrays

use crate::core::NodeResult;
use crate::nodes::evaluation_context::EvaluationContext;
use crate::nodes::unified_node::Node;
use crate::Character;

pub struct CountArrayNode {
    array_node: Box<dyn Node<Vec<Character>>>,
}

impl CountArrayNode {
    pub fn new(array_node: Box<dyn Node<Vec<Character>>>) -> Self {
        Self { array_node }
    }
}

impl Node<i32> for CountArrayNode {
    fn evaluate(&self, eval_context: &EvaluationContext, rng: &mut dyn rand::RngCore) -> NodeResult<i32> {
        let characters = self.array_node.evaluate(eval_context, rng)?;
        Ok(characters.len() as i32)
    }
}