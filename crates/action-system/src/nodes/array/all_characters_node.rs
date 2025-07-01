// AllCharactersNode - returns all characters in the battle context

use super::CharacterArrayNode;
use crate::core::NodeResult;
use crate::nodes::evaluation_context::EvaluationContext;

#[derive(Debug)]
pub struct AllCharactersNode;

impl AllCharactersNode {
    pub fn new() -> Self {
        Self
    }
}

impl CharacterArrayNode for AllCharactersNode {
    fn evaluate(&self, eval_context: &EvaluationContext, _rng: &mut dyn rand::RngCore) -> NodeResult<Vec<crate::Character>> {
        let battle_context = eval_context.get_battle_context();
        let character_refs = battle_context.all_characters();
        let characters = character_refs.into_iter().cloned().collect();
        Ok(characters)
    }
}