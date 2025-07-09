use crate::nodes::unified_node::Node;
use crate::nodes::evaluation_context::EvaluationContext;
use crate::core::NodeResult;
use crate::core::character_hp::CharacterHP;

/// CharacterHP配列内の最小HP値を持つCharacterHPを返すノード
pub struct MinCharacterHPNode {
    character_hp_array_node: Box<dyn Node<Vec<CharacterHP>>>,
}

impl MinCharacterHPNode {
    pub fn new(character_hp_array_node: Box<dyn Node<Vec<CharacterHP>>>) -> Self {
        Self { character_hp_array_node }
    }
}

impl Node<CharacterHP> for MinCharacterHPNode {
    fn evaluate(&self, eval_context: &EvaluationContext, rng: &mut dyn rand::RngCore) -> NodeResult<CharacterHP> {
        let character_hp_array = self.character_hp_array_node.evaluate(eval_context, rng)?;
        
        if character_hp_array.is_empty() {
            return Err(crate::NodeError::EvaluationError("Cannot find min of empty CharacterHP array".to_string()));
        }
        
        // CharacterHPはOrdを実装しているので、minで最小値を取得可能
        let min_character_hp = character_hp_array.iter().min().unwrap();
        Ok(min_character_hp.clone())
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
    fn test_min_character_hp_node_basic() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(12345);
        
        let character = Character::new(1, "Test".to_string(), 100, 100, 10);
        let team = Team::new("Test Team".to_string(), vec![character.clone()]);
        let battle_context = BattleContext::new(&character, TeamSide::Player, &team, &team);
        let eval_context = EvaluationContext::new(&battle_context);
        
        // 異なるHPを持つCharacterHPを作成
        let char1 = Character::new(1, "High HP".to_string(), 100, 100, 10);
        let char2 = Character::new(2, "Low HP".to_string(), 100, 100, 10);
        let char3 = Character::new(3, "Medium HP".to_string(), 100, 100, 10);
        
        let char_hp1 = CharacterHP::from_character_with_hp(char1, 80);
        let char_hp2 = CharacterHP::from_character_with_hp(char2, 30); // 最小
        let char_hp3 = CharacterHP::from_character_with_hp(char3, 60);
        
        let array_node = Box::new(ConstantCharacterHPArrayNode::new(vec![char_hp1, char_hp2, char_hp3]));
        let min_node = MinCharacterHPNode::new(array_node);
        
        let result = min_node.evaluate(&eval_context, &mut rng).unwrap();
        assert_eq!(result.get_hp(), 30);
        assert_eq!(result.get_character().id, 2);
        assert_eq!(result.get_character().name, "Low HP");
    }

    #[test]
    fn test_min_character_hp_node_single_element() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(12345);
        
        let character = Character::new(1, "Test".to_string(), 100, 100, 10);
        let team = Team::new("Test Team".to_string(), vec![character.clone()]);
        let battle_context = BattleContext::new(&character, TeamSide::Player, &team, &team);
        let eval_context = EvaluationContext::new(&battle_context);
        
        let single_char = Character::new(42, "Only One".to_string(), 100, 100, 10);
        let single_char_hp = CharacterHP::from_character_with_hp(single_char, 77);
        
        let array_node = Box::new(ConstantCharacterHPArrayNode::new(vec![single_char_hp]));
        let min_node = MinCharacterHPNode::new(array_node);
        
        let result = min_node.evaluate(&eval_context, &mut rng).unwrap();
        assert_eq!(result.get_hp(), 77);
        assert_eq!(result.get_character().id, 42);
    }

    #[test]
    fn test_min_character_hp_node_empty_array() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(12345);
        
        let character = Character::new(1, "Test".to_string(), 100, 100, 10);
        let team = Team::new("Test Team".to_string(), vec![character.clone()]);
        let battle_context = BattleContext::new(&character, TeamSide::Player, &team, &team);
        let eval_context = EvaluationContext::new(&battle_context);
        
        let array_node = Box::new(ConstantCharacterHPArrayNode::new(vec![]));
        let min_node = MinCharacterHPNode::new(array_node);
        
        let result = min_node.evaluate(&eval_context, &mut rng);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Cannot find min of empty CharacterHP array"));
    }

    #[test]
    fn test_min_character_hp_node_same_hp_values() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(12345);
        
        let character = Character::new(1, "Test".to_string(), 100, 100, 10);
        let team = Team::new("Test Team".to_string(), vec![character.clone()]);
        let battle_context = BattleContext::new(&character, TeamSide::Player, &team, &team);
        let eval_context = EvaluationContext::new(&battle_context);
        
        let char1 = Character::new(1, "Same HP 1".to_string(), 100, 100, 10);
        let char2 = Character::new(2, "Same HP 2".to_string(), 100, 100, 10);
        let char3 = Character::new(3, "Same HP 3".to_string(), 100, 100, 10);
        
        let char_hp1 = CharacterHP::from_character_with_hp(char1, 50);
        let char_hp2 = CharacterHP::from_character_with_hp(char2, 50);
        let char_hp3 = CharacterHP::from_character_with_hp(char3, 50);
        
        let array_node = Box::new(ConstantCharacterHPArrayNode::new(vec![char_hp1, char_hp2, char_hp3]));
        let min_node = MinCharacterHPNode::new(array_node);
        
        let result = min_node.evaluate(&eval_context, &mut rng).unwrap();
        assert_eq!(result.get_hp(), 50);
        // 最初に見つかったもの（通常は最初の要素）が返される
        assert_eq!(result.get_character().id, 1);
    }
}