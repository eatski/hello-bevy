// Character HP node - returns HP from a character node

use crate::nodes::value::ValueNode;
use super::character_nodes::CharacterNode;

#[derive(Debug)]
pub struct CharacterHpNode {
    pub character_node: Box<dyn CharacterNode>,
}

impl CharacterHpNode {
    pub fn new(character_node: Box<dyn CharacterNode>) -> Self {
        Self { character_node }
    }
}

impl ValueNode for CharacterHpNode {
    fn evaluate(&self, battle_context: &crate::BattleContext, rng: &mut dyn rand::RngCore) -> crate::core::NodeResult<i32> {
        let target_id = self.character_node.evaluate(battle_context, rng)?;
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
        assert_eq!(char_hp_node.evaluate(&battle_context, &mut rng), Ok(100));
    }
}