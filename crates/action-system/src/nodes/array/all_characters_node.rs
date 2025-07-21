// AllCharactersNode - returns all characters in the battle context

use crate::core::NodeResult;
use crate::nodes::evaluation_context::EvaluationContext;
use crate::nodes::unified_node::Node;
use crate::Character;

#[derive(Debug)]
pub struct AllCharactersNode;

impl AllCharactersNode {
    pub fn new() -> Self {
        Self
    }
}

impl Node<Vec<Character>> for AllCharactersNode {
    fn evaluate(&self, eval_context: &mut EvaluationContext) -> NodeResult<Vec<Character>> {
        let battle_context = eval_context.get_battle_context();
        let character_refs = battle_context.all_characters();
        let characters = character_refs.into_iter().cloned().collect();
        Ok(characters)
    }
}