use crate::nodes::unified_node::Node;
use crate::nodes::evaluation_context::EvaluationContext;
use crate::core::{NodeResult, Numeric};

/// Generic GreaterThan node that works with any Numeric type
pub struct GreaterThanNode<T: Numeric> {
    left_node: Box<dyn Node<T>>,
    right_node: Box<dyn Node<T>>,
}

impl<T: Numeric> GreaterThanNode<T> {
    pub fn new(left_node: Box<dyn Node<T>>, right_node: Box<dyn Node<T>>) -> Self {
        Self { left_node, right_node }
    }
}

impl<T: Numeric> Node<bool> for GreaterThanNode<T> {
    fn evaluate(&self, eval_context: &EvaluationContext, rng: &mut dyn rand::RngCore) -> NodeResult<bool> {
        let left_value = self.left_node.evaluate(eval_context, rng)?;
        let right_value = self.right_node.evaluate(eval_context, rng)?;
        
        Ok(left_value.to_i32() > right_value.to_i32())
    }
}

/// Mixed type GreaterThan node for CharacterHP vs i32
pub struct CharacterHpVsValueGreaterThanNode {
    left_node: Box<dyn Node<crate::core::CharacterHP>>,
    right_node: Box<dyn Node<i32>>,
}

impl CharacterHpVsValueGreaterThanNode {
    pub fn new(left_node: Box<dyn Node<crate::core::CharacterHP>>, right_node: Box<dyn Node<i32>>) -> Self {
        Self { left_node, right_node }
    }
}

impl Node<bool> for CharacterHpVsValueGreaterThanNode {
    fn evaluate(&self, eval_context: &EvaluationContext, rng: &mut dyn rand::RngCore) -> NodeResult<bool> {
        let left_value = self.left_node.evaluate(eval_context, rng)?;
        let right_value = self.right_node.evaluate(eval_context, rng)?;
        
        Ok(left_value.to_i32() > right_value)
    }
}

/// Mixed type GreaterThan node for i32 vs CharacterHP
pub struct ValueVsCharacterHpGreaterThanNode {
    left_node: Box<dyn Node<i32>>,
    right_node: Box<dyn Node<crate::core::CharacterHP>>,
}

impl ValueVsCharacterHpGreaterThanNode {
    pub fn new(left_node: Box<dyn Node<i32>>, right_node: Box<dyn Node<crate::core::CharacterHP>>) -> Self {
        Self { left_node, right_node }
    }
}

impl Node<bool> for ValueVsCharacterHpGreaterThanNode {
    fn evaluate(&self, eval_context: &EvaluationContext, rng: &mut dyn rand::RngCore) -> NodeResult<bool> {
        let left_value = self.left_node.evaluate(eval_context, rng)?;
        let right_value = self.right_node.evaluate(eval_context, rng)?;
        
        Ok(left_value > right_value.to_i32())
    }
}

