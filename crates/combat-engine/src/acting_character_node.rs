// Acting character node - returns the character currently performing action calculation

use super::character_nodes::CharacterNode;

#[derive(Debug)]
pub struct ActingCharacterNode;

impl CharacterNode for ActingCharacterNode {
    fn evaluate<'a>(&self, character: &'a crate::Character, _rng: &mut dyn rand::RngCore) -> &'a crate::Character {
        character
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Character;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_acting_character_node() {
        let character = Character::new("Test".to_string(), 100, 50, 25);
        let mut rng = StdRng::from_entropy();
        
        // Test ActingCharacter node
        let acting_char_node = ActingCharacterNode;
        let returned_char = acting_char_node.evaluate(&character, &mut rng);
        assert_eq!(returned_char.hp, 100);
        assert_eq!(returned_char.name, "Test");
    }
}