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

// Unified implementation for Character
impl Node<crate::Character> for ElementNode {
    fn evaluate(&self, eval_context: &mut crate::nodes::evaluation_context::EvaluationContext) -> NodeResult<crate::Character> {
        // Return the current character being processed (current element in array operations)
        match &eval_context.current_element {
            Some(crate::nodes::evaluation_context::CurrentElement::Character(character)) => {
                Ok(character.clone())
            }
            Some(_) => Err(crate::core::NodeError::EvaluationError("Current element is not a Character".to_string())),
            None => Err(crate::core::NodeError::EvaluationError("No current element available - ElementNode requires array context".to_string()))
        }
    }
}

// Implementation for Value
impl Node<i32> for ElementNode {
    fn evaluate(&self, eval_context: &mut crate::nodes::evaluation_context::EvaluationContext) -> NodeResult<i32> {
        match &eval_context.current_element {
            Some(crate::nodes::evaluation_context::CurrentElement::Value(value)) => {
                Ok(*value)
            }
            Some(_) => Err(crate::core::NodeError::EvaluationError("Current element is not a Value".to_string())),
            None => Err(crate::core::NodeError::EvaluationError("No current element available - ElementNode requires array context".to_string()))
        }
    }
}

// Implementation for TeamSide
impl Node<crate::TeamSide> for ElementNode {
    fn evaluate(&self, eval_context: &mut crate::nodes::evaluation_context::EvaluationContext) -> NodeResult<crate::TeamSide> {
        match &eval_context.current_element {
            Some(crate::nodes::evaluation_context::CurrentElement::TeamSide(team_side)) => {
                Ok(*team_side)
            }
            Some(_) => Err(crate::core::NodeError::EvaluationError("Current element is not a TeamSide".to_string())),
            None => Err(crate::core::NodeError::EvaluationError("No current element available - ElementNode requires array context".to_string()))
        }
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
    fn test_element_node_returns_current_element() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(12345);
        
        let mut character1 = Character::new(1, "Test1".to_string(), 100, 100, 10);
        character1.hp = 50;
        let mut character2 = Character::new(2, "Test2".to_string(), 100, 100, 15);
        character2.hp = 75;
        
        let player_team = Team::new("Player".to_string(), vec![character1.clone()]);
        let enemy_team = Team::new("Enemy".to_string(), vec![character2.clone()]);
        
        // Test with character1 as acting character, character2 as current element
        let battle_context1 = BattleContext::new(&character1, TeamSide::Player, &player_team, &enemy_team);
        let element_node = ElementNode::new();
        
        let mut rng1 = rng.clone();
        let mut eval_context1 = EvaluationContext::with_element(&battle_context1, &character2, &mut rng1);
        let result1 = Node::<crate::Character>::evaluate(&element_node, &mut eval_context1).unwrap();
        assert_eq!(result1.id, 2); // Should return character2 (current element)
        assert_eq!(result1.hp, 75);
        
        // Test with character2 as acting character, character1 as current element
        let battle_context2 = BattleContext::new(&character2, TeamSide::Enemy, &player_team, &enemy_team);
        let mut eval_context2 = EvaluationContext::with_element(&battle_context2, &character1, &mut rng);
        
        let result2 = Node::<crate::Character>::evaluate(&element_node, &mut eval_context2).unwrap();
        assert_eq!(result2.id, 1); // Should return character1 (current element)
        assert_eq!(result2.hp, 50);
    }
    

    #[test]
    fn test_element_node_with_different_types() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(12345);
        
        let character = Character::new(1, "Test".to_string(), 100, 100, 20);
        let team = Team::new("Test Team".to_string(), vec![character.clone()]);
        let battle_context = BattleContext::new(&character, TeamSide::Player, &team, &team);
        
        let element_node = ElementNode::new();
        
        // Test with Value element - should succeed with Node<i32>
        let mut rng2 = rng.clone();
        let mut value_eval_context = EvaluationContext::with_value_element(&battle_context, 42, &mut rng2);
        let value_result = Node::<i32>::evaluate(&element_node, &mut value_eval_context);
        assert!(value_result.is_ok(), "ElementNode should succeed with Value element");
        assert_eq!(value_result.unwrap(), 42);
        
        // Test with TeamSide element - should succeed with Node<TeamSide>
        let mut rng3 = rng.clone();
        let mut team_eval_context = EvaluationContext::with_team_side_element(&battle_context, TeamSide::Enemy, &mut rng3);
        let team_result = Node::<crate::TeamSide>::evaluate(&element_node, &mut team_eval_context);
        assert!(team_result.is_ok(), "ElementNode should succeed with TeamSide element");
        assert_eq!(team_result.unwrap(), TeamSide::Enemy);
        
        // Test type mismatches - should fail
        let value_as_char_result = Node::<crate::Character>::evaluate(&element_node, &mut value_eval_context);
        assert!(value_as_char_result.is_err(), "ElementNode should fail when expecting Character but got Value");
        
        let team_as_value_result = Node::<i32>::evaluate(&element_node, &mut team_eval_context);
        assert!(team_as_value_result.is_err(), "ElementNode should fail when expecting Value but got TeamSide");
    }

    #[test]
    fn test_element_node_unified() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(12345);
        
        let character = Character::new(99, "Unified Test".to_string(), 100, 100, 30);
        let current_element = Character::new(123, "Element".to_string(), 80, 80, 25);
        let team = Team::new("Unified Team".to_string(), vec![character.clone(), current_element.clone()]);
        let battle_context = BattleContext::new(&character, TeamSide::Player, &team, &team);
        
        let element_node = ElementNode::new();
        let mut rng4 = rng.clone();
        let mut eval_context = EvaluationContext::with_element(&battle_context, &current_element, &mut rng4);
        
        // Test unified implementation
        let result = Node::<crate::Character>::evaluate(&element_node, &mut eval_context).unwrap();
        assert_eq!(result.id, 123);
        
        // Test as boxed trait object
        let boxed_node: Box<dyn Node<crate::Character>> = Box::new(ElementNode::default());
        let boxed_result = boxed_node.evaluate(&mut eval_context).unwrap();
        assert_eq!(boxed_result.id, 123);
        
        // Test with new element context
        let element_character = Character::new(456, "NewElement".to_string(), 60, 60, 20);
        let mut element_eval_context = eval_context.with_element_from_context(&element_character);
        let element_result = Node::<crate::Character>::evaluate(&element_node, &mut element_eval_context).unwrap();
        assert_eq!(element_result.id, 456); // Should return new element character
    }
}