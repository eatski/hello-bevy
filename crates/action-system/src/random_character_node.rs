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
    fn evaluate<'a>(&self, battle_context: &BattleContext<'a>, rng: &mut dyn rand::RngCore) -> &'a crate::Character {
        let all_characters = battle_context.all_characters();
        // Select a random character from the battle
        all_characters.choose(rng).map_or(battle_context.get_acting_character(), |&character| character)
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
        let _random_char_node = RandomCharacterNode::new();
        // Just test that creation works
        assert!(true);
    }

    #[test]
    fn test_random_character_node_evaluation() {
        let player = Character::new("Player".to_string(), 100, 50, 25);
        let enemy = Character::new("Enemy".to_string(), 80, 30, 20);
        let acting_character = Character::new("Acting".to_string(), 100, 50, 25);
        let battle_context = BattleContext::new(&acting_character, &player, &enemy);
        
        let mut rng = StdRng::from_entropy();
        
        let random_char_node = RandomCharacterNode::new();
        let returned_char = random_char_node.evaluate(&battle_context, &mut rng);
        
        // Should return either player or enemy
        assert!(returned_char.name == "Player" || returned_char.name == "Enemy");
    }
    
    #[test]
    fn test_random_character_node_deterministic() {
        use rand::SeedableRng;
        
        let player = Character::new("Player".to_string(), 100, 50, 25);
        let enemy = Character::new("Enemy".to_string(), 80, 30, 20);
        let acting_character = Character::new("Acting".to_string(), 100, 50, 25);
        let battle_context = BattleContext::new(&acting_character, &player, &enemy);
        
        let mut rng1 = StdRng::seed_from_u64(12345);
        let mut rng2 = StdRng::seed_from_u64(12345);
        
        let random_char_node = RandomCharacterNode::new();
        let result1 = random_char_node.evaluate(&battle_context, &mut rng1);
        let result2 = random_char_node.evaluate(&battle_context, &mut rng2);
        
        // Same seed should produce same result
        assert_eq!(result1.name, result2.name);
    }
}