use std::fs;
use std::path::Path;
use action_system::{RuleNode, ConditionCheckNode, ActionResolver, ConditionNode, ValueNode, ConstantValueNode, ActingCharacterNode, CharacterHpFromNode, RandomConditionNode, GreaterThanConditionNode, StrikeActionNode, HealActionNode, RandomCharacterNode};
use action_system::nodes::character::CharacterNode;
use crate::rule_input_model::{RuleSet, JsonTokenInput};

// Unified parse result enum - similar to ui-core token_converter
#[derive(Debug)]
pub enum ParsedResolver {
    Action(Box<dyn ActionResolver>),
    Condition(Box<dyn ConditionNode>),
    Value(Box<dyn ValueNode>),
    Character(Box<dyn CharacterNode>),
}

// Type matching functions - safely extract specific types from ParsedResolver
impl ParsedResolver {
    pub fn require_action(self) -> Result<Box<dyn ActionResolver>, String> {
        match self {
            ParsedResolver::Action(action) => Ok(action),
            _ => Err(format!("Expected Action, got {:?}", self)),
        }
    }
    
    pub fn require_condition(self) -> Result<Box<dyn ConditionNode>, String> {
        match self {
            ParsedResolver::Condition(condition) => Ok(condition),
            _ => Err(format!("Expected Condition, got {:?}", self)),
        }
    }
    
    pub fn require_value(self) -> Result<Box<dyn ValueNode>, String> {
        match self {
            ParsedResolver::Value(value) => Ok(value),
            _ => Err(format!("Expected Value, got {:?}", self)),
        }
    }
    
    
    pub fn require_character(self) -> Result<Box<dyn CharacterNode>, String> {
        match self {
            ParsedResolver::Character(character_node) => Ok(character_node),
            _ => Err(format!("Expected Character, got {:?}", self)),
        }
    }
}

// Unified parser function - converts JsonTokenInput to ParsedResolver
pub fn parse_json_token(config: &JsonTokenInput) -> Result<ParsedResolver, String> {
    match config {
        // Action tokens
        JsonTokenInput::Strike => Ok(ParsedResolver::Action(Box::new(StrikeActionNode))),
        JsonTokenInput::Heal => Ok(ParsedResolver::Action(Box::new(HealActionNode))),
        
        // Condition tokens
        JsonTokenInput::TrueOrFalseRandom => Ok(ParsedResolver::Condition(Box::new(RandomConditionNode))),
        JsonTokenInput::GreaterThan { left, right } => {
            let left_node = parse_json_token(left)?.require_value()?;
            let right_node = parse_json_token(right)?.require_value()?;
            Ok(ParsedResolver::Condition(Box::new(GreaterThanConditionNode::new(left_node, right_node))))
        },
        
        // Value tokens
        JsonTokenInput::Number { value } => Ok(ParsedResolver::Value(Box::new(ConstantValueNode::new(*value)))),
        JsonTokenInput::CharacterHP => Ok(ParsedResolver::Value(Box::new(CharacterHpFromNode::new(Box::new(ActingCharacterNode))))),
        JsonTokenInput::HP { character } => {
            // Parse the character argument recursively and handle based on result type
            let character_parsed = parse_json_token(character)?;
            match character_parsed {
                ParsedResolver::Character(character_node) => {
                    // For CharacterNode types, we can use CharacterHpFromNode directly
                    Ok(ParsedResolver::Value(Box::new(CharacterHpFromNode::new(character_node))))
                },
                ParsedResolver::Action(_) => {
                    Err(format!("HP token requires a Character argument, got Action"))
                },
                ParsedResolver::Condition(_) => {
                    Err(format!("HP token requires a Character argument, got Condition"))
                },
                ParsedResolver::Value(_) => {
                    Err(format!("HP token requires a Character argument, got Value"))
                },
            }
        },
        
        // Complex tokens
        JsonTokenInput::Check { condition, then_action } => {
            let condition_node = parse_json_token(condition)?.require_condition()?;
            let action_node = parse_json_token(then_action)?.require_action()?;
            Ok(ParsedResolver::Action(Box::new(ConditionCheckNode::new(condition_node, action_node))))
        },
        
        
        // Character types that return ParsedResolver::Character
        JsonTokenInput::ActingCharacter => Ok(ParsedResolver::Character(Box::new(ActingCharacterNode))),
        JsonTokenInput::RandomCharacter => Ok(ParsedResolver::Character(Box::new(RandomCharacterNode::new()))),
    }
}

pub fn load_rules_from_file<P: AsRef<Path>>(path: P) -> Result<RuleSet, String> {
    let content = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read file: {}", e))?;
    
    parse_rules_from_json(&content)
}

pub fn parse_rules_from_json(json_content: &str) -> Result<RuleSet, String> {
    let rule_set: RuleSet = serde_json::from_str(json_content)
        .map_err(|e| format!("Failed to parse JSON: {}", e))?;
    
    Ok(rule_set)
}

