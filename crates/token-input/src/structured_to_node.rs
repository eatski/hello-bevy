// StructuredTokenInput → Node 変換

use crate::{StructuredTokenInput, RuleSet};
use action_system::*;
use std::any::Any;

// パース結果を表すAnyベースのResolver
pub struct ParsedResolver {
    pub node: Box<dyn Any>,
    pub type_name: String,
}

impl ParsedResolver {
    pub fn new<T: Any + 'static>(node: T, type_name: String) -> Self {
        Self {
            node: Box::new(node),
            type_name,
        }
    }
}

// StructuredTokenInput → Node 変換
pub fn convert_structured_to_node(token: &StructuredTokenInput) -> Result<ParsedResolver, String> {
    match token {
        // Action nodes
        StructuredTokenInput::Strike { target } => {
            let target_node = convert_to_character_node(target)?;
            let strike_node = StrikeActionNode::new(target_node);
            Ok(ParsedResolver::new(Box::new(strike_node) as Box<dyn Node<Box<dyn Action>>>, "Action".to_string()))
        }
        StructuredTokenInput::Heal { target } => {
            let target_node = convert_to_character_node(target)?;
            let heal_node = HealActionNode::new(target_node);
            Ok(ParsedResolver::new(Box::new(heal_node) as Box<dyn Node<Box<dyn Action>>>, "Action".to_string()))
        }
        
        // Condition nodes
        StructuredTokenInput::Check { condition, then_action } => {
            let condition_node = convert_to_bool_node(condition)?;
            let action_parsed = convert_structured_to_node(then_action)?;
            let action_node = action_parsed.node.downcast::<Box<dyn Node<Box<dyn Action>>>>()
                .map_err(|_| "Check requires an Action node for then_action".to_string())?;
            let check_node = ConditionCheckNode::new(condition_node, *action_node);
            Ok(ParsedResolver::new(Box::new(check_node) as Box<dyn Node<Box<dyn Action>>>, "Action".to_string()))
        }
        StructuredTokenInput::TrueOrFalseRandom => {
            let random_node = RandomConditionNode;
            Ok(ParsedResolver::new(Box::new(random_node) as Box<dyn Node<bool>>, "bool".to_string()))
        }
        StructuredTokenInput::GreaterThan { left, right } => {
            convert_greater_than_node(left, right)
        }
        StructuredTokenInput::Eq { left, right } => {
            convert_eq_node(left, right)
        }
        
        // Value nodes
        StructuredTokenInput::Number { value } => {
            let value_node = ConstantValueNode::new(*value);
            Ok(ParsedResolver::new(Box::new(value_node) as Box<dyn Node<i32>>, "i32".to_string()))
        }
        
        // Character nodes
        StructuredTokenInput::ActingCharacter => {
            let acting_node = ActingCharacterNode;
            Ok(ParsedResolver::new(Box::new(acting_node) as Box<dyn Node<Character>>, "Character".to_string()))
        }
        StructuredTokenInput::Element => {
            let element_node = ElementNode::new();
            Ok(ParsedResolver::new(Box::new(element_node) as Box<dyn Node<Character>>, "Character".to_string()))
        }
        StructuredTokenInput::CharacterHpToCharacter { character_hp } => {
            let hp_node = convert_to_character_hp_node(character_hp)?;
            let char_node = CharacterHpToCharacterNode::new(hp_node);
            Ok(ParsedResolver::new(Box::new(char_node) as Box<dyn Node<Character>>, "Character".to_string()))
        }
        
        // CharacterHP nodes
        StructuredTokenInput::CharacterToHp { character } => {
            let char_node = convert_to_character_node(character)?;
            let hp_node = CharacterToHpNode::new(char_node);
            Ok(ParsedResolver::new(Box::new(hp_node) as Box<dyn Node<CharacterHP>>, "CharacterHP".to_string()))
        }
        
        // Array nodes
        StructuredTokenInput::AllCharacters => {
            let all_chars_node = AllCharactersNode::new();
            Ok(ParsedResolver::new(Box::new(all_chars_node) as Box<dyn Node<Vec<Character>>>, "Vec<Character>".to_string()))
        }
        StructuredTokenInput::TeamMembers { team_side } => {
            match convert_structured_to_node(team_side)? {
                ParsedResolver { node, type_name } if type_name == "TeamSide" => {
                    let team_side_node = node.downcast::<Box<dyn Node<TeamSide>>>()
                        .map_err(|_| "Expected TeamSide node".to_string())?;
                    let team_members_node = TeamMembersNode::new_with_node(*team_side_node);
                    Ok(ParsedResolver::new(Box::new(team_members_node) as Box<dyn Node<Vec<Character>>>, "Vec<Character>".to_string()))
                }
                _ => {
                    // Try to get static TeamSide value from token
                    match team_side.as_ref() {
                        StructuredTokenInput::Enemy => {
                            let team_members_node = TeamMembersNode::new(TeamSide::Enemy);
                            Ok(ParsedResolver::new(Box::new(team_members_node) as Box<dyn Node<Vec<Character>>>, "Vec<Character>".to_string()))
                        }
                        StructuredTokenInput::Hero => {
                            let team_members_node = TeamMembersNode::new(TeamSide::Player);
                            Ok(ParsedResolver::new(Box::new(team_members_node) as Box<dyn Node<Vec<Character>>>, "Vec<Character>".to_string()))
                        }
                        _ => Err("TeamMembers requires a TeamSide node or constant".to_string())
                    }
                }
            }
        }
        StructuredTokenInput::AllTeamSides => {
            let all_sides_node = AllTeamSidesNode::new();
            Ok(ParsedResolver::new(Box::new(all_sides_node) as Box<dyn Node<Vec<TeamSide>>>, "Vec<TeamSide>".to_string()))
        }
        StructuredTokenInput::RandomPick { array } => {
            convert_random_pick_node(array)
        }
        StructuredTokenInput::FilterList { array, condition } => {
            let array_parsed = convert_structured_to_node(array)?;
            let condition_node = convert_to_bool_node(condition)?;
            
            if array_parsed.type_name == "Vec<Character>" {
                let array_node = array_parsed.node.downcast::<Box<dyn Node<Vec<Character>>>>()
                    .map_err(|_| "Expected Vec<Character> node".to_string())?;
                let filter_node = FilterListNode::new(*array_node, condition_node);
                Ok(ParsedResolver::new(Box::new(filter_node) as Box<dyn Node<Vec<Character>>>, "Vec<Character>".to_string()))
            } else {
                Err(format!("FilterList not supported for array type: {}", array_parsed.type_name))
            }
        }
        StructuredTokenInput::Map { array, transform } => {
            convert_map_node(array, transform)
        }
        StructuredTokenInput::Max { array } => {
            convert_max_node(array, false)
        }
        StructuredTokenInput::Min { array } => {
            convert_min_node(array, false)
        }
        StructuredTokenInput::NumericMax { array } => {
            convert_max_node(array, true)
        }
        StructuredTokenInput::NumericMin { array } => {
            convert_min_node(array, true)
        }
        
        // TeamSide nodes
        StructuredTokenInput::Enemy => {
            let enemy_node = EnemyNode;
            Ok(ParsedResolver::new(Box::new(enemy_node) as Box<dyn Node<TeamSide>>, "TeamSide".to_string()))
        }
        StructuredTokenInput::Hero => {
            let hero_node = HeroNode;
            Ok(ParsedResolver::new(Box::new(hero_node) as Box<dyn Node<TeamSide>>, "TeamSide".to_string()))
        }
        StructuredTokenInput::CharacterTeam { character } => {
            let char_node = convert_to_character_node(character)?;
            let team_node = CharacterTeamNode::new(char_node);
            Ok(ParsedResolver::new(Box::new(team_node) as Box<dyn Node<TeamSide>>, "TeamSide".to_string()))
        }
    }
}

