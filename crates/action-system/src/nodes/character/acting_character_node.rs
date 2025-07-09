// Acting character node - returns the character currently performing action calculation

use crate::nodes::unified_node::Node;

#[derive(Debug)]
pub struct ActingCharacterNode;

impl Node<crate::Character> for ActingCharacterNode {
    fn evaluate(&self, eval_context: &crate::nodes::evaluation_context::EvaluationContext, _rng: &mut dyn rand::RngCore) -> crate::core::NodeResult<crate::Character> {
        Ok(eval_context.get_battle_context().get_acting_character().clone())
    }
}

