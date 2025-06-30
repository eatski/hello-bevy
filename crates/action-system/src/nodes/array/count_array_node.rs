// CountArrayNode - counts elements in character arrays

use super::CharacterArrayNode;
use crate::core::NodeResult;
use crate::nodes::character::BattleContext;
use crate::nodes::value::ValueNode;

#[derive(Debug)]
pub struct CountArrayNode {
    array_node: Box<dyn CharacterArrayNode>,
}

impl CountArrayNode {
    pub fn new(array_node: Box<dyn CharacterArrayNode>) -> Self {
        Self { array_node }
    }
}

impl ValueNode for CountArrayNode {
    fn evaluate(&self, battle_context: &BattleContext, rng: &mut dyn rand::RngCore) -> NodeResult<i32> {
        let characters = self.array_node.evaluate(battle_context, rng)?;
        Ok(characters.len() as i32)
    }
}