// FlatTokenInput → StructuredTokenInput 変換

use crate::{FlatTokenInput, StructuredTokenInput};
use crate::structured_to_node::{convert_structured_to_node};
use action_system::RuleNode;

// FlatTokenInput → StructuredTokenInput 変換
pub fn convert_flat_to_structured(flat_tokens: &[FlatTokenInput]) -> Result<Vec<StructuredTokenInput>, String> {
    let mut result = Vec::new();
    let mut index = 0;
    
    while index < flat_tokens.len() {
        let (token, consumed) = parse_flat_token(flat_tokens, index)?;
        result.push(token);
        index += consumed;
    }
    
    Ok(result)
}

fn parse_flat_token(tokens: &[FlatTokenInput], index: usize) -> Result<(StructuredTokenInput, usize), String> {
    if index >= tokens.len() {
        return Err("No tokens to parse".to_string());
    }
    
    match &tokens[index] {
        FlatTokenInput::Strike => {
            if index + 1 >= tokens.len() {
                return Err("Strike requires a target".to_string());
            }
            let (target, consumed) = parse_flat_token(tokens, index + 1)?;
            Ok((StructuredTokenInput::Strike { target: Box::new(target) }, 1 + consumed))
        }
        FlatTokenInput::Heal => {
            if index + 1 >= tokens.len() {
                return Err("Heal requires a target".to_string());
            }
            let (target, consumed) = parse_flat_token(tokens, index + 1)?;
            Ok((StructuredTokenInput::Heal { target: Box::new(target) }, 1 + consumed))
        }
        FlatTokenInput::Check => {
            if index + 2 >= tokens.len() {
                return Err("Check requires a condition and action".to_string());
            }
            let (condition, cond_consumed) = parse_flat_token(tokens, index + 1)?;
            let (action, action_consumed) = parse_flat_token(tokens, index + 1 + cond_consumed)?;
            Ok((StructuredTokenInput::Check { 
                condition: Box::new(condition), 
                then_action: Box::new(action) 
            }, 1 + cond_consumed + action_consumed))
        }
        FlatTokenInput::GreaterThan => {
            if index + 2 >= tokens.len() {
                return Err("GreaterThan requires two operands".to_string());
            }
            let (left, left_consumed) = parse_flat_token(tokens, index + 1)?;
            let (right, right_consumed) = parse_flat_token(tokens, index + 1 + left_consumed)?;
            Ok((StructuredTokenInput::GreaterThan { 
                left: Box::new(left), 
                right: Box::new(right) 
            }, 1 + left_consumed + right_consumed))
        }
        FlatTokenInput::HP => {
            if index + 1 >= tokens.len() {
                return Err("HP requires a character".to_string());
            }
            let (character, consumed) = parse_flat_token(tokens, index + 1)?;
            Ok((StructuredTokenInput::HP { character: Box::new(character) }, 1 + consumed))
        }
        FlatTokenInput::Number(n) => Ok((StructuredTokenInput::Number { value: *n as i32 }, 1)),
        FlatTokenInput::ActingCharacter => Ok((StructuredTokenInput::ActingCharacter, 1)),
        FlatTokenInput::AllCharacters => Ok((StructuredTokenInput::AllCharacters, 1)),
        FlatTokenInput::RandomPick => {
            if index + 1 >= tokens.len() {
                return Err("RandomPick requires an array argument".to_string());
            }
            let (array_token, array_consumed) = parse_flat_token(tokens, index + 1)?;
            Ok((StructuredTokenInput::RandomPick { array: Box::new(array_token) }, 1 + array_consumed))
        }
        FlatTokenInput::TrueOrFalse => Ok((StructuredTokenInput::TrueOrFalseRandom, 1)),
        FlatTokenInput::FilterList => {
            if index + 2 >= tokens.len() {
                return Err("FilterList requires an array and a condition".to_string());
            }
            let (array, array_consumed) = parse_flat_token(tokens, index + 1)?;
            let (condition, condition_consumed) = parse_flat_token(tokens, index + 1 + array_consumed)?;
            Ok((StructuredTokenInput::FilterList {
                array: Box::new(array),
                condition: Box::new(condition),
            }, 1 + array_consumed + condition_consumed))
        }
        FlatTokenInput::Map => {
            if index + 2 >= tokens.len() {
                return Err("Map requires an array and a transform function".to_string());
            }
            let (array, array_consumed) = parse_flat_token(tokens, index + 1)?;
            let (transform, transform_consumed) = parse_flat_token(tokens, index + 1 + array_consumed)?;
            Ok((StructuredTokenInput::Map {
                array: Box::new(array),
                transform: Box::new(transform),
            }, 1 + array_consumed + transform_consumed))
        }
        FlatTokenInput::Eq => {
            if index + 2 >= tokens.len() {
                return Err("Eq requires two operands".to_string());
            }
            let (left, left_consumed) = parse_flat_token(tokens, index + 1)?;
            let (right, right_consumed) = parse_flat_token(tokens, index + 1 + left_consumed)?;
            Ok((StructuredTokenInput::Eq {
                left: Box::new(left),
                right: Box::new(right),
            }, 1 + left_consumed + right_consumed))
        }
        FlatTokenInput::CharacterTeam => {
            if index + 1 >= tokens.len() {
                return Err("CharacterTeam requires a character".to_string());
            }
            let (character, consumed) = parse_flat_token(tokens, index + 1)?;
            Ok((StructuredTokenInput::CharacterTeam {
                character: Box::new(character),
            }, 1 + consumed))
        }
        FlatTokenInput::Element => Ok((StructuredTokenInput::Element, 1)),
        FlatTokenInput::Enemy => Ok((StructuredTokenInput::Enemy, 1)),
        FlatTokenInput::Hero => Ok((StructuredTokenInput::Hero, 1)),
        FlatTokenInput::Max => {
            if index + 1 >= tokens.len() {
                return Err("Max requires an array argument".to_string());
            }
            let (array_token, array_consumed) = parse_flat_token(tokens, index + 1)?;
            Ok((StructuredTokenInput::Max { array: Box::new(array_token) }, 1 + array_consumed))
        }
        FlatTokenInput::Min => {
            if index + 1 >= tokens.len() {
                return Err("Min requires an array argument".to_string());
            }
            let (array_token, array_consumed) = parse_flat_token(tokens, index + 1)?;
            Ok((StructuredTokenInput::Min { array: Box::new(array_token) }, 1 + array_consumed))
        }
    }
}

