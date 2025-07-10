// MappingNode - applies a transformation function to each element of an array
// Similar to JavaScript's Array.map() function

use crate::core::NodeResult;
use crate::nodes::unified_node::Node;
use crate::nodes::evaluation_context::{EvaluationContext, CurrentElement};
use crate::Character;

/// Generic MappingNode that maps an array of input type to an array of output type
pub struct MappingNode<TInput, TOutput> {
    /// The array node to map over
    array_node: Box<dyn Node<Vec<TInput>>>,
    /// The transformation function to apply to each element
    transform_node: Box<dyn Node<TOutput>>,
}

impl<TInput, TOutput> MappingNode<TInput, TOutput> {
    pub fn new(
        array_node: Box<dyn Node<Vec<TInput>>>,
        transform_node: Box<dyn Node<TOutput>>,
    ) -> Self {
        Self {
            array_node,
            transform_node,
        }
    }
}

/// Character to Character mapping
pub type CharacterToCharacterMappingNode = MappingNode<Character, Character>;

/// Character to Value mapping
pub type CharacterToValueMappingNode = MappingNode<Character, i32>;

/// Value to Value mapping
pub type ValueToValueMappingNode = MappingNode<i32, i32>;

/// Value to Character mapping
pub type ValueToCharacterMappingNode = MappingNode<i32, Character>;

// Implementation for Character to Character mapping
impl Node<Vec<Character>> for CharacterToCharacterMappingNode {
    fn evaluate(&self, eval_context: &EvaluationContext, rng: &mut dyn rand::RngCore) -> NodeResult<Vec<Character>> {
        // Get the input array
        let input_array = self.array_node.evaluate(eval_context, rng)?;
        
        let mut output_array = Vec::new();
        
        // Apply the transformation to each element
        for element in input_array {
            // Create an evaluation context with the current element
            let element_eval_context = eval_context.with_new_current_element(CurrentElement::Character(element));
            
            // Apply the transformation function
            let transformed_element = self.transform_node.evaluate(&element_eval_context, rng)?;
            
            output_array.push(transformed_element);
        }
        
        Ok(output_array)
    }
}

// Implementation for Character to Value mapping
impl Node<Vec<i32>> for CharacterToValueMappingNode {
    fn evaluate(&self, eval_context: &EvaluationContext, rng: &mut dyn rand::RngCore) -> NodeResult<Vec<i32>> {
        // Get the input array
        let input_array = self.array_node.evaluate(eval_context, rng)?;
        
        let mut output_array = Vec::new();
        
        // Apply the transformation to each element
        for element in input_array {
            // Create an evaluation context with the current element
            let element_eval_context = eval_context.with_new_current_element(CurrentElement::Character(element));
            
            // Apply the transformation function
            let transformed_element = self.transform_node.evaluate(&element_eval_context, rng)?;
            
            output_array.push(transformed_element);
        }
        
        Ok(output_array)
    }
}

// Implementation for Value to Value mapping
impl Node<Vec<i32>> for ValueToValueMappingNode {
    fn evaluate(&self, eval_context: &EvaluationContext, rng: &mut dyn rand::RngCore) -> NodeResult<Vec<i32>> {
        // Get the input array
        let input_array = self.array_node.evaluate(eval_context, rng)?;
        
        let mut output_array = Vec::new();
        
        // Apply the transformation to each element
        for element in input_array {
            // Create an evaluation context with the current element
            let element_eval_context = eval_context.with_new_current_element(CurrentElement::Value(element));
            
            // Apply the transformation function
            let transformed_element = self.transform_node.evaluate(&element_eval_context, rng)?;
            
            output_array.push(transformed_element);
        }
        
        Ok(output_array)
    }
}