// Helper functions for node conversion
fn convert_to_character_node(token: &StructuredTokenInput) -> Result<Box<dyn Node<Character>>, String> {
    let parsed = convert_structured_to_node(token)?;
    if parsed.type_name == "Character" {
        parsed.node.downcast::<Box<dyn Node<Character>>>()
            .map(|n| *n)
            .map_err(|_| "Failed to downcast to Character node".to_string())
    } else {
        Err(format!("Expected Character node, got {}", parsed.type_name))
    }
}

fn convert_to_character_hp_node(token: &StructuredTokenInput) -> Result<Box<dyn Node<CharacterHP>>, String> {
    let parsed = convert_structured_to_node(token)?;
    if parsed.type_name == "CharacterHP" {
        parsed.node.downcast::<Box<dyn Node<CharacterHP>>>()
            .map(|n| *n)
            .map_err(|_| "Failed to downcast to CharacterHP node".to_string())
    } else {
        Err(format!("Expected CharacterHP node, got {}", parsed.type_name))
    }
}

fn convert_to_bool_node(token: &StructuredTokenInput) -> Result<Box<dyn Node<bool>>, String> {
    let parsed = convert_structured_to_node(token)?;
    if parsed.type_name == "bool" {
        parsed.node.downcast::<Box<dyn Node<bool>>>()
            .map(|n| *n)
            .map_err(|_| "Failed to downcast to bool node".to_string())
    } else {
        Err(format!("Expected bool node, got {}", parsed.type_name))
    }
}

