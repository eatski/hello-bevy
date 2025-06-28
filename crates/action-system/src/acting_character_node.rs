// Acting character node - returns the character currently performing action calculation

use super::character_nodes::CharacterNode;

#[derive(Debug)]
pub struct ActingCharacterNode;

impl CharacterNode for ActingCharacterNode {
    fn evaluate<'a>(&self, battle_context: &crate::BattleContext<'a>, _rng: &mut dyn rand::RngCore) -> &'a crate::Character {
        battle_context.get_acting_character()
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
        let player = Character::new("Player".to_string(), 100, 50, 25);
        let enemy = Character::new("Enemy".to_string(), 80, 30, 20);
        let acting_character = Character::new("Test".to_string(), 100, 50, 25);
        let battle_context = crate::BattleContext::new(&acting_character, &player, &enemy);
        
        let mut rng = StdRng::from_entropy();
        
        // Test ActingCharacter node
        let acting_char_node = ActingCharacterNode;
        let returned_char = acting_char_node.evaluate(&battle_context, &mut rng);
        assert_eq!(returned_char.hp, 100);
        assert_eq!(returned_char.name, "Test");
    }
}