// Implementation for Value to Character mapping
impl Node<Vec<Character>> for ValueToCharacterMappingNode {
    fn evaluate(&self, eval_context: &EvaluationContext, rng: &mut dyn rand::RngCore) -> NodeResult<Vec<Character>> {
        // Get the input array
        let input_array = self.array_node.evaluate(eval_context, rng)?;
        
        let mut output_array = Vec::new();
        
        // Apply the transformation to each element
        for element in input_array {
            // Create an evaluation context with the current element
            let element_eval_context = eval_context.with_new_current_element(CurrentElement::Value(element));
            
            // Apply the transformation function
            let transformed_element = self.transform_node.evaluate(&element_eval_context, rng)?;
            
            output_array.push(transformed_element);
        }
        
        Ok(output_array)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::nodes::array::team_members_node::TeamMembersNode;
    use crate::nodes::array::constant_array_node::ConstantArrayNode;
    use crate::nodes::character::element_node::ElementNode;
    use crate::nodes::character::character_hp_value_node::CharacterHpValueNode;
    use crate::nodes::value::constant_value_node::ConstantValueNode;
    use crate::nodes::character::acting_character_node::ActingCharacterNode;
    use crate::{BattleContext, Team, TeamSide};
    use rand::SeedableRng;

    #[test]
    fn test_character_to_value_mapping() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(12345);
        
        // Create characters with different HP values
        let mut char1 = Character::new(1, "Char1".to_string(), 100, 100, 10);
        char1.hp = 50;
        let mut char2 = Character::new(2, "Char2".to_string(), 100, 100, 15);
        char2.hp = 75;
        let mut char3 = Character::new(3, "Char3".to_string(), 100, 100, 12);
        char3.hp = 30;
        
        let player_team = Team::new("Player".to_string(), vec![char1.clone(), char2.clone(), char3.clone()]);
        let enemy_team = Team::new("Enemy".to_string(), vec![]);
        
        let battle_context = BattleContext::new(&char1, TeamSide::Player, &player_team, &enemy_team);
        
        // Create mapping that extracts HP from each character
        let team_array = Box::new(TeamMembersNode::new(TeamSide::Player));
        let hp_extractor = Box::new(CharacterHpValueNode::new(Box::new(ElementNode::new())));
        
        let mapping_node = CharacterToValueMappingNode::new(team_array, hp_extractor);
        
        let eval_context = EvaluationContext::new(&battle_context);
        let result = Node::<Vec<i32>>::evaluate(&mapping_node, &eval_context, &mut rng).unwrap();
        
        // Should return [50, 75, 30] - the HP values of the characters
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], 50);
        assert_eq!(result[1], 75);
        assert_eq!(result[2], 30);
    }

    #[test]
    fn test_value_to_value_mapping() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(12345);
        
        let char1 = Character::new(1, "Test".to_string(), 100, 100, 10);
        let player_team = Team::new("Player".to_string(), vec![char1.clone()]);
        let enemy_team = Team::new("Enemy".to_string(), vec![]);
        
        let battle_context = BattleContext::new(&char1, TeamSide::Player, &player_team, &enemy_team);
        
        // Create mapping that doubles each value
        let values = vec![10, 20, 30];
        let value_array = Box::new(ConstantArrayNode::new(values));
        
        // Create a node that returns Element * 2 (we'll use a simple constant for testing)
        let double_transform = Box::new(ConstantValueNode::new(42)); // For simplicity
        
        let mapping_node = ValueToValueMappingNode::new(value_array, double_transform);
        
        let eval_context = EvaluationContext::new(&battle_context);
        let result = Node::<Vec<i32>>::evaluate(&mapping_node, &eval_context, &mut rng).unwrap();
        
        // Should return [42, 42, 42] since we're using a constant transform
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], 42);
        assert_eq!(result[1], 42);
        assert_eq!(result[2], 42);
    }

    #[test]
    fn test_character_to_character_mapping() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(12345);
        
        // Create characters
        let char1 = Character::new(1, "Char1".to_string(), 100, 100, 10);
        let char2 = Character::new(2, "Char2".to_string(), 100, 100, 15);
        
        let player_team = Team::new("Player".to_string(), vec![char1.clone(), char2.clone()]);
        let enemy_team = Team::new("Enemy".to_string(), vec![]);
        
        let battle_context = BattleContext::new(&char1, TeamSide::Player, &player_team, &enemy_team);
        
        // Create mapping that returns the acting character for each element
        let team_array = Box::new(TeamMembersNode::new(TeamSide::Player));
        let acting_char_transform = Box::new(ActingCharacterNode);
        
        let mapping_node = CharacterToCharacterMappingNode::new(team_array, acting_char_transform);
        
        let eval_context = EvaluationContext::new(&battle_context);
        let result = Node::<Vec<Character>>::evaluate(&mapping_node, &eval_context, &mut rng).unwrap();
        
        // Should return [acting_char, acting_char] - the acting character for each element
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].id, 1); // Acting character is char1
        assert_eq!(result[1].id, 1); // Acting character is char1
    }

    #[test]
    fn test_mapping_empty_array() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(12345);
        
        let char1 = Character::new(1, "Test".to_string(), 100, 100, 10);
        let player_team = Team::new("Player".to_string(), vec![char1.clone()]);
        let enemy_team = Team::new("Enemy".to_string(), vec![]);
        
        let battle_context = BattleContext::new(&char1, TeamSide::Player, &player_team, &enemy_team);
        
        // Create mapping with empty array
        let empty_array = Box::new(ConstantArrayNode::new(vec![]));
        let transform = Box::new(ConstantValueNode::new(42));
        
        let mapping_node = ValueToValueMappingNode::new(empty_array, transform);
        
        let eval_context = EvaluationContext::new(&battle_context);
        let result = Node::<Vec<i32>>::evaluate(&mapping_node, &eval_context, &mut rng).unwrap();
        
        // Should return empty array
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_mapping_node_boxed() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(12345);
        
        let char1 = Character::new(1, "Test".to_string(), 100, 100, 10);
        let player_team = Team::new("Player".to_string(), vec![char1.clone()]);
        let enemy_team = Team::new("Enemy".to_string(), vec![]);
        
        let battle_context = BattleContext::new(&char1, TeamSide::Player, &player_team, &enemy_team);
        
        // Test as boxed trait object
        let values = vec![10, 20];
        let value_array = Box::new(ConstantArrayNode::new(values));
        let transform = Box::new(ConstantValueNode::new(99));
        
        let mapping_node: Box<dyn Node<Vec<i32>>> = Box::new(ValueToValueMappingNode::new(value_array, transform));
        
        let eval_context = EvaluationContext::new(&battle_context);
        let result = mapping_node.evaluate(&eval_context, &mut rng).unwrap();
        
        // Should return [99, 99]
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], 99);
        assert_eq!(result[1], 99);
    }
}