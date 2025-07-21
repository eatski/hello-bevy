// Character HP Value node - returns i32 HP value from a character node

use crate::nodes::evaluation_context::EvaluationContext;
use node_core::Node;
use crate::nodes::evaluation_context::EvaluationContext;
use crate::core::NodeResult;

pub struct CharacterHpValueNode {
    pub character_node: Box<dyn Node<crate::Character>>,
}

impl CharacterHpValueNode {
    pub fn new(character_node: Box<dyn Node<crate::Character>>) -> Self {
        Self { character_node }
    }
}

impl Node<i32> for CharacterHpValueNode {
    fn evaluate(&self, eval_context: &mut EvaluationContext) -> NodeResult<i32> {
        let character = self.character_node.evaluate(eval_context)?;
        Ok(character.hp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Character, Team, TeamSide, ActingCharacterNode};
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_character_hp_value_node() {
        let player = Character::new(1, "Player".to_string(), 100, 50, 25);
        let enemy = Character::new(2, "Enemy".to_string(), 80, 30, 20);
        let acting_character = Character::new(3, "Test".to_string(), 100, 50, 25);
        
        let player_team = Team::new("Player Team".to_string(), vec![player.clone(), acting_character.clone()]);
        let enemy_team = Team::new("Enemy Team".to_string(), vec![enemy.clone()]);
        let battle_context = crate::BattleContext::new(&acting_character, TeamSide::Player, &player_team, &enemy_team);
        
        let mut rng = StdRng::from_entropy();
        
        // Test CharacterHpValueNode with ActingCharacterNode
        let char_hp_value_node = CharacterHpValueNode::new(Box::new(ActingCharacterNode));
        let mut eval_context = EvaluationContext::new(&battle_context, &mut rng);
        let result = Node::<i32>::evaluate(&char_hp_value_node, &mut eval_context).unwrap();
        
        assert_eq!(result, 100);
    }

    #[test]
    fn test_character_hp_value_node_with_damaged_character() {
        let mut acting_character = Character::new(4, "Damaged".to_string(), 100, 50, 25);
        acting_character.hp = 60; // ダメージを受けている
        
        let player_team = Team::new("Player Team".to_string(), vec![acting_character.clone()]);
        let enemy_team = Team::new("Enemy Team".to_string(), vec![]);
        let battle_context = crate::BattleContext::new(&acting_character, TeamSide::Player, &player_team, &enemy_team);
        
        let mut rng = StdRng::from_entropy();
        
        let char_hp_value_node = CharacterHpValueNode::new(Box::new(ActingCharacterNode));
        let mut eval_context = EvaluationContext::new(&battle_context, &mut rng);
        let result = Node::<i32>::evaluate(&char_hp_value_node, &mut eval_context).unwrap();
        
        assert_eq!(result, 60);
    }
}