fn convert_to_i32_node(token: &StructuredTokenInput) -> Result<Box<dyn Node<i32>>, String> {
    let parsed = convert_structured_to_node(token)?;
    if parsed.type_name == "i32" {
        parsed.node.downcast::<Box<dyn Node<i32>>>()
            .map(|n| *n)
            .map_err(|_| "Failed to downcast to i32 node".to_string())
    } else {
        Err(format!("Expected i32 node, got {}", parsed.type_name))
    }
}


fn convert_greater_than_node(left: &StructuredTokenInput, right: &StructuredTokenInput) -> Result<ParsedResolver, String> {
    // Determine the types of left and right for proper comparison
    match (left, right) {
        // Both are numbers
        (StructuredTokenInput::Number { .. }, StructuredTokenInput::Number { .. }) => {
            let left_node = convert_to_i32_node(left)?;
            let right_node = convert_to_i32_node(right)?;
            let gt_node = GreaterThanConditionNode::new(left_node, right_node);
            Ok(ParsedResolver::new(Box::new(gt_node) as Box<dyn Node<bool>>, "bool".to_string()))
        }
        // CharacterHP vs i32
        (StructuredTokenInput::CharacterToHp { .. }, StructuredTokenInput::Number { .. }) => {
            let left_node = convert_to_character_hp_node(left)?;
            let right_node = convert_to_i32_node(right)?;
            let gt_node = CharacterHpVsValueConditionNode::new(left_node, right_node);
            Ok(ParsedResolver::new(Box::new(gt_node) as Box<dyn Node<bool>>, "bool".to_string()))
        }
        // i32 vs CharacterHP
        (StructuredTokenInput::Number { .. }, StructuredTokenInput::CharacterToHp { .. }) => {
            let left_node = convert_to_i32_node(left)?;
            let right_node = convert_to_character_hp_node(right)?;
            let gt_node = ValueVsCharacterHpConditionNode::new(left_node, right_node);
            Ok(ParsedResolver::new(Box::new(gt_node) as Box<dyn Node<bool>>, "bool".to_string()))
        }
        // Both are CharacterHP
        (StructuredTokenInput::CharacterToHp { .. }, StructuredTokenInput::CharacterToHp { .. }) => {
            let left_node = convert_to_character_hp_node(left)?;
            let right_node = convert_to_character_hp_node(right)?;
            let gt_node = GreaterThanNode::<CharacterHP>::new(left_node, right_node);
            Ok(ParsedResolver::new(Box::new(gt_node) as Box<dyn Node<bool>>, "bool".to_string()))
        }
        // Try to infer types
        _ => {
            // Try as i32 first
            if let (Ok(left_i32), Ok(right_i32)) = (convert_to_i32_node(left), convert_to_i32_node(right)) {
                let gt_node = GreaterThanConditionNode::new(left_i32, right_i32);
                Ok(ParsedResolver::new(Box::new(gt_node) as Box<dyn Node<bool>>, "bool".to_string()))
            } else if let (Ok(left_hp), Ok(right_i32)) = (convert_to_character_hp_node(left), convert_to_i32_node(right)) {
                let gt_node = CharacterHpVsValueConditionNode::new(left_hp, right_i32);
                Ok(ParsedResolver::new(Box::new(gt_node) as Box<dyn Node<bool>>, "bool".to_string()))
            } else if let (Ok(left_i32), Ok(right_hp)) = (convert_to_i32_node(left), convert_to_character_hp_node(right)) {
                let gt_node = ValueVsCharacterHpConditionNode::new(left_i32, right_hp);
                Ok(ParsedResolver::new(Box::new(gt_node) as Box<dyn Node<bool>>, "bool".to_string()))
            } else if let (Ok(left_hp), Ok(right_hp)) = (convert_to_character_hp_node(left), convert_to_character_hp_node(right)) {
                let gt_node = GreaterThanNode::<CharacterHP>::new(left_hp, right_hp);
                Ok(ParsedResolver::new(Box::new(gt_node) as Box<dyn Node<bool>>, "bool".to_string()))
            } else {
                Err("GreaterThan requires numeric types (i32 or CharacterHP)".to_string())
            }
        }
    }
}