// Type aliases for convenience
pub type GreaterThanConditionNode = GreaterThanNode<i32>;
pub type CharacterHpVsValueConditionNode = CharacterHpVsValueGreaterThanNode;
pub type ValueVsCharacterHpConditionNode = ValueVsCharacterHpGreaterThanNode;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Character, CharacterHP};
    use crate::nodes::character::BattleContext;
    use crate::nodes::value::ConstantValueNode;
    use crate::TeamSide;
    use crate::Team;
    use rand::SeedableRng;

    // Test helper for constant CharacterHP
    struct ConstantCharacterHPNode {
        character_hp: CharacterHP,
    }

    impl ConstantCharacterHPNode {
        fn new(character_hp: CharacterHP) -> Self {
            Self { character_hp }
        }
    }

    impl Node<CharacterHP> for ConstantCharacterHPNode {
        fn evaluate(&self, _eval_context: &EvaluationContext, _rng: &mut dyn rand::RngCore) -> NodeResult<CharacterHP> {
            Ok(self.character_hp.clone())
        }
    }

    #[test]
    fn test_greater_than_i32() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(12345);
        
        let character = Character::new(1, "Test".to_string(), 100, 100, 10);
        let team = Team::new("Test Team".to_string(), vec![character.clone()]);
        let battle_context = BattleContext::new(&character, TeamSide::Player, &team, &team);
        let eval_context = EvaluationContext::new(&battle_context);
        
        let left_node = Box::new(ConstantValueNode::new(50));
        let right_node = Box::new(ConstantValueNode::new(30));
        let gt_node = GreaterThanConditionNode::new(left_node, right_node);
        
        let result = gt_node.evaluate(&eval_context, &mut rng).unwrap();
        assert_eq!(result, true);
    }

    #[test]
    fn test_greater_than_i32_false() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(12345);
        
        let character = Character::new(1, "Test".to_string(), 100, 100, 10);
        let team = Team::new("Test Team".to_string(), vec![character.clone()]);
        let battle_context = BattleContext::new(&character, TeamSide::Player, &team, &team);
        let eval_context = EvaluationContext::new(&battle_context);
        
        let left_node = Box::new(ConstantValueNode::new(20));
        let right_node = Box::new(ConstantValueNode::new(30));
        let gt_node = GreaterThanConditionNode::new(left_node, right_node);
        
        let result = gt_node.evaluate(&eval_context, &mut rng).unwrap();
        assert_eq!(result, false);
    }

    #[test]
    fn test_character_hp_vs_value_greater_than() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(12345);
        
        let character = Character::new(1, "Test".to_string(), 100, 100, 10);
        let team = Team::new("Test Team".to_string(), vec![character.clone()]);
        let battle_context = BattleContext::new(&character, TeamSide::Player, &team, &team);
        let eval_context = EvaluationContext::new(&battle_context);
        
        let test_char = Character::new(1, "TestChar".to_string(), 100, 100, 10);
        let char_hp = CharacterHP::from_character_with_hp(test_char, 80);
        
        let left_node = Box::new(ConstantCharacterHPNode::new(char_hp));
        let right_node = Box::new(ConstantValueNode::new(50));
        let gt_node = CharacterHpVsValueConditionNode::new(left_node, right_node);
        
        let result = gt_node.evaluate(&eval_context, &mut rng).unwrap();
        assert_eq!(result, true);
    }

    #[test]
    fn test_value_vs_character_hp_greater_than() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(12345);
        
        let character = Character::new(1, "Test".to_string(), 100, 100, 10);
        let team = Team::new("Test Team".to_string(), vec![character.clone()]);
        let battle_context = BattleContext::new(&character, TeamSide::Player, &team, &team);
        let eval_context = EvaluationContext::new(&battle_context);
        
        let test_char = Character::new(1, "TestChar".to_string(), 100, 100, 10);
        let char_hp = CharacterHP::from_character_with_hp(test_char, 30);
        
        let left_node = Box::new(ConstantValueNode::new(50));
        let right_node = Box::new(ConstantCharacterHPNode::new(char_hp));
        let gt_node = ValueVsCharacterHpConditionNode::new(left_node, right_node);
        
        let result = gt_node.evaluate(&eval_context, &mut rng).unwrap();
        assert_eq!(result, true);
    }

    #[test]
    fn test_character_hp_vs_character_hp_greater_than() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(12345);
        
        let character = Character::new(1, "Test".to_string(), 100, 100, 10);
        let team = Team::new("Test Team".to_string(), vec![character.clone()]);
        let battle_context = BattleContext::new(&character, TeamSide::Player, &team, &team);
        let eval_context = EvaluationContext::new(&battle_context);
        
        let test_char1 = Character::new(1, "TestChar1".to_string(), 100, 100, 10);
        let test_char2 = Character::new(2, "TestChar2".to_string(), 100, 100, 10);
        let char_hp1 = CharacterHP::from_character_with_hp(test_char1, 80);
        let char_hp2 = CharacterHP::from_character_with_hp(test_char2, 60);
        
        let left_node = Box::new(ConstantCharacterHPNode::new(char_hp1));
        let right_node = Box::new(ConstantCharacterHPNode::new(char_hp2));
        let gt_node = GreaterThanNode::<CharacterHP>::new(left_node, right_node);
        
        let result = gt_node.evaluate(&eval_context, &mut rng).unwrap();
        assert_eq!(result, true);
    }
}