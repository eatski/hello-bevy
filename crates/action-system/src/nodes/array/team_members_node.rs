// TeamMembersNode - returns all members from a specific team

use crate::core::NodeResult;
use crate::nodes::evaluation_context::EvaluationContext;
use crate::nodes::unified_node::Node;
use crate::{Character, TeamSide};

#[derive(Debug)]
pub struct TeamMembersNode {
    team: TeamSide,
}

impl TeamMembersNode {
    pub fn new(team: TeamSide) -> Self {
        Self { team }
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