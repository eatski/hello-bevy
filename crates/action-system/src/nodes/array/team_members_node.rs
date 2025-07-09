// TeamMembersNode - returns all members from a specific team

use crate::core::NodeResult;
use crate::nodes::evaluation_context::EvaluationContext;
use crate::nodes::unified_node::Node;
use crate::{Character, TeamSide};

#[derive(Debug)]
pub struct TeamMembersNode {
    team: TeamSide,
}

pub struct TeamMembersNodeWithNode {
    team_side_node: Box<dyn Node<TeamSide>>,
}

impl TeamMembersNode {
    pub fn new(team: TeamSide) -> Self {
        Self { team }
    }
    
    pub fn new_with_node(team_side_node: Box<dyn Node<TeamSide>>) -> TeamMembersNodeWithNode {
        TeamMembersNodeWithNode { team_side_node }
    }
}

impl Node<Vec<Character>> for TeamMembersNode {
    fn evaluate(&self, eval_context: &EvaluationContext, _rng: &mut dyn rand::RngCore) -> NodeResult<Vec<Character>> {
        let battle_context = eval_context.get_battle_context();
        let character_refs = battle_context.get_team_members(self.team);
        let characters = character_refs.into_iter().cloned().collect();
        Ok(characters)
    }
}

impl Node<Vec<Character>> for TeamMembersNodeWithNode {
    fn evaluate(&self, eval_context: &EvaluationContext, rng: &mut dyn rand::RngCore) -> NodeResult<Vec<Character>> {
        let team_side = self.team_side_node.evaluate(eval_context, rng)?;
        let battle_context = eval_context.get_battle_context();
        let character_refs = battle_context.get_team_members(team_side);
        let characters = character_refs.into_iter().cloned().collect();
        Ok(characters)
    }
}