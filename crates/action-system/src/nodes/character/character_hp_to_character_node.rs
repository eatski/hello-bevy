// HP Character node - returns Character from a CharacterHP node

use crate::nodes::evaluation_context::EvaluationContext;
use crate::nodes::unified_node::Node;
use crate::core::character_hp::CharacterHP;

pub struct CharacterHpToCharacterNode {
    pub character_hp_node: Box<dyn Node<CharacterHP>>,
}

impl CharacterHpToCharacterNode {
    pub fn new(character_hp_node: Box<dyn Node<CharacterHP>>) -> Self {
        Self { character_hp_node }
    }
}

impl Node<crate::Character> for CharacterHpToCharacterNode {
    fn evaluate(&self, eval_context: &EvaluationContext, rng: &mut dyn rand::RngCore) -> crate::core::NodeResult<crate::Character> {
        let character_hp = self.character_hp_node.evaluate(eval_context, rng)?;
        Ok(character_hp.get_character().clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Character, Team, TeamSide};
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    // テスト用のCharacterHPを返すNode
    struct TestCharacterHPNode {
        character_hp: CharacterHP,
    }

    impl TestCharacterHPNode {
        fn new(character_hp: CharacterHP) -> Self {
            Self { character_hp }
        }
    }

    impl Node<CharacterHP> for TestCharacterHPNode {
        fn evaluate(&self, _eval_context: &EvaluationContext, _rng: &mut dyn rand::RngCore) -> crate::core::NodeResult<CharacterHP> {
            Ok(self.character_hp.clone())
        }
    }

    #[test]
    fn test_hp_character_node() {
        let character = Character::new(1, "Test Player".to_string(), 100, 50, 25);
        let character_hp = CharacterHP::new(character.clone());
        
        let player_team = Team::new("Player Team".to_string(), vec![character.clone()]);
        let enemy_team = Team::new("Enemy Team".to_string(), vec![]);
        let battle_context = crate::BattleContext::new(&character, TeamSide::Player, &player_team, &enemy_team);
        
        let mut rng = StdRng::from_entropy();
        
        // Test CharacterHpToCharacterNode with TestCharacterHPNode
        let hp_char_node = CharacterHpToCharacterNode::new(Box::new(TestCharacterHPNode::new(character_hp)));
        let eval_context = EvaluationContext::new(&battle_context);
        let result = Node::<Character>::evaluate(&hp_char_node, &eval_context, &mut rng).unwrap();
        
        assert_eq!(result.id, 1);
        assert_eq!(result.name, "Test Player");
        assert_eq!(result.hp, 100);
    }

    #[test]
    fn test_hp_character_node_with_modified_hp() {
        let mut character = Character::new(2, "Injured Player".to_string(), 100, 50, 25);
        character.hp = 75; // 体力を減らしておく
        
        let character_hp = CharacterHP::new(character.clone());
        
        let player_team = Team::new("Player Team".to_string(), vec![character.clone()]);
        let enemy_team = Team::new("Enemy Team".to_string(), vec![]);
        let battle_context = crate::BattleContext::new(&character, TeamSide::Player, &player_team, &enemy_team);
        
        let mut rng = StdRng::from_entropy();
        
        let hp_char_node = CharacterHpToCharacterNode::new(Box::new(TestCharacterHPNode::new(character_hp)));
        let eval_context = EvaluationContext::new(&battle_context);
        let result = Node::<Character>::evaluate(&hp_char_node, &eval_context, &mut rng).unwrap();
        
        assert_eq!(result.id, 2);
        assert_eq!(result.name, "Injured Player");
        assert_eq!(result.hp, 75);
    }
}