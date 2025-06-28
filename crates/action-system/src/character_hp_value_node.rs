// Character HP value node - returns character's current HP

use super::value_nodes::ValueNode;

#[derive(Debug)]
pub struct CharacterHpValueNode {
    pub character: crate::Character,
}

impl CharacterHpValueNode {
    pub fn new(character: crate::Character) -> Self {
        Self { character }
    }
}

impl ValueNode for CharacterHpValueNode {
    fn evaluate(&self, _character: &crate::Character, _rng: &mut dyn rand::RngCore) -> i32 {
        self.character.hp
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Character;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_character_hp_value_node() {
        let character = Character::new("Test".to_string(), 100, 50, 25);
        let mut rng = StdRng::from_entropy();
        
        // Test CharacterHP value node
        let char_hp_node = CharacterHpValueNode::new(character.clone());
        assert_eq!(char_hp_node.evaluate(&character, &mut rng), 100);
    }
}