// CountArrayNode - counts elements in character arrays

use crate::core::NodeResult;
use crate::nodes::evaluation_context::EvaluationContext;
use crate::nodes::unified_node::{CoreNode as Node, BoxedNode};
use crate::Character;

pub struct CountArrayNode {
    array_node: BoxedNode<Vec<Character>>,
}

impl CountArrayNode {
    pub fn new(array_node: BoxedNode<Vec<Character>>) -> Self {
        Self { array_node }
    }
}

impl<'a> Node<i32, EvaluationContext<'a>> for CountArrayNode {
    fn evaluate(&self, eval_context: &mut EvaluationContext) -> NodeResult<i32> {
        let characters = self.array_node.evaluate(eval_context)?;
        Ok(characters.len() as i32)
    }
}