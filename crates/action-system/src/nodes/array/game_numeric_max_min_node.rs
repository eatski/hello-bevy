use crate::nodes::unified_node::Node;
use crate::nodes::evaluation_context::EvaluationContext;
use crate::core::{NodeResult, GameNumeric};

/// Generic Max node that works with any GameNumeric type
pub struct GameNumericMaxNode<T: GameNumeric> {
    array_node: Box<dyn Node<Vec<T>>>,
}

impl<T: GameNumeric> GameNumericMaxNode<T> {
    pub fn new(array_node: Box<dyn Node<Vec<T>>>) -> Self {
        Self { array_node }
    }
}

impl<T: GameNumeric> Node<T> for GameNumericMaxNode<T> {
    fn evaluate(&self, eval_context: &EvaluationContext, rng: &mut dyn rand::RngCore) -> NodeResult<T> {
        let array = self.array_node.evaluate(eval_context, rng)?;
        
        if array.is_empty() {
            return Err(crate::core::NodeError::EvaluationError("Cannot find max of empty array".to_string()));
        }
        
        let mut max_value = array[0].clone();
        for item in array.iter().skip(1) {
            max_value = max_value.max(item.clone());
        }
        
        Ok(max_value)
    }
}

/// Generic Min node that works with any GameNumeric type
pub struct GameNumericMinNode<T: GameNumeric> {
    array_node: Box<dyn Node<Vec<T>>>,
}

impl<T: GameNumeric> GameNumericMinNode<T> {
    pub fn new(array_node: Box<dyn Node<Vec<T>>>) -> Self {
        Self { array_node }
    }
}

impl<T: GameNumeric> Node<T> for GameNumericMinNode<T> {
    fn evaluate(&self, eval_context: &EvaluationContext, rng: &mut dyn rand::RngCore) -> NodeResult<T> {
        let array = self.array_node.evaluate(eval_context, rng)?;
        
        if array.is_empty() {
            return Err(crate::core::NodeError::EvaluationError("Cannot find min of empty array".to_string()));
        }
        
        let mut min_value = array[0].clone();
        for item in array.iter().skip(1) {
            min_value = min_value.min(item.clone());
        }
        
        Ok(min_value)
    }
}

