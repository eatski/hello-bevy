// RandomCharacterPickNode - selects a random character from an array and returns the Character object
use crate::core::{NodeError, NodeResult};
use crate::nodes::evaluation_context::EvaluationContext;
use crate::nodes::unified_node::Node;
use crate::Character;
use rand::Rng;

/// RandomCharacterPickNode that returns the actual Character object
pub struct RandomCharacterPickNode {
    array_node: Box<dyn Node<Vec<Character>>>,
}

impl RandomCharacterPickNode {
    pub fn new(array_node: Box<dyn Node<Vec<Character>>>) -> Self {
        Self { array_node }
    }
}

impl Node<Character> for RandomCharacterPickNode {
    fn evaluate(&self, context: &EvaluationContext, rng: &mut dyn rand::RngCore) -> NodeResult<Character> {
        let characters = self.array_node.evaluate(context, rng)?;
        if characters.is_empty() {
            return Err(NodeError::EvaluationError("Cannot pick from empty character array".to_string()));
        }
        
        let index = rng.gen_range(0..characters.len());
        Ok(characters[index].clone())
    }
}