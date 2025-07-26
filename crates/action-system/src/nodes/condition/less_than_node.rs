use crate::nodes::unified_node::{CoreNode as Node, BoxedNode};
use crate::core::{NodeResult, Numeric};
use crate::nodes::evaluation_context::EvaluationContext;

/// Generic LessThan node that works with different Numeric types on left and right
pub struct LessThanNode<L: Numeric, R: Numeric> {
    left_node: BoxedNode<L>,
    right_node: BoxedNode<R>,
}

impl<L: Numeric, R: Numeric> LessThanNode<L, R> {
    pub fn new(left_node: BoxedNode<L>, right_node: BoxedNode<R>) -> Self {
        Self { left_node, right_node }
    }
}

impl<'a, L: Numeric, R: Numeric> Node<bool, EvaluationContext<'a>> for LessThanNode<L, R> {
    fn evaluate(&self, eval_context: &mut EvaluationContext) -> NodeResult<bool> {
        let left_value = self.left_node.evaluate(eval_context)?;
        let right_value = self.right_node.evaluate(eval_context)?;
        
        Ok(left_value.to_i32() < right_value.to_i32())
    }
}


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

    impl<'a> Node<CharacterHP, EvaluationContext<'a>> for ConstantCharacterHPNode {
        fn evaluate(&self, _eval_context: &mut EvaluationContext) -> NodeResult<CharacterHP> {
            Ok(self.character_hp.clone())
        }
    }

    #[test]
    fn test_less_than_i32() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(12345);
        
        let character = Character::new(1, "Test".to_string(), 100, 100, 10);
        let team = Team::new("Test Team".to_string(), vec![character.clone()]);
        let battle_context = BattleContext::new(&character, TeamSide::Player, &team, &team);
        let mut eval_context = EvaluationContext::new(&battle_context, &mut rng);
        
        let left_node = Box::new(ConstantValueNode::new(30));
        let right_node = Box::new(ConstantValueNode::new(50));
        let lt_node = LessThanNode::<i32, i32>::new(left_node, right_node);
        
        let result = lt_node.evaluate(&mut eval_context).unwrap();
        assert_eq!(result, true);
    }

    #[test]
    fn test_less_than_i32_false() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(12345);
        
        let character = Character::new(1, "Test".to_string(), 100, 100, 10);
        let team = Team::new("Test Team".to_string(), vec![character.clone()]);
        let battle_context = BattleContext::new(&character, TeamSide::Player, &team, &team);
        let mut eval_context = EvaluationContext::new(&battle_context, &mut rng);
        
        let left_node = Box::new(ConstantValueNode::new(50));
        let right_node = Box::new(ConstantValueNode::new(30));
        let lt_node = LessThanNode::<i32, i32>::new(left_node, right_node);
        
        let result = lt_node.evaluate(&mut eval_context).unwrap();
        assert_eq!(result, false);
    }

    #[test]
    fn test_character_hp_vs_value_less_than() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(12345);
        
        let character = Character::new(1, "Test".to_string(), 100, 100, 10);
        let team = Team::new("Test Team".to_string(), vec![character.clone()]);
        let battle_context = BattleContext::new(&character, TeamSide::Player, &team, &team);
        let mut eval_context = EvaluationContext::new(&battle_context, &mut rng);
        
        let test_char = Character::new(1, "TestChar".to_string(), 100, 100, 10);
        let char_hp = CharacterHP::from_character_with_hp(test_char, 30);
        
        let left_node = Box::new(ConstantCharacterHPNode::new(char_hp));
        let right_node = Box::new(ConstantValueNode::new(50));
        let lt_node = LessThanNode::<CharacterHP, i32>::new(left_node, right_node);
        
        let result = lt_node.evaluate(&mut eval_context).unwrap();
        assert_eq!(result, true);
    }

    #[test]
    fn test_value_vs_character_hp_less_than() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(12345);
        
        let character = Character::new(1, "Test".to_string(), 100, 100, 10);
        let team = Team::new("Test Team".to_string(), vec![character.clone()]);
        let battle_context = BattleContext::new(&character, TeamSide::Player, &team, &team);
        let mut eval_context = EvaluationContext::new(&battle_context, &mut rng);
        
        let test_char = Character::new(1, "TestChar".to_string(), 100, 100, 10);
        let char_hp = CharacterHP::from_character_with_hp(test_char, 80);
        
        let left_node = Box::new(ConstantValueNode::new(50));
        let right_node = Box::new(ConstantCharacterHPNode::new(char_hp));
        let lt_node = LessThanNode::<i32, CharacterHP>::new(left_node, right_node);
        
        let result = lt_node.evaluate(&mut eval_context).unwrap();
        assert_eq!(result, true);
    }

    #[test]
    fn test_character_hp_vs_character_hp_less_than() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(12345);
        
        let character = Character::new(1, "Test".to_string(), 100, 100, 10);
        let team = Team::new("Test Team".to_string(), vec![character.clone()]);
        let battle_context = BattleContext::new(&character, TeamSide::Player, &team, &team);
        let mut eval_context = EvaluationContext::new(&battle_context, &mut rng);
        
        let test_char1 = Character::new(1, "TestChar1".to_string(), 100, 100, 10);
        let test_char2 = Character::new(2, "TestChar2".to_string(), 100, 100, 10);
        let char_hp1 = CharacterHP::from_character_with_hp(test_char1, 60);
        let char_hp2 = CharacterHP::from_character_with_hp(test_char2, 80);
        
        let left_node = Box::new(ConstantCharacterHPNode::new(char_hp1));
        let right_node = Box::new(ConstantCharacterHPNode::new(char_hp2));
        let lt_node = LessThanNode::<CharacterHP, CharacterHP>::new(left_node, right_node);
        
        let result = lt_node.evaluate(&mut eval_context).unwrap();
        assert_eq!(result, true);
    }
}