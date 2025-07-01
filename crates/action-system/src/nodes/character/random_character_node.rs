// Random character node - returns a random character from the battle

use super::character_nodes::{CharacterNode, BattleContext};
use rand::seq::SliceRandom;

#[derive(Debug)]
pub struct RandomCharacterNode;

impl RandomCharacterNode {
    pub fn new() -> Self {
        Self
    }
}

impl CharacterNode for RandomCharacterNode {
    fn evaluate(&self, battle_context: &BattleContext, rng: &mut dyn rand::RngCore) -> crate::core::NodeResult<i32> {
        let all_characters = battle_context.all_characters();
        if all_characters.is_empty() {
            return Err(crate::core::NodeError::EvaluationError("No characters available for random selection".to_string()));
        }
        // Select a random character from the battle
        match all_characters.choose(rng) {
            Some(&character) => Ok(character.id),
            None => Ok(battle_context.get_acting_character().id), // fallback
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Character;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_random_character_node_creation() {
        let random_char_node = RandomCharacterNode::new();
        // Just test that creation works without panicking
        // No additional assertions needed for construction test
        drop(random_char_node);
    }

    #[test]
    fn test_random_character_node_evaluation() {
        let player = Character::new(1, "Player".to_string(), 100, 50, 25);
        let enemy = Character::new(2, "Enemy".to_string(), 80, 30, 20);
        let acting_character = Character::new(3, "Acting".to_string(), 100, 50, 25);
        let battle_context = BattleContext::new(&acting_character, &player, &enemy);
        
        let mut rng = StdRng::from_entropy();
        
        let random_char_node = RandomCharacterNode::new();
        let returned_char_id = random_char_node.evaluate(&battle_context, &mut rng).unwrap();
        
        // Should return either player or enemy ID
        assert!(returned_char_id == player.id || returned_char_id == enemy.id, "Expected player or enemy ID, got {}", returned_char_id);
    }
    
    #[test]
    fn test_single_rng_multiple_character_selections_vary() {
        // 1つのRNGで複数回キャラクター選択し、結果が変わることを検証
        use rand::SeedableRng;
        
        let player = Character::new(4, "Player".to_string(), 100, 50, 25);
        let enemy = Character::new(5, "Enemy".to_string(), 80, 30, 20);
        let acting_character = Character::new(6, "Acting".to_string(), 100, 50, 25);
        let battle_context = BattleContext::new(&acting_character, &player, &enemy);
        
        let mut rng = StdRng::seed_from_u64(12345);
        let random_char_node = RandomCharacterNode::new();
        
        let mut results = Vec::new();
        
        // 同一RNGで20回キャラクター選択
        for _ in 0..20 {
            let result_id = random_char_node.evaluate(&battle_context, &mut rng).unwrap();
            results.push(result_id);
        }
        
        // 全て同じキャラクターではないことを確認
        let first_id = results[0];
        let has_different_character = results.iter().any(|&id| id != first_id);
        
        assert!(has_different_character, "Multiple character selections with same RNG should produce different results");
    }
}