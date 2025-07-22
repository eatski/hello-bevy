// TeamMembersNode - returns all members from a specific team

use crate::core::NodeResult;
use crate::nodes::evaluation_context::EvaluationContext;
use crate::nodes::unified_node::{CoreNode as Node, BoxedNode};
use crate::{Character, TeamSide};

#[derive(Debug)]
pub struct TeamMembersNode {
    team: TeamSide,
}

pub struct TeamMembersNodeWithNode {
    team_side_node: BoxedNode<TeamSide>,
}

impl TeamMembersNode {
    pub fn new(team: TeamSide) -> Self {
        Self { team }
    }
    
    pub fn new_with_node(team_side_node: BoxedNode<TeamSide>) -> TeamMembersNodeWithNode {
        TeamMembersNodeWithNode { team_side_node }
    }
}

impl<'a> Node<Vec<Character>, EvaluationContext<'a>> for TeamMembersNode {
    fn evaluate(&self, eval_context: &mut EvaluationContext) -> NodeResult<Vec<Character>> {
        let battle_context = eval_context.get_battle_context();
        let character_refs = battle_context.get_team_members(self.team);
        let characters = character_refs.into_iter().cloned().collect();
        Ok(characters)
    }
}

impl<'a> Node<Vec<Character>, EvaluationContext<'a>> for TeamMembersNodeWithNode {
    fn evaluate(&self, eval_context: &mut EvaluationContext) -> NodeResult<Vec<Character>> {
        let team_side = self.team_side_node.evaluate(eval_context)?;
        let battle_context = eval_context.get_battle_context();
        let character_refs = battle_context.get_team_members(team_side);
        let characters = character_refs.into_iter().cloned().collect();
        Ok(characters)
    }
}