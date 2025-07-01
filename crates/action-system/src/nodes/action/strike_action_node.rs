// Strike action node - resolves to strike action with target character

use crate::core::{ActionResolver, NodeResult, NodeError, Action, StrikeAction};
use crate::nodes::character::CharacterNode;
use crate::nodes::evaluation_context::EvaluationContext;

#[derive(Debug)]
pub struct StrikeActionNode {
    target: Box<dyn CharacterNode>,
}

impl StrikeActionNode {
    pub fn new(target: Box<dyn CharacterNode>) -> Self {
        Self { target }
    }
}

impl ActionResolver for StrikeActionNode {
    fn resolve(&self, eval_context: &EvaluationContext, rng: &mut dyn rand::RngCore) -> NodeResult<Box<dyn Action>> {
        let battle_context = eval_context.get_battle_context();
        let acting_character = battle_context.get_acting_character();
        
        // Check if acting character can attack (alive)
        if acting_character.hp <= 0 {
            return Err(NodeError::Break);
        }
        
        // Evaluate target character ID
        let target_id = self.target.evaluate(eval_context, rng)?;
        
        // Create and return StrikeAction with the evaluated target ID
        Ok(Box::new(StrikeAction::new(target_id)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Character, Team, TeamSide};
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_strike_action_node() {
        use crate::nodes::character::ActingCharacterNode;
        
        let player = Character::new(1, "Player".to_string(), 100, 50, 25);
        let enemy = Character::new(2, "Enemy".to_string(), 80, 30, 20);
        
        let acting_character = Character::new(3, "Test".to_string(), 100, 50, 25);
        let player_team = Team::new("Player Team".to_string(), vec![player.clone(), acting_character.clone()]);
        let enemy_team = Team::new("Enemy Team".to_string(), vec![enemy.clone()]);
        let battle_context = crate::BattleContext::new(&acting_character, TeamSide::Player, &player_team, &enemy_team);
        
        // Create strike action with enemy as target
        let target = Box::new(ActingCharacterNode);
        let strike = StrikeActionNode::new(target);
        let mut rng = StdRng::from_entropy();
        
        let eval_context = EvaluationContext::new(&battle_context);
        let result = strike.resolve(&eval_context, &mut rng);
        assert!(result.is_ok(), "StrikeActionNode should return StrikeAction for alive character");
        if let Ok(action) = result {
            assert_eq!(action.get_action_name(), "Strike");
        }
        
        let dead_character = Character::new(4, "Dead".to_string(), 0, 0, 25);
        let dead_player_team = Team::new("Player Team".to_string(), vec![player.clone(), dead_character.clone()]);
        let dead_enemy_team = Team::new("Enemy Team".to_string(), vec![enemy.clone()]);
        let dead_battle_context = crate::BattleContext::new(&dead_character, TeamSide::Player, &dead_player_team, &dead_enemy_team);
        let target_dead = Box::new(ActingCharacterNode);
        let strike_dead = StrikeActionNode::new(target_dead);
        let dead_eval_context = EvaluationContext::new(&dead_battle_context);
        let result = strike_dead.resolve(&dead_eval_context, &mut rng);
        assert!(matches!(result, Err(NodeError::Break)), "StrikeActionNode should return Break error for dead character");
    }
}