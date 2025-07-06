// Unified Node<T> trait - single generic trait for all node types

use crate::core::NodeResult;
use crate::nodes::evaluation_context::EvaluationContext;

/// Unified generic trait for all node types
/// Replaces CharacterNode, ValueNode, ConditionNode, and ArrayNode<T>
pub trait Node<T>: Send + Sync {
    fn evaluate(&self, eval_context: &EvaluationContext, rng: &mut dyn rand::RngCore) -> NodeResult<T>;
}

// Box implementation for trait objects
impl<T> Node<T> for Box<dyn Node<T> + Send + Sync> {
    fn evaluate(&self, eval_context: &EvaluationContext, rng: &mut dyn rand::RngCore) -> NodeResult<T> {
        (**self).evaluate(eval_context, rng)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Character, Team, TeamSide, BattleContext};
    use rand::SeedableRng;

    // Test struct implementing Node<i32>
    #[derive(Debug)]
    struct TestCharacterNode {
        character_id: i32,
    }

    impl Node<i32> for TestCharacterNode {
        fn evaluate(&self, _eval_context: &EvaluationContext, _rng: &mut dyn rand::RngCore) -> NodeResult<i32> {
            Ok(self.character_id)
        }
    }

    // Test struct implementing Node<bool>
    #[derive(Debug)]
    struct TestConditionNode {
        result: bool,
    }

    impl Node<bool> for TestConditionNode {
        fn evaluate(&self, _eval_context: &EvaluationContext, _rng: &mut dyn rand::RngCore) -> NodeResult<bool> {
            Ok(self.result)
        }
    }

    // Test struct implementing Node<Vec<Character>>
    #[derive(Debug)]
    struct TestArrayNode {
        characters: Vec<Character>,
    }

    impl Node<Vec<Character>> for TestArrayNode {
        fn evaluate(&self, _eval_context: &EvaluationContext, _rng: &mut dyn rand::RngCore) -> NodeResult<Vec<Character>> {
            Ok(self.characters.clone())
        }
    }

    #[test]
    fn test_unified_node_character() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(12345);
        
        let char1 = Character::new(1, "Test".to_string(), 100, 100, 10);
        let team = Team::new("Team".to_string(), vec![char1.clone()]);
        let battle_context = BattleContext::new(&char1, TeamSide::Player, &team, &team);
        let eval_context = EvaluationContext::new(&battle_context);
        
        let char_node = TestCharacterNode { character_id: 42 };
        let result = char_node.evaluate(&eval_context, &mut rng).unwrap();
        assert_eq!(result, 42);
    }

    #[test]
    fn test_unified_node_condition() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(12345);
        
        let char1 = Character::new(1, "Test".to_string(), 100, 100, 10);
        let team = Team::new("Team".to_string(), vec![char1.clone()]);
        let battle_context = BattleContext::new(&char1, TeamSide::Player, &team, &team);
        let eval_context = EvaluationContext::new(&battle_context);
        
        let condition_node = TestConditionNode { result: true };
        let result = condition_node.evaluate(&eval_context, &mut rng).unwrap();
        assert_eq!(result, true);
    }

    #[test]
    fn test_unified_node_array() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(12345);
        
        let char1 = Character::new(1, "Test1".to_string(), 100, 100, 10);
        let char2 = Character::new(2, "Test2".to_string(), 80, 80, 15);
        let characters = vec![char1.clone(), char2.clone()];
        
        let team = Team::new("Team".to_string(), vec![char1.clone()]);
        let battle_context = BattleContext::new(&char1, TeamSide::Player, &team, &team);
        let eval_context = EvaluationContext::new(&battle_context);
        
        let array_node = TestArrayNode { characters: characters.clone() };
        let result = array_node.evaluate(&eval_context, &mut rng).unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].id, 1);
        assert_eq!(result[1].id, 2);
    }

    #[test]
    fn test_unified_node_boxed() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(12345);
        
        let char1 = Character::new(1, "Test".to_string(), 100, 100, 10);
        let team = Team::new("Team".to_string(), vec![char1.clone()]);
        let battle_context = BattleContext::new(&char1, TeamSide::Player, &team, &team);
        let eval_context = EvaluationContext::new(&battle_context);
        
        // Test boxed trait objects
        let char_node: Box<dyn Node<i32>> = Box::new(TestCharacterNode { character_id: 99 });
        let condition_node: Box<dyn Node<bool>> = Box::new(TestConditionNode { result: false });
        
        let char_result = char_node.evaluate(&eval_context, &mut rng).unwrap();
        let condition_result = condition_node.evaluate(&eval_context, &mut rng).unwrap();
        
        assert_eq!(char_result, 99);
        assert_eq!(condition_result, false);
    }
}