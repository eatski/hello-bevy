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
    fn evaluate(&self, _battle_context: &crate::BattleContext, _rng: &mut dyn rand::RngCore) -> i32 {
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
        let player = Character::new("Player".to_string(), 100, 50, 25);
        let enemy = Character::new("Enemy".to_string(), 80, 30, 20);
        let acting_character = Character::new("Test".to_string(), 100, 50, 25);
        let battle_context = crate::BattleContext::new(&acting_character, &player, &enemy);
        
        let mut rng = StdRng::from_entropy();
        
        // Test CharacterHP value node
        let char_hp_node = CharacterHpValueNode::new(acting_character.clone());
        assert_eq!(char_hp_node.evaluate(&battle_context, &mut rng), 100);
    }
}