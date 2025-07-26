// Element node - references the current element being processed in array operations
use crate::core::NodeResult;
use crate::nodes::unified_node::CoreNode as Node;
use crate::nodes::evaluation_context::EvaluationContext;
use std::convert::TryInto;
use std::marker::PhantomData;

/// Generic node that returns the current element being processed in array operations
/// This is typically used within FilterList conditions to reference the element being evaluated
#[derive(Debug)]
pub struct ElementNode<T> {
    phantom: PhantomData<T>,
}

impl<T> ElementNode<T> {
    pub fn new() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}

impl<T> Default for ElementNode<T> {
    fn default() -> Self {
        Self::new()
    }
}

// Generic implementation for any type that can be converted from UnknownValue
impl<'a, T> Node<T, EvaluationContext<'a>> for ElementNode<T>
where
    T: TryFrom<crate::nodes::unknown_value::UnknownValue> + Clone + Send + Sync + 'static,
    T::Error: std::fmt::Display,
{
    fn evaluate(&self, eval_context: &mut EvaluationContext) -> NodeResult<T> {
        match &eval_context.current_element {
            Some(value) => {
                value.clone()
                    .try_into()
                    .map_err(|e| crate::core::NodeError::EvaluationError(
                        format!("Current element type mismatch: {}", e)
                    ))
            }
            None => Err(crate::core::NodeError::EvaluationError(
                "No current element available - ElementNode requires array context".to_string()
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Character, Team, TeamSide};
    use crate::{BattleContext};
    use crate::nodes::evaluation_context::EvaluationContext;
    use crate::nodes::unified_node::BoxedNode;
    use crate::nodes::unknown_value::UnknownValue;
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
        let element_node = ElementNode::<Character>::new();
        
        let mut rng1 = rng.clone();
        let mut eval_context1 = EvaluationContext::new(&battle_context1, &mut rng1);
        eval_context1.current_element = Some(UnknownValue::Character(character2.clone()));
        let result1: crate::Character = element_node.evaluate(&mut eval_context1).unwrap();
        assert_eq!(result1.id, 2); // Should return character2 (current element)
        assert_eq!(result1.hp, 75);
        
        // Test with character2 as acting character, character1 as current element
        let battle_context2 = BattleContext::new(&character2, TeamSide::Enemy, &player_team, &enemy_team);
        let mut eval_context2 = EvaluationContext::new(&battle_context2, &mut rng);
        eval_context2.current_element = Some(UnknownValue::Character(character1.clone()));
        
        let result2: crate::Character = element_node.evaluate(&mut eval_context2).unwrap();
        assert_eq!(result2.id, 1); // Should return character1 (current element)
        assert_eq!(result2.hp, 50);
    }
    

    #[test]
    fn test_element_node_with_different_types() {
        let rng = rand::rngs::StdRng::seed_from_u64(12345);
        
        let character = Character::new(1, "Test".to_string(), 100, 100, 20);
        let team = Team::new("Test Team".to_string(), vec![character.clone()]);
        let battle_context = BattleContext::new(&character, TeamSide::Player, &team, &team);
        
        // Test with Value element - should succeed with Node<i32>
        let mut rng2 = rng.clone();
        let mut value_eval_context = EvaluationContext::new(&battle_context, &mut rng2);
        value_eval_context.current_element = Some(UnknownValue::Value(42));
        let element_node_i32 = ElementNode::<i32>::new();
        let value_result: NodeResult<i32> = element_node_i32.evaluate(&mut value_eval_context);
        assert!(value_result.is_ok(), "ElementNode should succeed with Value element");
        assert_eq!(value_result.unwrap(), 42);
        
        // Test with TeamSide element - should succeed with Node<TeamSide>
        let mut rng3 = rng.clone();
        let mut team_eval_context = EvaluationContext::new(&battle_context, &mut rng3);
        team_eval_context.current_element = Some(UnknownValue::TeamSide(TeamSide::Enemy));
        let element_node_team = ElementNode::<TeamSide>::new();
        let team_result: NodeResult<crate::TeamSide> = element_node_team.evaluate(&mut team_eval_context);
        assert!(team_result.is_ok(), "ElementNode should succeed with TeamSide element");
        assert_eq!(team_result.unwrap(), TeamSide::Enemy);
        
        // Test with CharacterHP element - should succeed with Node<CharacterHP>
        let mut rng4 = rng.clone();
        let character_hp = crate::core::character_hp::CharacterHP::new(character.clone());
        let mut hp_eval_context = EvaluationContext::new(&battle_context, &mut rng4);
        hp_eval_context.current_element = Some(UnknownValue::CharacterHP(character_hp.clone()));
        let element_node_hp = ElementNode::<crate::core::character_hp::CharacterHP>::new();
        let hp_result: NodeResult<crate::core::character_hp::CharacterHP> = element_node_hp.evaluate(&mut hp_eval_context);
        assert!(hp_result.is_ok(), "ElementNode should succeed with CharacterHP element");
        assert_eq!(hp_result.unwrap().hp_value, character_hp.hp_value);
        
        // Test type mismatches - should fail
        let element_node_char = ElementNode::<Character>::new();
        let value_as_char_result: NodeResult<crate::Character> = element_node_char.evaluate(&mut value_eval_context);
        assert!(value_as_char_result.is_err(), "ElementNode should fail when expecting Character but got Value");
        
        let team_as_value_result: NodeResult<i32> = element_node_i32.evaluate(&mut team_eval_context);
        assert!(team_as_value_result.is_err(), "ElementNode should fail when expecting Value but got TeamSide");
    }

    #[test]
    fn test_element_node_unified() {
        let rng = rand::rngs::StdRng::seed_from_u64(12345);
        
        let character = Character::new(99, "Unified Test".to_string(), 100, 100, 30);
        let current_element = Character::new(123, "Element".to_string(), 80, 80, 25);
        let team = Team::new("Unified Team".to_string(), vec![character.clone(), current_element.clone()]);
        let battle_context = BattleContext::new(&character, TeamSide::Player, &team, &team);
        
        let element_node = ElementNode::<Character>::new();
        let mut rng4 = rng.clone();
        let mut eval_context = EvaluationContext::new(&battle_context, &mut rng4);
        eval_context.current_element = Some(UnknownValue::Character(current_element.clone()));
        
        // Test unified implementation
        let result: crate::Character = element_node.evaluate(&mut eval_context).unwrap();
        assert_eq!(result.id, 123);
        
        // Test as boxed trait object
        let boxed_node: BoxedNode<crate::Character> = Box::new(ElementNode::<Character>::default());
        let boxed_result = boxed_node.evaluate(&mut eval_context).unwrap();
        assert_eq!(boxed_result.id, 123);
        
        // Test with new element context
        let element_character = Character::new(456, "NewElement".to_string(), 60, 60, 20);
        let mut element_eval_context = eval_context.with_current_element_from_context(UnknownValue::Character(element_character.clone()));
        let element_result: crate::Character = element_node.evaluate(&mut element_eval_context).unwrap();
        assert_eq!(element_result.id, 456); // Should return new element character
    }
}