// Type aliases for convenience
pub type MaxNode = GameNumericMaxNode<i32>;
pub type MinNode = GameNumericMinNode<i32>;
pub type MaxCharacterHPNode = GameNumericMaxNode<crate::core::CharacterHP>;
pub type MinCharacterHPNode = GameNumericMinNode<crate::core::CharacterHP>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Character, CharacterHP};
    use crate::nodes::character::BattleContext;
    use crate::TeamSide;
    use crate::Team;
    use rand::SeedableRng;

    // Test helper for constant value arrays
    struct ConstantValueArrayNode {
        values: Vec<i32>,
    }

    impl ConstantValueArrayNode {
        fn new(values: Vec<i32>) -> Self {
            Self { values }
        }
    }

    impl Node<Vec<i32>> for ConstantValueArrayNode {
        fn evaluate(&self, _eval_context: &EvaluationContext, _rng: &mut dyn rand::RngCore) -> NodeResult<Vec<i32>> {
            Ok(self.values.clone())
        }
    }

    // Test helper for constant CharacterHP arrays
    struct ConstantCharacterHPArrayNode {
        character_hps: Vec<CharacterHP>,
    }

    impl ConstantCharacterHPArrayNode {
        fn new(character_hps: Vec<CharacterHP>) -> Self {
            Self { character_hps }
        }
    }

    impl Node<Vec<CharacterHP>> for ConstantCharacterHPArrayNode {
        fn evaluate(&self, _eval_context: &EvaluationContext, _rng: &mut dyn rand::RngCore) -> NodeResult<Vec<CharacterHP>> {
            Ok(self.character_hps.clone())
        }
    }

    #[test]
    fn test_max_node_i32() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(12345);
        
        let character = Character::new(1, "Test".to_string(), 100, 100, 10);
        let team = Team::new("Test Team".to_string(), vec![character.clone()]);
        let battle_context = BattleContext::new(&character, TeamSide::Player, &team, &team);
        let eval_context = EvaluationContext::new(&battle_context);
        
        let values = vec![10, 50, 30, 20];
        let array_node = Box::new(ConstantValueArrayNode::new(values));
        let max_node = MaxNode::new(array_node);
        
        let result = max_node.evaluate(&eval_context, &mut rng).unwrap();
        assert_eq!(result, 50);
    }

    #[test]
    fn test_min_node_i32() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(12345);
        
        let character = Character::new(1, "Test".to_string(), 100, 100, 10);
        let team = Team::new("Test Team".to_string(), vec![character.clone()]);
        let battle_context = BattleContext::new(&character, TeamSide::Player, &team, &team);
        let eval_context = EvaluationContext::new(&battle_context);
        
        let values = vec![10, 50, 30, 20];
        let array_node = Box::new(ConstantValueArrayNode::new(values));
        let min_node = MinNode::new(array_node);
        
        let result = min_node.evaluate(&eval_context, &mut rng).unwrap();
        assert_eq!(result, 10);
    }

    #[test]
    fn test_max_character_hp_node() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(12345);
        
        let character = Character::new(1, "Test".to_string(), 100, 100, 10);
        let team = Team::new("Test Team".to_string(), vec![character.clone()]);
        let battle_context = BattleContext::new(&character, TeamSide::Player, &team, &team);
        let eval_context = EvaluationContext::new(&battle_context);
        
        let char1 = Character::new(1, "Char1".to_string(), 100, 100, 10);
        let char2 = Character::new(2, "Char2".to_string(), 100, 100, 10);
        let char3 = Character::new(3, "Char3".to_string(), 100, 100, 10);
        
        let hp1 = CharacterHP::from_character_with_hp(char1, 80);
        let hp2 = CharacterHP::from_character_with_hp(char2, 60);
        let hp3 = CharacterHP::from_character_with_hp(char3, 90);
        
        let character_hps = vec![hp1, hp2, hp3];
        let array_node = Box::new(ConstantCharacterHPArrayNode::new(character_hps));
        let max_node = MaxCharacterHPNode::new(array_node);
        
        let result = max_node.evaluate(&eval_context, &mut rng).unwrap();
        assert_eq!(result.get_hp(), 90);
        assert_eq!(result.get_character().id, 3);
    }

    #[test]
    fn test_min_character_hp_node() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(12345);
        
        let character = Character::new(1, "Test".to_string(), 100, 100, 10);
        let team = Team::new("Test Team".to_string(), vec![character.clone()]);
        let battle_context = BattleContext::new(&character, TeamSide::Player, &team, &team);
        let eval_context = EvaluationContext::new(&battle_context);
        
        let char1 = Character::new(1, "Char1".to_string(), 100, 100, 10);
        let char2 = Character::new(2, "Char2".to_string(), 100, 100, 10);
        let char3 = Character::new(3, "Char3".to_string(), 100, 100, 10);
        
        let hp1 = CharacterHP::from_character_with_hp(char1, 80);
        let hp2 = CharacterHP::from_character_with_hp(char2, 60);
        let hp3 = CharacterHP::from_character_with_hp(char3, 90);
        
        let character_hps = vec![hp1, hp2, hp3];
        let array_node = Box::new(ConstantCharacterHPArrayNode::new(character_hps));
        let min_node = MinCharacterHPNode::new(array_node);
        
        let result = min_node.evaluate(&eval_context, &mut rng).unwrap();
        assert_eq!(result.get_hp(), 60);
        assert_eq!(result.get_character().id, 2);
    }

    #[test]
    fn test_empty_array_error() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(12345);
        
        let character = Character::new(1, "Test".to_string(), 100, 100, 10);
        let team = Team::new("Test Team".to_string(), vec![character.clone()]);
        let battle_context = BattleContext::new(&character, TeamSide::Player, &team, &team);
        let eval_context = EvaluationContext::new(&battle_context);
        
        let empty_array_node = Box::new(ConstantValueArrayNode::new(vec![]));
        let max_node = MaxNode::new(empty_array_node);
        
        let result = max_node.evaluate(&eval_context, &mut rng);
        assert!(result.is_err());
    }
}