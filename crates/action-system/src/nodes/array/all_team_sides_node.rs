// AllTeamSidesNode - returns all possible team sides
use crate::core::NodeResult;
use crate::nodes::unified_node::Node;
use crate::nodes::evaluation_context::EvaluationContext;
use crate::TeamSide;

/// Node that returns an array of both team sides
/// This is useful for operations that need to work with both Player and Enemy teams
#[derive(Debug)]
pub struct AllTeamSidesNode;

impl AllTeamSidesNode {
    pub fn new() -> Self {
        Self
    }
}

impl Default for AllTeamSidesNode {
    fn default() -> Self {
        Self::new()
    }
}

impl Node<Vec<TeamSide>> for AllTeamSidesNode {
    fn evaluate(&self, _eval_context: &EvaluationContext, _rng: &mut dyn rand::RngCore) -> NodeResult<Vec<TeamSide>> {
        Ok(vec![TeamSide::Player, TeamSide::Enemy])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Character, Team, BattleContext};
    use rand::SeedableRng;

    #[test]
    fn test_all_team_sides_node() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(12345);
        
        let character = Character::new(1, "Test".to_string(), 100, 100, 20);
        let team = Team::new("Test Team".to_string(), vec![character.clone()]);
        let battle_context = BattleContext::new(&character, TeamSide::Player, &team, &team);
        let eval_context = EvaluationContext::new(&battle_context);
        
        let all_team_sides_node = AllTeamSidesNode::new();
        let result = Node::<Vec<TeamSide>>::evaluate(&all_team_sides_node, &eval_context, &mut rng).unwrap();
        
        assert_eq!(result, vec![TeamSide::Player, TeamSide::Enemy]);
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_all_team_sides_node_default() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(12345);
        
        let character = Character::new(1, "Test".to_string(), 100, 100, 20);
        let team = Team::new("Test Team".to_string(), vec![character.clone()]);
        let battle_context = BattleContext::new(&character, TeamSide::Player, &team, &team);
        let eval_context = EvaluationContext::new(&battle_context);
        
        let all_team_sides_node = AllTeamSidesNode::default();
        let result = Node::<Vec<TeamSide>>::evaluate(&all_team_sides_node, &eval_context, &mut rng).unwrap();
        
        assert_eq!(result[0], TeamSide::Player);
        assert_eq!(result[1], TeamSide::Enemy);
    }

    #[test]
    fn test_all_team_sides_node_boxed() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(12345);
        
        let character = Character::new(1, "Test".to_string(), 100, 100, 20);
        let team = Team::new("Test Team".to_string(), vec![character.clone()]);
        let battle_context = BattleContext::new(&character, TeamSide::Player, &team, &team);
        let eval_context = EvaluationContext::new(&battle_context);
        
        let boxed_node: Box<dyn Node<Vec<TeamSide>>> = Box::new(AllTeamSidesNode::new());
        let result = boxed_node.evaluate(&eval_context, &mut rng).unwrap();
        
        assert_eq!(result, vec![TeamSide::Player, TeamSide::Enemy]);
    }
}