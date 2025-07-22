// Type aliases for action-system specific Node traits

use crate::nodes::evaluation_context::EvaluationContext;

// Re-export the core Node trait
pub use node_core::Node as CoreNode;
pub use node_core::{NodeResult, NodeError};

/// Type alias for Node trait with action-system's EvaluationContext
pub type Node<T> = dyn for<'a> CoreNode<T, EvaluationContext<'a>> + Send + Sync;

/// Type alias for boxed Node trait objects
pub type BoxedNode<T> = Box<Node<T>>;

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

    impl<'a> CoreNode<i32, EvaluationContext<'a>> for TestCharacterNode {
        fn evaluate(&self, _eval_context: &mut EvaluationContext<'a>) -> NodeResult<i32> {
            Ok(self.character_id)
        }
    }

    // Test struct implementing Node<bool>
    #[derive(Debug)]
    struct TestConditionNode {
        result: bool,
    }

    impl<'a> CoreNode<bool, EvaluationContext<'a>> for TestConditionNode {
        fn evaluate(&self, _eval_context: &mut EvaluationContext<'a>) -> NodeResult<bool> {
            Ok(self.result)
        }
    }

    // Test struct implementing Node<Vec<Character>
    #[derive(Debug)]
    struct TestArrayNode {
        characters: Vec<Character>,
    }

    impl<'a> CoreNode<Vec<Character>, EvaluationContext<'a>> for TestArrayNode {
        fn evaluate(&self, _eval_context: &mut EvaluationContext<'a>) -> NodeResult<Vec<Character>> {
            Ok(self.characters.clone())
        }
    }

    #[test]
    fn test_unified_node_character() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(12345);
        
        let char1 = Character::new(1, "Test".to_string(), 100, 100, 10);
        let team = Team::new("Team".to_string(), vec![char1.clone()]);
        let battle_context = BattleContext::new(&char1, TeamSide::Player, &team, &team);
        let mut eval_context = EvaluationContext::new(&battle_context, &mut rng);
        
        let char_node = TestCharacterNode { character_id: 42 };
        let result = char_node.evaluate(&mut eval_context).unwrap();
        assert_eq!(result, 42);
    }

    #[test]
    fn test_unified_node_condition() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(12345);
        
        let char1 = Character::new(1, "Test".to_string(), 100, 100, 10);
        let team = Team::new("Team".to_string(), vec![char1.clone()]);
        let battle_context = BattleContext::new(&char1, TeamSide::Player, &team, &team);
        let mut eval_context = EvaluationContext::new(&battle_context, &mut rng);
        
        let condition_node = TestConditionNode { result: true };
        let result = condition_node.evaluate(&mut eval_context).unwrap();
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
        let mut eval_context = EvaluationContext::new(&battle_context, &mut rng);
        
        let array_node = TestArrayNode { characters: characters.clone() };
        let result = array_node.evaluate(&mut eval_context).unwrap();
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
        let mut eval_context = EvaluationContext::new(&battle_context, &mut rng);
        
        // Test boxed trait objects
        let char_node: Box<dyn for<'a> CoreNode<i32, EvaluationContext<'a>>> = Box::new(TestCharacterNode { character_id: 99 });
        let condition_node: Box<dyn for<'a> CoreNode<bool, EvaluationContext<'a>>> = Box::new(TestConditionNode { result: false });
        
        let char_result = char_node.evaluate(&mut eval_context).unwrap();
        let condition_result = condition_node.evaluate(&mut eval_context).unwrap();
        
        assert_eq!(char_result, 99);
        assert_eq!(condition_result, false);
    }
}