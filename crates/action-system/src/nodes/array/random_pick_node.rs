// RandomPickNode - selects a random character from character arrays

use super::CharacterArrayNode;
use crate::core::{NodeError, NodeResult};
use crate::nodes::character::CharacterNode;
use crate::nodes::evaluation_context::EvaluationContext;
use rand::Rng;

#[derive(Debug)]
pub struct RandomPickNode {
    array_node: Box<dyn CharacterArrayNode>,
}

impl RandomPickNode {
    pub fn new(array_node: Box<dyn CharacterArrayNode>) -> Self {
        Self { array_node }
    }
}

impl CharacterNode for RandomPickNode {
    fn evaluate(&self, eval_context: &EvaluationContext, rng: &mut dyn rand::RngCore) -> NodeResult<i32> {
        let characters = self.array_node.evaluate(eval_context, rng)?;
        if characters.is_empty() {
            return Err(NodeError::EvaluationError("Cannot pick from empty character array".to_string()));
        }
        let index = rng.gen_range(0..characters.len());
        Ok(characters[index].id)
    }
}