// Vec<Vec<FlatTokenInput>> → Vec<RuleNode> 変換（UI入力経路）
pub fn convert_flat_rules_to_nodes(flat_rules: &[Vec<FlatTokenInput>]) -> Vec<RuleNode> {
    flat_rules
        .iter()
        .filter(|rule_row| !rule_row.is_empty())
        .filter_map(|rule_row| {
            // FlatTokenInput → StructuredTokenInput → Node
            match convert_flat_to_structured(rule_row) {
                Ok(structured_tokens) => {
                    if structured_tokens.is_empty() {
                        println!("WARNING: Empty structured tokens for rule: {:?}", rule_row);
                        return None;
                    }
                    
                    match convert_structured_to_node(&structured_tokens[0]) {
                        Ok(parsed) => {
                            match parsed.require_action() {
                                Ok(action) => Some(action),
                                Err(e) => {
                                    println!("ERROR: Failed to require action for rule: {:?}", rule_row);
                                    println!("       Error: {}", e);
                                    None
                                }
                            }
                        }
                        Err(e) => {
                            println!("ERROR: Failed to convert structured to node for rule: {:?}", rule_row);
                            println!("       Error: {}", e);
                            None
                        }
                    }
                }
                Err(e) => {
                    println!("ERROR: Failed to convert flat to structured for rule: {:?}", rule_row);
                    println!("       Error: {}", e);
                    None
                }
            }
        })
        .collect()
}

