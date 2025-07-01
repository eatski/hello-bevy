// Character HP node - returns HP from a character node

use crate::nodes::evaluation_context::EvaluationContext;
use crate::nodes::unified_node::Node;

#[derive(Debug)]
pub struct CharacterHpNode {
    pub character_node: Box<dyn Node<i32>>,
}

impl CharacterHpNode {
    pub fn new(character_node: Box<dyn Node<i32>>) -> Self {
        Self { character_node }
    }
}

impl Node<i32> for CharacterHpNode {
    fn evaluate(&self, eval_context: &EvaluationContext, rng: &mut dyn rand::RngCore) -> crate::core::NodeResult<i32> {
        let target_id = self.character_node.evaluate(eval_context, rng)?;
        let battle_context = eval_context.get_battle_context();
        let target_character = battle_context.get_character_by_id(target_id)
            .ok_or_else(|| crate::core::NodeError::EvaluationError(format!("Character with ID {} not found", target_id)))?;
        Ok(target_character.hp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Character, Team, TeamSide, ActingCharacterNode};
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_character_hp_node() {
        let player = Character::new(1, "Player".to_string(), 100, 50, 25);
        let enemy = Character::new(2, "Enemy".to_string(), 80, 30, 20);
        let acting_character = Character::new(3, "Test".to_string(), 100, 50, 25);
        
        let player_team = Team::new("Player Team".to_string(), vec![player.clone(), acting_character.clone()]);
        let enemy_team = Team::new("Enemy Team".to_string(), vec![enemy.clone()]);
        let battle_context = crate::BattleContext::new(&acting_character, TeamSide::Player, &player_team, &enemy_team);
        
        let mut rng = StdRng::from_entropy();
        
        // Test CharacterHpNode with ActingCharacterNode
        let char_hp_node = CharacterHpNode::new(Box::new(ActingCharacterNode));
        let eval_context = EvaluationContext::new(&battle_context);
        assert_eq!(Node::<i32>::evaluate(&char_hp_node, &eval_context, &mut rng), Ok(100));
    }

    #[test]
    fn test_character_hp_node_boxed() {
        let player = Character::new(1, "Player".to_string(), 100, 50, 25);
        let enemy = Character::new(2, "Enemy".to_string(), 80, 30, 20);
        let acting_character = Character::new(3, "Test".to_string(), 100, 50, 25);
        
        let player_team = Team::new("Player Team".to_string(), vec![player.clone(), acting_character.clone()]);
        let enemy_team = Team::new("Enemy Team".to_string(), vec![enemy.clone()]);
        let battle_context = crate::BattleContext::new(&acting_character, TeamSide::Player, &player_team, &enemy_team);
        
        let mut rng = StdRng::from_entropy();
        
        // Test CharacterHpNode with ActingCharacterNode using unified Node<i32>
        let char_hp_node = CharacterHpNode::new(Box::new(ActingCharacterNode));
        let eval_context = EvaluationContext::new(&battle_context);
        assert_eq!(Node::<i32>::evaluate(&char_hp_node, &eval_context, &mut rng), Ok(100));
    }
}