fn convert_eq_node(left: &StructuredTokenInput, right: &StructuredTokenInput) -> Result<ParsedResolver, String> {
    // Determine the type of equality comparison needed
    match (left, right) {
        // TeamSide comparisons
        (StructuredTokenInput::Enemy, _) | (StructuredTokenInput::Hero, _) |
        (_, StructuredTokenInput::Enemy) | (_, StructuredTokenInput::Hero) |
        (StructuredTokenInput::CharacterTeam { .. }, _) | (_, StructuredTokenInput::CharacterTeam { .. }) => {
            let left_parsed = convert_structured_to_node(left)?;
            let right_parsed = convert_structured_to_node(right)?;
            
            if left_parsed.type_name == "TeamSide" && right_parsed.type_name == "TeamSide" {
                let left_node = left_parsed.node.downcast::<Box<dyn Node<TeamSide>>>()
                    .map_err(|_| "Failed to downcast to TeamSide node".to_string())?;
                let right_node = right_parsed.node.downcast::<Box<dyn Node<TeamSide>>>()
                    .map_err(|_| "Failed to downcast to TeamSide node".to_string())?;
                let eq_node = TeamSideEqNode::new(*left_node, *right_node);
                Ok(ParsedResolver::new(Box::new(eq_node) as Box<dyn Node<bool>>, "bool".to_string()))
            } else {
                Err("Eq comparison requires matching types".to_string())
            }
        }
        // Add more type-specific equality comparisons as needed
        _ => Err("Eq comparison not implemented for these types".to_string())
    }
}

fn convert_random_pick_node(array: &StructuredTokenInput) -> Result<ParsedResolver, String> {
    let array_parsed = convert_structured_to_node(array)?;
    
    if array_parsed.type_name == "Vec<Character>" {
        let array_node = array_parsed.node.downcast::<Box<dyn Node<Vec<Character>>>>()
            .map_err(|_| "Expected Vec<Character> node".to_string())?;
        let random_node = CharacterRandomPickNode::new(*array_node);
        Ok(ParsedResolver::new(Box::new(random_node) as Box<dyn Node<Character>>, "Character".to_string()))
    } else {
        Err(format!("RandomPick not supported for array type: {}", array_parsed.type_name))
    }
}

fn convert_map_node(array: &StructuredTokenInput, transform: &StructuredTokenInput) -> Result<ParsedResolver, String> {
    let array_parsed = convert_structured_to_node(array)?;
    
    if array_parsed.type_name == "Vec<Character>" {
        let array_node = array_parsed.node.downcast::<Box<dyn Node<Vec<Character>>>>()
            .map_err(|_| "Expected Vec<Character> node".to_string())?;
        
        // Determine transform type
        match transform {
            StructuredTokenInput::CharacterToHp { .. } => {
                // Map characters to their HP values (as CharacterHP)
                let char_to_hp_node = CharacterToHpNode::new(Box::new(ElementNode::new()));
                let map_node = MappingNode::new(*array_node, Box::new(char_to_hp_node));
                Ok(ParsedResolver::new(Box::new(map_node) as Box<dyn Node<Vec<CharacterHP>>>, "Vec<CharacterHP>".to_string()))
            }
            _ => Err("Map transform not supported for this type".to_string())
        }
    } else {
        Err(format!("Map not supported for array type: {}", array_parsed.type_name))
    }
}

