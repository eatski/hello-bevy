// RandomPickNode - selects a random character from character arrays

use super::CharacterArrayNode;
use crate::core::{NodeError, NodeResult};
use crate::nodes::character::{BattleContext, CharacterNode};
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
    fn evaluate(&self, battle_context: &BattleContext, rng: &mut dyn rand::RngCore) -> NodeResult<crate::Character> {
        let characters = self.array_node.evaluate(battle_context, rng)?;
        if characters.is_empty() {
            return Err(NodeError::EvaluationError("Cannot pick from empty character array".to_string()));
        }
        let index = rng.gen_range(0..characters.len());
        Ok(characters[index].clone())
    }
}