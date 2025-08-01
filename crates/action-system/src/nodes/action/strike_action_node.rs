// Strike action node - resolves to strike action with target character

use crate::core::{NodeResult, NodeError, Action, StrikeAction};
use crate::nodes::evaluation_context::EvaluationContext;
use crate::nodes::unified_node::{CoreNode as Node, BoxedNode};

pub struct StrikeActionNode {
    target: BoxedNode<crate::Character>,
}

impl StrikeActionNode {
    pub fn new(target: BoxedNode<crate::Character>) -> Self {
        Self { target }
    }
}

impl<'a> Node<Box<dyn Action>, EvaluationContext<'a>> for StrikeActionNode {
    fn evaluate(&self, eval_context: &mut EvaluationContext) -> NodeResult<Box<dyn Action>> {
        let battle_context = eval_context.get_battle_context();
        let acting_character = battle_context.get_acting_character();
        
        // Check if acting character can attack (alive)
        if acting_character.hp <= 0 {
            return Err(NodeError::Break);
        }
        
        // Evaluate target character
        let target_character = self.target.evaluate(eval_context)?;
        
        // Create and return StrikeAction with the target character's ID
        Ok(Box::new(StrikeAction::new(target_character.id)))
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
        
        // Create strike action with acting character as target
        let target = Box::new(ActingCharacterNode);
        let strike = StrikeActionNode::new(target);
        let mut rng = StdRng::from_entropy();
        
        let mut eval_context = EvaluationContext::new(&battle_context, &mut rng);
        let result = Node::<Box<dyn Action>, EvaluationContext>::evaluate(&strike, &mut eval_context);
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
        let mut dead_eval_context = EvaluationContext::new(&dead_battle_context, &mut rng);
        let result = Node::<Box<dyn Action>, EvaluationContext>::evaluate(&strike_dead, &mut dead_eval_context);
        assert!(matches!(result, Err(NodeError::Break)), "StrikeActionNode should return Break error for dead character");
    }
}