fn convert_max_node(array: &StructuredTokenInput, use_numeric: bool) -> Result<ParsedResolver, String> {
    let array_parsed = convert_structured_to_node(array)?;
    
    if array_parsed.type_name == "Vec<i32>" {
        let array_node = array_parsed.node.downcast::<Box<dyn Node<Vec<i32>>>>()
            .map_err(|_| "Expected Vec<i32> node".to_string())?;
        
        if use_numeric {
            // Use numeric max node (future extension point)
            let max_node = MaxNodeI32::new(*array_node);
            Ok(ParsedResolver::new(Box::new(max_node) as Box<dyn Node<i32>>, "i32".to_string()))
        } else {
            let max_node = MaxNodeI32::new(*array_node);
            Ok(ParsedResolver::new(Box::new(max_node) as Box<dyn Node<i32>>, "i32".to_string()))
        }
    } else if array_parsed.type_name == "Vec<CharacterHP>" {
        // For CharacterHP arrays, use MaxNode directly with CharacterHP
        let array_node = array_parsed.node.downcast::<Box<dyn Node<Vec<CharacterHP>>>>()
            .map_err(|_| "Expected Vec<CharacterHP> node".to_string())?;
        
        let max_node = MaxNode::<CharacterHP>::new(*array_node);
        Ok(ParsedResolver::new(Box::new(max_node) as Box<dyn Node<CharacterHP>>, "CharacterHP".to_string()))
    } else {
        Err(format!("Max not supported for array type: {}", array_parsed.type_name))
    }
}

fn convert_min_node(array: &StructuredTokenInput, use_numeric: bool) -> Result<ParsedResolver, String> {
    let array_parsed = convert_structured_to_node(array)?;
    
    if array_parsed.type_name == "Vec<i32>" {
        let array_node = array_parsed.node.downcast::<Box<dyn Node<Vec<i32>>>>()
            .map_err(|_| "Expected Vec<i32> node".to_string())?;
        
        if use_numeric {
            // Use numeric min node (future extension point)
            let min_node = MinNodeI32::new(*array_node);
            Ok(ParsedResolver::new(Box::new(min_node) as Box<dyn Node<i32>>, "i32".to_string()))
        } else {
            let min_node = MinNodeI32::new(*array_node);
            Ok(ParsedResolver::new(Box::new(min_node) as Box<dyn Node<i32>>, "i32".to_string()))
        }
    } else if array_parsed.type_name == "Vec<CharacterHP>" {
        // For CharacterHP arrays, use MinNode directly with CharacterHP
        let array_node = array_parsed.node.downcast::<Box<dyn Node<Vec<CharacterHP>>>>()
            .map_err(|_| "Expected Vec<CharacterHP> node".to_string())?;
        
        let min_node = MinNode::<CharacterHP>::new(*array_node);
        Ok(ParsedResolver::new(Box::new(min_node) as Box<dyn Node<CharacterHP>>, "CharacterHP".to_string()))
    } else {
        Err(format!("Min not supported for array type: {}", array_parsed.type_name))
    }
}

