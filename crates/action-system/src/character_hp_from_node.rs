// Character HP from node - returns HP from a character node

use super::value_nodes::ValueNode;
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
    fn evaluate(&self, character: &crate::Character, rng: &mut dyn rand::RngCore) -> i32 {
        let target_character = self.character_node.evaluate(character, rng);
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
        let character = Character::new("Test".to_string(), 100, 50, 25);
        let mut rng = StdRng::from_entropy();
        
        // Test CharacterHpFromNode with ActingCharacterNode
        let char_hp_from_node = CharacterHpFromNode::new(Box::new(ActingCharacterNode));
        assert_eq!(char_hp_from_node.evaluate(&character, &mut rng), 100);
    }
}