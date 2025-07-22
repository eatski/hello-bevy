// Acting character node - returns the character currently performing action calculation

use crate::nodes::unified_node::CoreNode as Node;
use crate::nodes::evaluation_context::EvaluationContext;

#[derive(Debug)]
pub struct ActingCharacterNode;

impl<'a> Node<crate::Character, EvaluationContext<'a>> for ActingCharacterNode {
    fn evaluate(&self, eval_context: &mut crate::nodes::evaluation_context::EvaluationContext) -> crate::core::NodeResult<crate::Character> {
        Ok(eval_context.get_battle_context().get_acting_character().clone())
    }
}