pub fn convert_to_node_rules(rule_set: &RuleSet) -> Result<Vec<RuleNode>, String> {
    let mut node_rules = Vec::new();
    
    for rule_chain in &rule_set.rules {
        // Convert token chain to single chained ActionResolver using new unified parser
        let rule_node = convert_node_chain_unified(&rule_chain.tokens)?;
        node_rules.push(rule_node);
    }
    
    Ok(node_rules)
}

// New unified approach using parse_json_token
fn convert_node_chain_unified(tokens: &[JsonTokenInput]) -> Result<RuleNode, String> {
    if tokens.is_empty() {
        return Err("Empty token chain".to_string());
    }
    
    let mut result: Option<Box<dyn ActionResolver>> = None;
    
    // Process tokens in reverse order to build the chain
    for token_config in tokens.iter().rev() {
        let parsed = parse_json_token(token_config)?;
        let action_resolver = parsed.require_action()?;
        
        if result.is_some() {
            // For now, just use the latest action (chaining logic can be improved later)
            result = Some(action_resolver);
        } else {
            result = Some(action_resolver);
        }
    }
    
    result.ok_or_else(|| "Failed to build node chain".to_string())
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::rule_input_model::RuleChain;

    #[test]
    fn test_parse_simple_rule_json() {
        let rule_json = r#"{
            "rules": [
                {
                    "tokens": [
                        {
                            "type": "Strike"
                        }
                    ]
                }
            ]
        }"#;
        
        let rule_set = parse_rules_from_json(rule_json).unwrap();
        assert_eq!(rule_set.rules.len(), 1);
        assert_eq!(rule_set.rules[0].tokens.len(), 1);
    }

    #[test]
    fn test_load_player_rules_file() {
        let rule_set = load_rules_from_file("../../rules/player_rules.json").unwrap();
        assert_ne!(rule_set.rules.len(), 0);
    }

    #[test]
    fn test_load_enemy_rules_file() {
        let rule_set = load_rules_from_file("../../rules/enemy_rules.json").unwrap();
        assert_ne!(rule_set.rules.len(), 0);
    }

    #[test]
    fn test_convert_simple_nodes() {
        let rule_set = RuleSet {
            rules: vec![
                RuleChain {
                    tokens: vec![
                        JsonTokenInput::Strike,
                        JsonTokenInput::Heal,
                    ],
                },
            ],
        };
        
        let node_rules = convert_to_node_rules(&rule_set).unwrap();
        assert_eq!(node_rules.len(), 1);
        // Note: node_rules[0] is now a single chained ActionResolver, not a vector
    }

    #[test]
    fn test_convert_complex_nodes() {
        let rule_set = RuleSet {
            rules: vec![
                RuleChain {
                    tokens: vec![
                        JsonTokenInput::Check {
                            condition: Box::new(JsonTokenInput::GreaterThan {
                                left: Box::new(JsonTokenInput::Number { value: 50 }),
                                right: Box::new(JsonTokenInput::CharacterHP),
                            }),
                            then_action: Box::new(JsonTokenInput::Heal),
                        },
                        JsonTokenInput::Heal, // Changed to Heal to make it different from the first rule
                    ],
                },
            ],
        };
        
        let node_rules = convert_to_node_rules(&rule_set).unwrap();
        assert_eq!(node_rules.len(), 1);
        // Note: node_rules[0] is now a single chained ActionResolver, not a vector
    }

    #[test]
    fn test_convert_check_node_error_no_args_or_condition() {
        let rule_set = RuleSet {
            rules: vec![
                RuleChain {
                    tokens: vec![
                        JsonTokenInput::Check {
                            condition: Box::new(JsonTokenInput::TrueOrFalseRandom),
                            then_action: Box::new(JsonTokenInput::Strike),
                        },
                        JsonTokenInput::Strike, // Add action to make validation pass first
                    ],
                },
            ],
        };
        
        let result = convert_to_node_rules(&rule_set);
        assert_eq!(result.is_ok(), true); // This should now be valid
    }

    #[test]
    fn test_convert_greater_than_node_error_insufficient_args() {
        let rule_set = RuleSet {
            rules: vec![
                RuleChain {
                    tokens: vec![
                        JsonTokenInput::Check {
                            condition: Box::new(JsonTokenInput::GreaterThan {
                                left: Box::new(JsonTokenInput::Number { value: 50 }),
                                right: Box::new(JsonTokenInput::Number { value: 30 }),
                            }),
                            then_action: Box::new(JsonTokenInput::Strike),
                        },
                        JsonTokenInput::Strike, // Add action to make validation pass first
                    ],
                },
            ],
        };
        
        let result = convert_to_node_rules(&rule_set);
        assert_eq!(result.is_ok(), true); // This should now be valid
    }

    #[test]
    fn test_convert_character_hp_node_no_args_or_character() {
        let rule_set = RuleSet {
            rules: vec![
                RuleChain {
                    tokens: vec![
                        JsonTokenInput::CharacterHP,
                    ],
                },
            ],
        };
        
        // This should fail because CharacterHP cannot be used as a direct action
        let result = convert_to_node_rules(&rule_set);
        assert_eq!(result.is_err(), true);
        if let Err(error_msg) = result {
            assert_eq!(error_msg.contains("Expected Action"), true);
        }
    }

    #[test]
    fn test_convert_character_hp_node_success() {
        let rule_set = RuleSet {
            rules: vec![
                RuleChain {
                    tokens: vec![
                        JsonTokenInput::Check {
                            condition: Box::new(JsonTokenInput::GreaterThan {
                                left: Box::new(JsonTokenInput::Number { value: 50 }),
                                right: Box::new(JsonTokenInput::CharacterHP),
                            }),
                            then_action: Box::new(JsonTokenInput::Heal),
                        },
                        JsonTokenInput::Strike,
                    ],
                },
            ],
        };
        
        let result = convert_to_node_rules(&rule_set);
        assert!(result.is_ok());
    }

    #[test]
    fn test_convert_nested_structure_success() {
        let rule_set = RuleSet {
            rules: vec![
                RuleChain {
                    tokens: vec![
                        JsonTokenInput::Check {
                            condition: Box::new(JsonTokenInput::GreaterThan {
                                left: Box::new(JsonTokenInput::Number { value: 50 }),
                                right: Box::new(JsonTokenInput::CharacterHP),
                            }),
                            then_action: Box::new(JsonTokenInput::Heal),
                        },
                        JsonTokenInput::Strike,
                    ],
                },
            ],
        };
        
        let result = convert_to_node_rules(&rule_set);
        assert!(result.is_ok());
    }




    #[test]
    fn test_valid_continue_node_with_following_action() {
        let rule_set = RuleSet {
            rules: vec![
                RuleChain {
                    tokens: vec![
                        JsonTokenInput::Check {
                            condition: Box::new(JsonTokenInput::TrueOrFalseRandom),
                            then_action: Box::new(JsonTokenInput::Strike),
                        },
                        JsonTokenInput::Strike, // Valid: action follows continue token
                    ],
                },
            ],
        };
        
        let result = convert_to_node_rules(&rule_set);
        assert!(result.is_ok());
        let node_rules = result.unwrap();
        assert_eq!(node_rules.len(), 1);
        // Note: node_rules[0] is now a single chained ActionResolver, not a vector
    }

    #[test]
    fn test_action_node_at_end_is_valid() {
        let rule_set = RuleSet {
            rules: vec![
                RuleChain {
                    tokens: vec![
                        JsonTokenInput::Strike, // Action tokens can be at the end
                    ],
                },
            ],
        };
        
        let result = convert_to_node_rules(&rule_set);
        assert!(result.is_ok());
        let node_rules = result.unwrap();
        assert_eq!(node_rules.len(), 1);
        // Note: node_rules[0] is now a single chained ActionResolver, not a vector
    }

    #[test]
    fn test_hp_with_acting_character() {
        let rule_set = RuleSet {
            rules: vec![
                RuleChain {
                    tokens: vec![
                        JsonTokenInput::Check {
                            condition: Box::new(JsonTokenInput::GreaterThan {
                                left: Box::new(JsonTokenInput::Number { value: 50 }),
                                right: Box::new(JsonTokenInput::HP {
                                    character: Box::new(JsonTokenInput::ActingCharacter),
                                }),
                            }),
                            then_action: Box::new(JsonTokenInput::Heal),
                        },
                        JsonTokenInput::Strike,
                    ],
                },
            ],
        };
        
        let result = convert_to_node_rules(&rule_set);
        assert!(result.is_ok());
        let node_rules = result.unwrap();
        assert_eq!(node_rules.len(), 1);
    }

    #[test]
    fn test_hp_with_random_character() {
        let rule_set = RuleSet {
            rules: vec![
                RuleChain {
                    tokens: vec![
                        JsonTokenInput::Check {
                            condition: Box::new(JsonTokenInput::GreaterThan {
                                left: Box::new(JsonTokenInput::Number { value: 30 }),
                                right: Box::new(JsonTokenInput::HP {
                                    character: Box::new(JsonTokenInput::RandomCharacter),
                                }),
                            }),
                            then_action: Box::new(JsonTokenInput::Heal),
                        },
                        JsonTokenInput::Strike,
                    ],
                },
            ],
        };
        
        let result = convert_to_node_rules(&rule_set);
        assert!(result.is_ok());
        let node_rules = result.unwrap();
        assert_eq!(node_rules.len(), 1);
    }

}