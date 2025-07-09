use crate::nodes::unified_node::Node;
use crate::nodes::evaluation_context::EvaluationContext;
use crate::core::NodeResult;
use crate::core::character_hp::CharacterHP;

/// CharacterHP配列からCharacter配列への変換ノード
pub struct CharacterHPToCharacterMappingNode {
    character_hp_array_node: Box<dyn Node<Vec<CharacterHP>>>,
}

impl CharacterHPToCharacterMappingNode {
    pub fn new(character_hp_array_node: Box<dyn Node<Vec<CharacterHP>>>) -> Self {
        Self { character_hp_array_node }
    }
}

impl Node<Vec<crate::Character>> for CharacterHPToCharacterMappingNode {
    fn evaluate(&self, eval_context: &EvaluationContext, rng: &mut dyn rand::RngCore) -> NodeResult<Vec<crate::Character>> {
        let character_hp_array = self.character_hp_array_node.evaluate(eval_context, rng)?;
        
        let character_array: Vec<crate::Character> = character_hp_array
            .iter()
            .map(|char_hp| char_hp.get_character().clone())
            .collect();
        
        Ok(character_array)
    }
}

/// Character配列からCharacterHP配列への変換ノード
pub struct CharacterToCharacterHPMappingNode {
    character_array_node: Box<dyn Node<Vec<crate::Character>>>,
}

impl CharacterToCharacterHPMappingNode {
    pub fn new(character_array_node: Box<dyn Node<Vec<crate::Character>>>) -> Self {
        Self { character_array_node }
    }
}

impl Node<Vec<CharacterHP>> for CharacterToCharacterHPMappingNode {
    fn evaluate(&self, eval_context: &EvaluationContext, rng: &mut dyn rand::RngCore) -> NodeResult<Vec<CharacterHP>> {
        let character_array = self.character_array_node.evaluate(eval_context, rng)?;
        
        let character_hp_array: Vec<CharacterHP> = character_array
            .iter()
            .map(|character| CharacterHP::new(character.clone()))
            .collect();
        
        Ok(character_hp_array)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Character;
    use crate::Team;
    use crate::TeamSide;
    use crate::nodes::character::BattleContext;
    use rand::SeedableRng;

    // テスト用のConstantCharacterHPArrayNode
    struct ConstantCharacterHPArrayNode {
        character_hp_array: Vec<CharacterHP>,
    }

    impl ConstantCharacterHPArrayNode {
        fn new(character_hp_array: Vec<CharacterHP>) -> Self {
            Self { character_hp_array }
        }
    }

    impl Node<Vec<CharacterHP>> for ConstantCharacterHPArrayNode {
        fn evaluate(&self, _eval_context: &EvaluationContext, _rng: &mut dyn rand::RngCore) -> NodeResult<Vec<CharacterHP>> {
            Ok(self.character_hp_array.clone())
        }
    }

    // テスト用のConstantCharacterArrayNode
    struct ConstantCharacterArrayNode {
        character_array: Vec<Character>,
    }

    impl ConstantCharacterArrayNode {
        fn new(character_array: Vec<Character>) -> Self {
            Self { character_array }
        }
    }

    impl Node<Vec<Character>> for ConstantCharacterArrayNode {
        fn evaluate(&self, _eval_context: &EvaluationContext, _rng: &mut dyn rand::RngCore) -> NodeResult<Vec<Character>> {
            Ok(self.character_array.clone())
        }
    }

    #[test]
    fn test_character_hp_to_character_mapping() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(12345);
        
        let character = Character::new(1, "Test".to_string(), 100, 100, 10);
        let team = Team::new("Test Team".to_string(), vec![character.clone()]);
        let battle_context = BattleContext::new(&character, TeamSide::Player, &team, &team);
        let eval_context = EvaluationContext::new(&battle_context);
        
        let char1 = Character::new(1, "Character 1".to_string(), 100, 100, 10);
        let char2 = Character::new(2, "Character 2".to_string(), 100, 100, 10);
        
        let char_hp1 = CharacterHP::from_character_with_hp(char1, 80);
        let char_hp2 = CharacterHP::from_character_with_hp(char2, 60);
        
        let array_node = Box::new(ConstantCharacterHPArrayNode::new(vec![char_hp1, char_hp2]));
        let mapping_node = CharacterHPToCharacterMappingNode::new(array_node);
        
        let result = mapping_node.evaluate(&eval_context, &mut rng).unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].id, 1);
        assert_eq!(result[0].name, "Character 1");
        assert_eq!(result[1].id, 2);
        assert_eq!(result[1].name, "Character 2");
    }

    #[test]
    fn test_character_to_character_hp_mapping() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(12345);
        
        let character = Character::new(1, "Test".to_string(), 100, 100, 10);
        let team = Team::new("Test Team".to_string(), vec![character.clone()]);
        let battle_context = BattleContext::new(&character, TeamSide::Player, &team, &team);
        let eval_context = EvaluationContext::new(&battle_context);
        
        let mut char1 = Character::new(1, "Character 1".to_string(), 100, 100, 10);
        char1.hp = 80;
        let mut char2 = Character::new(2, "Character 2".to_string(), 100, 100, 10);
        char2.hp = 60;
        
        let array_node = Box::new(ConstantCharacterArrayNode::new(vec![char1, char2]));
        let mapping_node = CharacterToCharacterHPMappingNode::new(array_node);
        
        let result = mapping_node.evaluate(&eval_context, &mut rng).unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].get_hp(), 80);
        assert_eq!(result[0].get_character().id, 1);
        assert_eq!(result[1].get_hp(), 60);
        assert_eq!(result[1].get_character().id, 2);
    }
}