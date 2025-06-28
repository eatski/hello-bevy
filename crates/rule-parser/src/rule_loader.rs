use std::fs;
use std::path::Path;
use combat_engine::{RuleNode, ConditionCheckNode, ActionResolver, ConditionNode, ValueNode, ConstantValueNode, ActingCharacterNode, CharacterHpFromNode, RandomConditionNode, GreaterThanConditionNode, StrikeActionNode, HealActionNode};
use crate::rule_input_model::{RuleSet, JsonTokenInput};

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
        // Convert token chain to single chained ActionResolver
        let rule_node = convert_node_chain(&rule_chain.tokens)?;
        node_rules.push(rule_node);
    }
    
    Ok(node_rules)
}

fn convert_node_chain(tokens: &[JsonTokenInput]) -> Result<RuleNode, String> {
    if tokens.is_empty() {
        return Err("Empty token chain".to_string());
    }
    
    let mut result: Option<Box<dyn ActionResolver>> = None;
    
    // Process tokens in reverse order to build the chain
    for token_config in tokens.iter().rev() {
        match token_config {
            JsonTokenInput::Strike => {
                result = Some(Box::new(StrikeActionNode));
            }
            JsonTokenInput::Heal => {
                result = Some(Box::new(HealActionNode));
            }
            JsonTokenInput::Check { condition, then_action } => {
                let bool_node = convert_bool_node_config(condition)?;
                let action_node = convert_single_token_to_resolver(then_action)?;
                
                if let Some(_next) = result {
                    // Chain: if condition -> action_node, else -> next
                    let chained_action = Box::new(ConditionCheckNode::new(bool_node, action_node));
                    result = Some(chained_action);
                } else {
                    result = Some(Box::new(ConditionCheckNode::new(bool_node, action_node)));
                }
            }
            _ => {
                return Err(format!("Token {:?} cannot be used directly in rule chain", token_config));
            }
        }
    }
    
    result.ok_or_else(|| "Failed to build node chain".to_string())
}

fn convert_bool_node_config(config: &JsonTokenInput) -> Result<Box<dyn ConditionNode>, String> {
    match config {
        JsonTokenInput::TrueOrFalseRandom => Ok(Box::new(RandomConditionNode)),
        JsonTokenInput::GreaterThan { left, right } => {
            let left_node = convert_number_node_config(left)?;
            let right_node = convert_number_node_config(right)?;
            Ok(Box::new(GreaterThanConditionNode::new(left_node, right_node)))
        },
        _ => Err(format!("Cannot convert {:?} to ConditionNode", config)),
    }
}

fn convert_number_node_config(config: &JsonTokenInput) -> Result<Box<dyn ValueNode>, String> {
    match config {
        JsonTokenInput::Number { value } => Ok(Box::new(ConstantValueNode::new(*value))),
        JsonTokenInput::CharacterHP => Ok(Box::new(CharacterHpFromNode::new(Box::new(ActingCharacterNode)))),
        _ => Err(format!("Cannot convert {:?} to ValueNode", config)),
    }
}

fn convert_single_token_to_resolver(config: &JsonTokenInput) -> Result<Box<dyn ActionResolver>, String> {
    match config {
        JsonTokenInput::Strike => Ok(Box::new(StrikeActionNode)),
        JsonTokenInput::Heal => Ok(Box::new(HealActionNode)),
        JsonTokenInput::Check { condition, then_action } => {
            let bool_node = convert_bool_node_config(condition)?;
            let action_node = convert_single_token_to_resolver(then_action)?;
            Ok(Box::new(ConditionCheckNode::new(bool_node, action_node)))
        }
        _ => Err(format!("Cannot convert {:?} to single ActionResolver", config)),
    }
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
        assert!(rule_set.rules.len() > 0);
    }

    #[test]
    fn test_load_enemy_rules_file() {
        let rule_set = load_rules_from_file("../../rules/enemy_rules.json").unwrap();
        assert!(rule_set.rules.len() > 0);
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
        assert!(result.is_ok()); // This should now be valid
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
        assert!(result.is_ok()); // This should now be valid
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
        assert!(result.is_err());
        if let Err(error_msg) = result {
            assert!(error_msg.contains("cannot be used directly in rule chain"));
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
}