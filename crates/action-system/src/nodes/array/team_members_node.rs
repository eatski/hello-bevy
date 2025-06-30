// TeamMembersNode - returns all members from a specific team

use super::CharacterArrayNode;
use crate::core::NodeResult;
use crate::nodes::character::BattleContext;

#[derive(Debug)]
pub struct TeamMembersNode {
    team: crate::TeamSide,
}

impl TeamMembersNode {
    pub fn new(team: crate::TeamSide) -> Self {
        Self { team }
    }
}

impl CharacterArrayNode for TeamMembersNode {
    fn evaluate(&self, battle_context: &BattleContext, _rng: &mut dyn rand::RngCore) -> NodeResult<Vec<crate::Character>> {
        let character_refs = battle_context.get_team_members(self.team);
        let characters = character_refs.into_iter().cloned().collect();
        Ok(characters)
    }
}