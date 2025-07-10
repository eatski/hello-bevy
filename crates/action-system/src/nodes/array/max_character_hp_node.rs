use crate::nodes::unified_node::Node;
use crate::nodes::evaluation_context::EvaluationContext;
use crate::core::NodeResult;
use crate::core::character_hp::CharacterHP;

/// CharacterHP配列内の最大HP値を持つCharacterHPを返すノード
pub struct MaxCharacterHPNode {
    character_hp_array_node: Box<dyn Node<Vec<CharacterHP>>>,
}

impl MaxCharacterHPNode {
    pub fn new(character_hp_array_node: Box<dyn Node<Vec<CharacterHP>>>) -> Self {
        Self { character_hp_array_node }
    }
}

impl Node<CharacterHP> for MaxCharacterHPNode {
    fn evaluate(&self, eval_context: &EvaluationContext, rng: &mut dyn rand::RngCore) -> NodeResult<CharacterHP> {
        let character_hp_array = self.character_hp_array_node.evaluate(eval_context, rng)?;
        
        if character_hp_array.is_empty() {
            return Err(crate::NodeError::EvaluationError("Cannot find max of empty CharacterHP array".to_string()));
        }
        
        // CharacterHPはOrdを実装しているので、maxで最大値を取得可能
        let max_character_hp = character_hp_array.iter().max().unwrap();
        Ok(max_character_hp.clone())
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

    #[test]
    fn test_max_character_hp_node_basic() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(12345);
        
        let character = Character::new(1, "Test".to_string(), 100, 100, 10);
        let team = Team::new("Test Team".to_string(), vec![character.clone()]);
        let battle_context = BattleContext::new(&character, TeamSide::Player, &team, &team);
        let eval_context = EvaluationContext::new(&battle_context);
        
        let char1 = Character::new(1, "Character 1".to_string(), 100, 100, 10);
        let char2 = Character::new(2, "Character 2".to_string(), 100, 100, 10);
        let char3 = Character::new(3, "Character 3".to_string(), 100, 100, 10);
        
        let char_hp1 = CharacterHP::from_character_with_hp(char1, 30);
        let char_hp2 = CharacterHP::from_character_with_hp(char2, 80); // 最大
        let char_hp3 = CharacterHP::from_character_with_hp(char3, 60);
        
        let array_node = Box::new(ConstantCharacterHPArrayNode::new(vec![char_hp1, char_hp2, char_hp3]));
        let max_node = MaxCharacterHPNode::new(array_node);
        
        let result = max_node.evaluate(&eval_context, &mut rng).unwrap();
        assert_eq!(result.get_hp(), 80);
        assert_eq!(result.get_character().id, 2);
    }

    #[test]
    fn test_max_character_hp_node_empty_array() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(12345);
        
        let character = Character::new(1, "Test".to_string(), 100, 100, 10);
        let team = Team::new("Test Team".to_string(), vec![character.clone()]);
        let battle_context = BattleContext::new(&character, TeamSide::Player, &team, &team);
        let eval_context = EvaluationContext::new(&battle_context);
        
        let array_node = Box::new(ConstantCharacterHPArrayNode::new(vec![]));
        let max_node = MaxCharacterHPNode::new(array_node);
        
        let result = max_node.evaluate(&eval_context, &mut rng);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Cannot find max of empty CharacterHP array"));
    }

    #[test]
    fn test_max_character_hp_node_single_element() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(12345);
        
        let character = Character::new(1, "Test".to_string(), 100, 100, 10);
        let team = Team::new("Test Team".to_string(), vec![character.clone()]);
        let battle_context = BattleContext::new(&character, TeamSide::Player, &team, &team);
        let eval_context = EvaluationContext::new(&battle_context);
        
        let char1 = Character::new(1, "Solo Character".to_string(), 100, 100, 10);
        let char_hp1 = CharacterHP::from_character_with_hp(char1, 45);
        
        let array_node = Box::new(ConstantCharacterHPArrayNode::new(vec![char_hp1]));
        let max_node = MaxCharacterHPNode::new(array_node);
        
        let result = max_node.evaluate(&eval_context, &mut rng).unwrap();
        assert_eq!(result.get_hp(), 45);
        assert_eq!(result.get_character().id, 1);
    }

    #[test]
    fn test_max_character_hp_node_same_hp_values() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(12345);
        
        let character = Character::new(1, "Test".to_string(), 100, 100, 10);
        let team = Team::new("Test Team".to_string(), vec![character.clone()]);
        let battle_context = BattleContext::new(&character, TeamSide::Player, &team, &team);
        let eval_context = EvaluationContext::new(&battle_context);
        
        let char1 = Character::new(1, "Character 1".to_string(), 100, 100, 10);
        let char2 = Character::new(2, "Character 2".to_string(), 100, 100, 10);
        let char3 = Character::new(3, "Character 3".to_string(), 100, 100, 10);
        
        let char_hp1 = CharacterHP::from_character_with_hp(char1, 50);
        let char_hp2 = CharacterHP::from_character_with_hp(char2, 50);
        let char_hp3 = CharacterHP::from_character_with_hp(char3, 50);
        
        let array_node = Box::new(ConstantCharacterHPArrayNode::new(vec![char_hp1, char_hp2, char_hp3]));
        let max_node = MaxCharacterHPNode::new(array_node);
        
        let result = max_node.evaluate(&eval_context, &mut rng).unwrap();
        assert_eq!(result.get_hp(), 50);
        // Should return one of the characters with max HP (deterministic based on the order)
        assert!(result.get_character().id >= 1 && result.get_character().id <= 3);
    }
}