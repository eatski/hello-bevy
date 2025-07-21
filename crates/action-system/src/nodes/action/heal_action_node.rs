// Heal action node - resolves to heal action with target character

use crate::core::{NodeResult, NodeError, Action, HealAction};
use node_core::Node;

pub struct HealActionNode {
    target: Box<dyn Node<crate::Character>>,
}

impl HealActionNode {
    pub fn new(target: Box<dyn Node<crate::Character>>) -> Self {
        Self { target }
    }
}

impl Node<Box<dyn Action>> for HealActionNode {
    fn evaluate(&self, eval_context: &mut EvaluationContext) -> NodeResult<Box<dyn Action>> {
        let battle_context = eval_context.get_battle_context();
        let acting_character = battle_context.get_acting_character();
        
        // Check if acting character can perform heal (alive and has MP)
        if acting_character.hp <= 0 || acting_character.mp < 10 {
            return Err(NodeError::Break);
        }
        
        // Evaluate target character
        let target_character = self.target.evaluate(eval_context)?;
        
        // Create and return HealAction with the target character's ID
        Ok(Box::new(HealAction::new(target_character.id)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Character, Team, TeamSide};
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_heal_action_node() {
        use crate::nodes::character::ActingCharacterNode;
        
        let player = Character::new(1, "Player".to_string(), 100, 50, 25);
        let enemy = Character::new(2, "Enemy".to_string(), 80, 30, 20);
        
        let acting_character = Character::new(3, "Test".to_string(), 100, 50, 25);
        let player_team = Team::new("Player Team".to_string(), vec![player.clone(), acting_character.clone()]);
        let enemy_team = Team::new("Enemy Team".to_string(), vec![enemy.clone()]);
        let battle_context = crate::BattleContext::new(&acting_character, TeamSide::Player, &player_team, &enemy_team);
        
        // Create heal action with acting character as target
        let target = Box::new(ActingCharacterNode);
        let heal = HealActionNode::new(target);
        let mut rng = StdRng::from_entropy();
        
        let mut eval_context = EvaluationContext::new(&battle_context, &mut rng);
        let result = Node::<Box<dyn Action>>::evaluate(&heal, &mut eval_context);
        assert!(result.is_ok(), "HealActionNode should return HealAction for alive character");
        if let Ok(action) = result {
            assert_eq!(action.get_action_name(), "Heal");
        }
        
        let dead_character = Character::new(4, "Dead".to_string(), 0, 0, 25);
        let dead_player_team = Team::new("Player Team".to_string(), vec![player.clone(), dead_character.clone()]);
        let dead_enemy_team = Team::new("Enemy Team".to_string(), vec![enemy.clone()]);
        let dead_battle_context = crate::BattleContext::new(&dead_character, TeamSide::Player, &dead_player_team, &dead_enemy_team);
        let target_dead = Box::new(ActingCharacterNode);
        let heal_dead = HealActionNode::new(target_dead);
        let mut dead_eval_context = EvaluationContext::new(&dead_battle_context, &mut rng);
        let result = Node::<Box<dyn Action>>::evaluate(&heal_dead, &mut dead_eval_context);
        assert!(matches!(result, Err(NodeError::Break)), "HealActionNode should return Break error for dead character");
    }
}