// RuleSet → Vec<RuleNode> 変換（JSON入力経路）
pub fn convert_ruleset_to_nodes(ruleset: &RuleSet) -> Vec<RuleNode> {
    ruleset.rules.iter()
        .filter_map(|token| {
            match convert_structured_to_node(token) {
                Ok(parsed) => {
                    if parsed.type_name == "Action" {
                        parsed.node.downcast::<Box<dyn Node<Box<dyn Action>>>>()
                            .ok()
                            .map(|node| *node as RuleNode)
                    } else {
                        None
                    }
                }
                Err(_) => None
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_convert_action_nodes() {
        // Test Strike
        let strike_token = StructuredTokenInput::Strike {
            target: Box::new(StructuredTokenInput::ActingCharacter),
        };
        let result = convert_structured_to_node(&strike_token).unwrap();
        assert_eq!(result.type_name, "Action");
        assert!(result.node.downcast::<Box<dyn Node<Box<dyn Action>>>>().is_ok());
        
        // Test Heal
        let heal_token = StructuredTokenInput::Heal {
            target: Box::new(StructuredTokenInput::RandomPick {
                array: Box::new(StructuredTokenInput::AllCharacters),
            }),
        };
        let result = convert_structured_to_node(&heal_token).unwrap();
        assert_eq!(result.type_name, "Action");
    }
    
    #[test]
    fn test_convert_condition_nodes() {
        // Test TrueOrFalseRandom
        let random_token = StructuredTokenInput::TrueOrFalseRandom;
        let result = convert_structured_to_node(&random_token).unwrap();
        assert_eq!(result.type_name, "bool");
        
        // Test GreaterThan with mixed types
        let gt_token = StructuredTokenInput::GreaterThan {
            left: Box::new(StructuredTokenInput::CharacterToHp {
                character: Box::new(StructuredTokenInput::ActingCharacter),
            }),
            right: Box::new(StructuredTokenInput::Number { value: 50 }),
        };
        let result = convert_structured_to_node(&gt_token).unwrap();
        assert_eq!(result.type_name, "bool");
        
        // Test Check
        let check_token = StructuredTokenInput::Check {
            condition: Box::new(StructuredTokenInput::TrueOrFalseRandom),
            then_action: Box::new(StructuredTokenInput::Strike {
                target: Box::new(StructuredTokenInput::ActingCharacter),
            }),
        };
        let result = convert_structured_to_node(&check_token).unwrap();
        assert_eq!(result.type_name, "Action");
    }
    
    #[test]
    fn test_convert_array_nodes() {
        // Test AllCharacters
        let all_chars_token = StructuredTokenInput::AllCharacters;
        let result = convert_structured_to_node(&all_chars_token).unwrap();
        assert_eq!(result.type_name, "Vec<Character>");
        
        // Test TeamMembers with Enemy
        let team_members_token = StructuredTokenInput::TeamMembers {
            team_side: Box::new(StructuredTokenInput::Enemy),
        };
        let result = convert_structured_to_node(&team_members_token).unwrap();
        assert_eq!(result.type_name, "Vec<Character>");
        
        // Test RandomPick
        let random_pick_token = StructuredTokenInput::RandomPick {
            array: Box::new(StructuredTokenInput::AllCharacters),
        };
        let result = convert_structured_to_node(&random_pick_token).unwrap();
        assert_eq!(result.type_name, "Character");
        
        // Test FilterList
        let filter_token = StructuredTokenInput::FilterList {
            array: Box::new(StructuredTokenInput::AllCharacters),
            condition: Box::new(StructuredTokenInput::Eq {
                left: Box::new(StructuredTokenInput::CharacterTeam {
                    character: Box::new(StructuredTokenInput::Element),
                }),
                right: Box::new(StructuredTokenInput::Enemy),
            }),
        };
        let result = convert_structured_to_node(&filter_token).unwrap();
        assert_eq!(result.type_name, "Vec<Character>");
    }
    
    #[test]
    fn test_convert_map_node() {
        // Test Map with CharacterToHp transform
        let map_token = StructuredTokenInput::Map {
            array: Box::new(StructuredTokenInput::AllCharacters),
            transform: Box::new(StructuredTokenInput::CharacterToHp {
                character: Box::new(StructuredTokenInput::Element),
            }),
        };
        let result = convert_structured_to_node(&map_token).unwrap();
        assert_eq!(result.type_name, "Vec<CharacterHP>");
    }
    
    #[test]
    fn test_convert_max_min_nodes() {
        // Test Max with i32 array
        let max_token = StructuredTokenInput::Max {
            array: Box::new(StructuredTokenInput::Map {
                array: Box::new(StructuredTokenInput::AllCharacters),
                transform: Box::new(StructuredTokenInput::CharacterToHp {
                    character: Box::new(StructuredTokenInput::Element),
                }),
            }),
        };
        let result = convert_structured_to_node(&max_token).unwrap();
        assert_eq!(result.type_name, "CharacterHP");
        
        // Test NumericMin
        let min_token = StructuredTokenInput::NumericMin {
            array: Box::new(StructuredTokenInput::Map {
                array: Box::new(StructuredTokenInput::AllCharacters),
                transform: Box::new(StructuredTokenInput::CharacterToHp {
                    character: Box::new(StructuredTokenInput::Element),
                }),
            }),
        };
        let result = convert_structured_to_node(&min_token).unwrap();
        assert_eq!(result.type_name, "CharacterHP");
    }
    
    #[test]
    fn test_convert_team_side_nodes() {
        // Test Enemy
        let enemy_token = StructuredTokenInput::Enemy;
        let result = convert_structured_to_node(&enemy_token).unwrap();
        assert_eq!(result.type_name, "TeamSide");
        
        // Test Hero
        let hero_token = StructuredTokenInput::Hero;
        let result = convert_structured_to_node(&hero_token).unwrap();
        assert_eq!(result.type_name, "TeamSide");
        
        // Test CharacterTeam
        let char_team_token = StructuredTokenInput::CharacterTeam {
            character: Box::new(StructuredTokenInput::ActingCharacter),
        };
        let result = convert_structured_to_node(&char_team_token).unwrap();
        assert_eq!(result.type_name, "TeamSide");
    }
    
    #[test]
    fn test_convert_all_team_sides() {
        let all_sides_token = StructuredTokenInput::AllTeamSides;
        let result = convert_structured_to_node(&all_sides_token).unwrap();
        assert_eq!(result.type_name, "Vec<TeamSide>");
    }
    
    #[test]
    fn test_convert_complex_nested_structure() {
        // Complex: Strike the character with minimum HP from enemy team
        let complex_token = StructuredTokenInput::Strike {
            target: Box::new(StructuredTokenInput::CharacterHpToCharacter {
                character_hp: Box::new(StructuredTokenInput::Min {
                    array: Box::new(StructuredTokenInput::Map {
                        array: Box::new(StructuredTokenInput::FilterList {
                            array: Box::new(StructuredTokenInput::AllCharacters),
                            condition: Box::new(StructuredTokenInput::Eq {
                                left: Box::new(StructuredTokenInput::CharacterTeam {
                                    character: Box::new(StructuredTokenInput::Element),
                                }),
                                right: Box::new(StructuredTokenInput::Enemy),
                            }),
                        }),
                        transform: Box::new(StructuredTokenInput::CharacterToHp {
                            character: Box::new(StructuredTokenInput::Element),
                        }),
                    }),
                }),
            }),
        };
        let result = convert_structured_to_node(&complex_token).unwrap();
        assert_eq!(result.type_name, "Action");
    }
    
    #[test]
    fn test_error_cases() {
        // Test invalid Check with non-action then_action
        let invalid_check = StructuredTokenInput::Check {
            condition: Box::new(StructuredTokenInput::TrueOrFalseRandom),
            then_action: Box::new(StructuredTokenInput::Number { value: 42 }), // Not an action
        };
        let result = convert_structured_to_node(&invalid_check);
        assert!(result.is_err());
        
        // Test unsupported RandomPick array type
        // This should work now since we don't have non-Character arrays in the test
    }
    
    #[test]
    fn test_convert_ruleset_to_nodes() {
        let ruleset = RuleSet {
            rules: vec![
                StructuredTokenInput::Strike {
                    target: Box::new(StructuredTokenInput::ActingCharacter),
                },
                StructuredTokenInput::Check {
                    condition: Box::new(StructuredTokenInput::GreaterThan {
                        left: Box::new(StructuredTokenInput::Number { value: 50 }),
                        right: Box::new(StructuredTokenInput::CharacterToHp {
                            character: Box::new(StructuredTokenInput::ActingCharacter),
                        }),
                    }),
                    then_action: Box::new(StructuredTokenInput::Heal {
                        target: Box::new(StructuredTokenInput::ActingCharacter),
                    }),
                },
                // This should be filtered out as it's not an Action
                StructuredTokenInput::Number { value: 100 },
            ],
        };
        
        let nodes = convert_ruleset_to_nodes(&ruleset);
        assert_eq!(nodes.len(), 2); // Only action nodes should be included
    }
}