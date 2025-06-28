// Character HP from node - returns HP from a character node

use crate::nodes::value::ValueNode;
use super::character_nodes::CharacterNode;

#[derive(Debug)]
pub struct CharacterHpFromNode {
    pub character_node: Box<dyn CharacterNode>,
}

impl CharacterHpFromNode {
    pub fn new(character_node: Box<dyn CharacterNode>) -> Self {
        Self { character_node }
    }
}

impl ValueNode for CharacterHpFromNode {
    fn evaluate(&self, battle_context: &crate::BattleContext, rng: &mut dyn rand::RngCore) -> i32 {
        let target_character = self.character_node.evaluate(battle_context, rng);
        target_character.hp
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Character, ActingCharacterNode};
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_character_hp_from_node() {
        let player = Character::new("Player".to_string(), 100, 50, 25);
        let enemy = Character::new("Enemy".to_string(), 80, 30, 20);
        let acting_character = Character::new("Test".to_string(), 100, 50, 25);
        let battle_context = crate::BattleContext::new(&acting_character, &player, &enemy);
        
        let mut rng = StdRng::from_entropy();
        
        // Test CharacterHpFromNode with ActingCharacterNode
        let char_hp_from_node = CharacterHpFromNode::new(Box::new(ActingCharacterNode));
        assert_eq!(char_hp_from_node.evaluate(&battle_context, &mut rng), 100);
    }
}