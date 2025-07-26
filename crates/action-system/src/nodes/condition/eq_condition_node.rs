use crate::core::{NodeError, NodeResult};
use crate::nodes::evaluation_context::EvaluationContext;
use crate::nodes::unified_node::{CoreNode as Node, BoxedNode};
use crate::TeamSide;
use std::fmt;

pub struct EqConditionNode<T> {
    left: BoxedNode<T>,
    right: BoxedNode<T>,
}

impl<T> EqConditionNode<T> {
    pub fn new(left: BoxedNode<T>, right: BoxedNode<T>) -> Self {
        Self { left, right }
    }
}

impl<'a, T: PartialEq + fmt::Debug + Clone + Send + Sync + 'static> Node<bool, EvaluationContext<'a>> for EqConditionNode<T> {
    fn evaluate(&self, context: &mut EvaluationContext) -> NodeResult<bool> {
        let left_value = self.left.evaluate(context)?;
        let right_value = self.right.evaluate(context)?;
        Ok(left_value == right_value)
    }
}


pub struct CharacterTeamNode {
    character_node: BoxedNode<crate::Character>,
}

impl CharacterTeamNode {
    pub fn new(character_node: BoxedNode<crate::Character>) -> Self {
        Self { character_node }
    }
}

impl<'a> Node<TeamSide, EvaluationContext<'a>> for CharacterTeamNode {
    fn evaluate(&self, context: &mut EvaluationContext) -> NodeResult<TeamSide> {
        let character = self.character_node.evaluate(context)?;
        
        // Check if character is in player team
        if context.battle_context.player_team.get_member_by_id(character.id).is_some() {
            return Ok(TeamSide::Player);
        }
        
        // Check if character is in enemy team
        if context.battle_context.enemy_team.get_member_by_id(character.id).is_some() {
            return Ok(TeamSide::Enemy);
        }
        
        Err(NodeError::EvaluationError(format!(
            "Character with ID {} not found in any team", 
            character.id
        )))
    }
}