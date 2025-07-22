// MappingNode - applies a transformation function to each element of an array
// Similar to JavaScript's Array.map() function

use crate::core::NodeResult;
use crate::core::character_hp::CharacterHP;
use crate::nodes::unified_node::{CoreNode as Node, BoxedNode};
use crate::nodes::evaluation_context::{EvaluationContext, CurrentElement};
use crate::Character;

/// Generic MappingNode that maps an array of input type to an array of output type
pub struct MappingNode<TInput, TOutput> {
    /// The array node to map over
    array_node: BoxedNode<Vec<TInput>>,
    /// The transformation function to apply to each element
    transform_node: BoxedNode<TOutput>,
}

impl<TInput, TOutput> MappingNode<TInput, TOutput> {
    pub fn new(
        array_node: BoxedNode<Vec<TInput>>,
        transform_node: BoxedNode<TOutput>,
    ) -> Self {
        Self {
            array_node,
            transform_node,
        }
    }
}

/// Macro to generate MappingNode implementations
/// This approach scales better than manual type aliases while maintaining clarity
macro_rules! impl_mapping_for_types {
    // Base case: generate implementation for each type combination
    ($(($input_type:ty, $output_type:ty, $current_element_variant:ident)),* $(,)?) => {
        $(
            impl<'a> Node<Vec<$output_type>, EvaluationContext<'a>> for MappingNode<$input_type, $output_type> {
                fn evaluate(&self, eval_context: &mut EvaluationContext) -> NodeResult<Vec<$output_type>> {
                    // Get the input array
                    let input_array = self.array_node.evaluate(eval_context)?;
                    
                    let mut output_array = Vec::new();
                    
                    // Apply the transformation to each element
                    for element in input_array {
                        // Create an evaluation context with the current element
                        let mut element_eval_context = eval_context.with_current_element_from_context(CurrentElement::$current_element_variant(element));
                        
                        // Apply the transformation function
                        let transformed_element = self.transform_node.evaluate(&mut element_eval_context)?;
                        
                        output_array.push(transformed_element);
                    }
                    
                    Ok(output_array)
                }
            }
        )*
    };
}

// Register all supported type combinations here
// To add new types: just add them to this list and they'll work everywhere
impl_mapping_for_types! {
    (Character, Character, Character),
    (Character, i32, Character),
    (Character, CharacterHP, Character),
    (i32, i32, Value),
    (i32, Character, Value),
    (CharacterHP, Character, CharacterHP),
    // Future types go here - adding a new type here automatically supports it everywhere
    // Example: (TeamSide, TeamSide, TeamSide),
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::nodes::array::team_members_node::TeamMembersNode;
    // ConstantArrayNode removed - using direct values in tests
    use crate::nodes::character::element_node::ElementNode;
    use crate::nodes::character::character_hp_value_node::CharacterHpValueNode;
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
        
        let mapping_node = MappingNode::new(team_array, hp_extractor);
        
        let mut eval_context = EvaluationContext::new(&battle_context, &mut rng);
        let result = Node::<Vec<i32>, EvaluationContext>::evaluate(&mapping_node, &mut eval_context).unwrap();
        
        // Should return [50, 75, 30] - the HP values of the characters
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], 50);
        assert_eq!(result[1], 75);
        assert_eq!(result[2], 30);
    }

    // Removed test_value_to_value_mapping - ConstantArrayNode deleted

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
        
        let mapping_node = MappingNode::new(team_array, acting_char_transform);
        
        let mut eval_context = EvaluationContext::new(&battle_context, &mut rng);
        let result = Node::<Vec<Character>, EvaluationContext>::evaluate(&mapping_node, &mut eval_context).unwrap();
        
        // Should return [acting_char, acting_char] - the acting character for each element
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].id, 1); // Acting character is char1
        assert_eq!(result[1].id, 1); // Acting character is char1
    }

    // Removed test_mapping_empty_array - ConstantArrayNode deleted

    // Removed test_mapping_node_boxed - ConstantArrayNode deleted
}