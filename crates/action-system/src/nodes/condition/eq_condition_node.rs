use crate::core::{NodeError, NodeResult};
use crate::nodes::evaluation_context::EvaluationContext;
use node_core::Node;
use crate::nodes::evaluation_context::EvaluationContext;
use crate::TeamSide;
use std::fmt;

pub struct EqConditionNode<T> {
    left: Box<dyn Node<T>>,
    right: Box<dyn Node<T>>,
}

impl<T> EqConditionNode<T> {
    pub fn new(left: Box<dyn Node<T>>, right: Box<dyn Node<T>>) -> Self {
        Self { left, right }
    }
}

impl<T: PartialEq + fmt::Debug + Clone + Send + Sync + 'static> Node<bool> for EqConditionNode<T> {
    fn evaluate(&self, context: &mut EvaluationContext) -> NodeResult<bool> {
        let left_value = self.left.evaluate(context)?;
        let right_value = self.right.evaluate(context)?;
        Ok(left_value == right_value)
    }
}

// Specialized for TeamSide comparison
pub type TeamSideEqNode = EqConditionNode<TeamSide>;

pub struct CharacterTeamNode {
    character_node: Box<dyn Node<crate::Character>>,
}

impl CharacterTeamNode {
    pub fn new(character_node: Box<dyn Node<crate::Character>>) -> Self {
        Self { character_node }
    }
}

impl Node<TeamSide> for CharacterTeamNode {
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