// Result版 - エラーを明示的に返す
pub fn convert_flat_rules_to_nodes_strict(flat_rules: &[Vec<FlatTokenInput>]) -> Result<Vec<RuleNode>, String> {
    let mut result = Vec::new();
    
    for (rule_index, rule_row) in flat_rules.iter().enumerate() {
        if rule_row.is_empty() {
            continue;
        }
        
        // FlatTokenInput → StructuredTokenInput → Node
        let structured_tokens = convert_flat_to_structured(rule_row)
            .map_err(|e| format!("Rule {}: Failed to convert flat to structured: {}", rule_index, e))?;
        
        if structured_tokens.is_empty() {
            return Err(format!("Rule {}: Empty structured tokens", rule_index));
        }
        
        let parsed = convert_structured_to_node(&structured_tokens[0])
            .map_err(|e| format!("Rule {}: Failed to convert structured to node: {}", rule_index, e))?;
        
        let action = parsed.require_action()
            .map_err(|e| format!("Rule {}: Failed to require action: {}", rule_index, e))?;
        
        result.push(action);
    }
    
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flat_to_structured_simple() {
        let flat = vec![FlatTokenInput::Strike, FlatTokenInput::ActingCharacter];
        let structured = convert_flat_to_structured(&flat).unwrap();
        
        assert_eq!(structured.len(), 1);
        match &structured[0] {
            StructuredTokenInput::Strike { target } => {
                match target.as_ref() {
                    StructuredTokenInput::ActingCharacter => (),
                    _ => panic!("Expected ActingCharacter target"),
                }
            }
            _ => panic!("Expected Strike"),
        }
    }

    #[test]
    fn test_all_characters_token() {
        // Test FlatTokenInput::AllCharacters conversion
        let flat = vec![FlatTokenInput::AllCharacters];
        let structured = convert_flat_to_structured(&flat).unwrap();
        
        assert_eq!(structured.len(), 1);
        match &structured[0] {
            StructuredTokenInput::AllCharacters => (),
            _ => panic!("Expected AllCharacters"),
        }
    }

    #[test]
    fn test_random_pick_token() {
        // Test FlatTokenInput::RandomPick with AllCharacters
        let flat = vec![FlatTokenInput::RandomPick, FlatTokenInput::AllCharacters];
        let structured = convert_flat_to_structured(&flat).unwrap();
        
        assert_eq!(structured.len(), 1);
        match &structured[0] {
            StructuredTokenInput::RandomPick { array } => {
                match array.as_ref() {
                    StructuredTokenInput::AllCharacters => (),
                    _ => panic!("Expected AllCharacters array"),
                }
            }
            _ => panic!("Expected RandomPick"),
        }
    }

    #[test]
    fn test_new_tokens_flat_to_structured() {
        // Test new tokens: FilterList, Eq, CharacterTeam, Element, Enemy, Hero
        let flat = vec![
            FlatTokenInput::FilterList, FlatTokenInput::AllCharacters, FlatTokenInput::Eq, 
            FlatTokenInput::CharacterTeam, FlatTokenInput::Element, FlatTokenInput::Enemy
        ];
        let structured = convert_flat_to_structured(&flat).unwrap();
        
        assert_eq!(structured.len(), 1);
        match &structured[0] {
            StructuredTokenInput::FilterList { array, condition } => {
                match array.as_ref() {
                    StructuredTokenInput::AllCharacters => (),
                    _ => panic!("Expected AllCharacters array"),
                }
                match condition.as_ref() {
                    StructuredTokenInput::Eq { left, right } => {
                        match left.as_ref() {
                            StructuredTokenInput::CharacterTeam { character } => {
                                match character.as_ref() {
                                    StructuredTokenInput::Element => (),
                                    _ => panic!("Expected Element character"),
                                }
                            }
                            _ => panic!("Expected CharacterTeam left"),
                        }
                        match right.as_ref() {
                            StructuredTokenInput::Enemy => (),
                            _ => panic!("Expected Enemy right"),
                        }
                    }
                    _ => panic!("Expected Eq condition"),
                }
            }
            _ => panic!("Expected FilterList"),
        }
    }

    #[test]
    fn test_enemy_hero_tokens() {
        // Test Enemy token
        let flat_enemy = vec![FlatTokenInput::Enemy];
        let structured_enemy = convert_flat_to_structured(&flat_enemy).unwrap();
        assert_eq!(structured_enemy.len(), 1);
        match &structured_enemy[0] {
            StructuredTokenInput::Enemy => (),
            _ => panic!("Expected Enemy"),
        }
        
        // Test Hero token
        let flat_hero = vec![FlatTokenInput::Hero];
        let structured_hero = convert_flat_to_structured(&flat_hero).unwrap();
        assert_eq!(structured_hero.len(), 1);
        match &structured_hero[0] {
            StructuredTokenInput::Hero => (),
            _ => panic!("Expected Hero"),
        }
    }

    #[test]
    fn test_eq_value_comparison() {
        // Test Eq with Number values
        let flat = vec![
            FlatTokenInput::Eq, 
            FlatTokenInput::Number(10), 
            FlatTokenInput::Number(10)
        ];
        let structured = convert_flat_to_structured(&flat).unwrap();
        
        assert_eq!(structured.len(), 1);
        match &structured[0] {
            StructuredTokenInput::Eq { left, right } => {
                match (left.as_ref(), right.as_ref()) {
                    (StructuredTokenInput::Number { value: 10 }, StructuredTokenInput::Number { value: 10 }) => (),
                    _ => panic!("Expected Number(10) == Number(10)"),
                }
            }
            _ => panic!("Expected Eq"),
        }
    }

    #[test]
    fn test_eq_character_comparison() {
        // Test Eq with Character values
        let flat = vec![
            FlatTokenInput::Eq, 
            FlatTokenInput::ActingCharacter, 
            FlatTokenInput::Element
        ];
        let structured = convert_flat_to_structured(&flat).unwrap();
        
        assert_eq!(structured.len(), 1);
        match &structured[0] {
            StructuredTokenInput::Eq { left, right } => {
                match (left.as_ref(), right.as_ref()) {
                    (StructuredTokenInput::ActingCharacter, StructuredTokenInput::Element) => (),
                    _ => panic!("Expected ActingCharacter == Element"),
                }
            }
            _ => panic!("Expected Eq"),
        }
    }

    #[test]
    fn test_eq_team_comparison() {
        // Test Eq with TeamSide values  
        let flat = vec![
            FlatTokenInput::Eq, 
            FlatTokenInput::Enemy, 
            FlatTokenInput::Hero
        ];
        let structured = convert_flat_to_structured(&flat).unwrap();
        
        assert_eq!(structured.len(), 1);
        match &structured[0] {
            StructuredTokenInput::Eq { left, right } => {
                match (left.as_ref(), right.as_ref()) {
                    (StructuredTokenInput::Enemy, StructuredTokenInput::Hero) => (),
                    _ => panic!("Expected Enemy == Hero"),
                }
            }
            _ => panic!("Expected Eq"),
        }
    }

    #[test]
    fn test_flat_rules_to_nodes() {
        let flat_rules = vec![
            vec![FlatTokenInput::Strike, FlatTokenInput::ActingCharacter],
            vec![FlatTokenInput::Heal, FlatTokenInput::RandomPick, FlatTokenInput::AllCharacters],
        ];
        
        let nodes = convert_flat_rules_to_nodes(&flat_rules);
        assert_eq!(nodes.len(), 2);
    }

    #[test]
    fn test_conversion_integration() {
        // Test full conversion pipeline: Flat → Structured → Node
        let flat = vec![FlatTokenInput::Strike, FlatTokenInput::ActingCharacter];
        let structured = convert_flat_to_structured(&flat).unwrap();
        let node_result = convert_structured_to_node(&structured[0]).unwrap();
        assert!(node_result.require_action().is_ok());
    }
}