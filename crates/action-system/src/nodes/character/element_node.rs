// Element node - references the current character being processed in array operations
use crate::core::NodeResult;
use crate::nodes::unified_node::Node;

/// Node that returns the current character being processed in array operations
/// This is typically used within FilterList conditions to reference the element being evaluated
#[derive(Debug)]
pub struct ElementNode;

impl ElementNode {
    pub fn new() -> Self {
        Self
    }
}

impl Default for ElementNode {
    fn default() -> Self {
        Self::new()
    }
}

// Unified implementation
impl Node<crate::Character> for ElementNode {
    fn evaluate(&self, eval_context: &crate::nodes::evaluation_context::EvaluationContext, _rng: &mut dyn rand::RngCore) -> NodeResult<crate::Character> {
        // Return the current character being processed (current element in array operations)
        Ok(eval_context.get_current_character().clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Character, Team, TeamSide};
    use crate::{BattleContext};
    use crate::nodes::evaluation_context::EvaluationContext;
    use rand::SeedableRng;

    #[test]
    fn test_element_node_returns_acting_character() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(12345);
        
        let mut character1 = Character::new(1, "Test1".to_string(), 100, 100, 10);
        character1.hp = 50;
        let mut character2 = Character::new(2, "Test2".to_string(), 100, 100, 15);
        character2.hp = 75;
        
        let player_team = Team::new("Player".to_string(), vec![character1.clone()]);
        let enemy_team = Team::new("Enemy".to_string(), vec![character2.clone()]);
        
        // Test with character1 as acting character
        let battle_context1 = BattleContext::new(&character1, TeamSide::Player, &player_team, &enemy_team);
        let element_node = ElementNode::new();
        
        let eval_context1 = EvaluationContext::new(&battle_context1);
        let result1 = Node::<crate::Character>::evaluate(&element_node, &eval_context1, &mut rng).unwrap();
        assert_eq!(result1.id, 1); // Should return character1
        assert_eq!(result1.hp, 50);
        
        // Test with character2 as acting character
        let battle_context2 = BattleContext::new(&character2, TeamSide::Enemy, &player_team, &enemy_team);
        let eval_context2 = EvaluationContext::new(&battle_context2);
        
        let result2 = Node::<crate::Character>::evaluate(&element_node, &eval_context2, &mut rng).unwrap();
        assert_eq!(result2.id, 2); // Should return character2
        assert_eq!(result2.hp, 75);
    }
    
    #[test]
    fn test_element_node_default_constructor() {
        let element_node = ElementNode::default();
        let character = Character::new(42, "Test".to_string(), 100, 100, 20);
        let team = Team::new("Test Team".to_string(), vec![character.clone()]);
        let battle_context = BattleContext::new(&character, TeamSide::Player, &team, &team);
        
        let mut rng = rand::rngs::StdRng::seed_from_u64(12345);
        let eval_context = EvaluationContext::new(&battle_context);
        let result = Node::<crate::Character>::evaluate(&element_node, &eval_context, &mut rng).unwrap();
        assert_eq!(result.id, 42);
    }

    #[test]
    fn test_element_node_unified() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(12345);
        
        let character = Character::new(99, "Unified Test".to_string(), 100, 100, 30);
        let team = Team::new("Unified Team".to_string(), vec![character.clone()]);
        let battle_context = BattleContext::new(&character, TeamSide::Player, &team, &team);
        
        let element_node = ElementNode::new();
        let eval_context = EvaluationContext::new(&battle_context);
        
        // Test unified implementation
        let result = Node::<crate::Character>::evaluate(&element_node, &eval_context, &mut rng).unwrap();
        assert_eq!(result.id, 99);
        
        // Test as boxed trait object
        let boxed_node: Box<dyn Node<crate::Character>> = Box::new(ElementNode::default());
        let boxed_result = boxed_node.evaluate(&eval_context, &mut rng).unwrap();
        assert_eq!(boxed_result.id, 99);
        
        // Test with new element context
        let element_character = Character::new(123, "Element".to_string(), 80, 80, 25);
        let element_eval_context = eval_context.with_new_element(&element_character);
        let element_result = Node::<crate::Character>::evaluate(&element_node, &element_eval_context, &mut rng).unwrap();
        assert_eq!(element_result.id, 123); // Should return element character
    }
}