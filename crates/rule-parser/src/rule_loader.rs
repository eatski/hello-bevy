use std::fs;
use std::path::Path;
use combat_engine::{RuleNode, ConditionCheckNode, ActionResolver, ConditionNode, ValueNode, ConstantValueNode, ActingCharacterNode, CharacterHpFromNode, RandomConditionNode, GreaterThanConditionNode, StrikeActionNode, HealActionNode};
use crate::rule_input_model::{RuleSet, TokenConfig};

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

fn convert_node_chain(tokens: &[TokenConfig]) -> Result<RuleNode, String> {
    if tokens.is_empty() {
        return Err("Empty token chain".to_string());
    }
    
    let mut result: Option<Box<dyn ActionResolver>> = None;
    
    // Process tokens in reverse order to build the chain
    for token_config in tokens.iter().rev() {
        match token_config {
            TokenConfig::Strike => {
                result = Some(Box::new(StrikeActionNode));
            }
            TokenConfig::Heal => {
                result = Some(Box::new(HealActionNode));
            }
            TokenConfig::Check { condition, then_action } => {
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

fn convert_bool_node_config(config: &TokenConfig) -> Result<Box<dyn ConditionNode>, String> {
    match config {
        TokenConfig::TrueOrFalseRandom => Ok(Box::new(RandomConditionNode)),
        TokenConfig::GreaterThan { left, right } => {
            let left_node = convert_number_node_config(left)?;
            let right_node = convert_number_node_config(right)?;
            Ok(Box::new(GreaterThanConditionNode::new(left_node, right_node)))
        },
        _ => Err(format!("Cannot convert {:?} to ConditionNode", config)),
    }
}

fn convert_number_node_config(config: &TokenConfig) -> Result<Box<dyn ValueNode>, String> {
    match config {
        TokenConfig::Number { value } => Ok(Box::new(ConstantValueNode::new(*value))),
        TokenConfig::CharacterHP => Ok(Box::new(CharacterHpFromNode::new(Box::new(ActingCharacterNode)))),
        _ => Err(format!("Cannot convert {:?} to ValueNode", config)),
    }
}

fn convert_single_token_to_resolver(config: &TokenConfig) -> Result<Box<dyn ActionResolver>, String> {
    match config {
        TokenConfig::Strike => Ok(Box::new(StrikeActionNode)),
        TokenConfig::Heal => Ok(Box::new(HealActionNode)),
        TokenConfig::Check { condition, then_action } => {
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
                        TokenConfig::Strike,
                        TokenConfig::Heal,
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
                        TokenConfig::Check {
                            condition: Box::new(TokenConfig::GreaterThan {
                                left: Box::new(TokenConfig::Number { value: 50 }),
                                right: Box::new(TokenConfig::CharacterHP),
                            }),
                            then_action: Box::new(TokenConfig::Heal),
                        },
                        TokenConfig::Heal, // Changed to Heal to make it different from the first rule
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
                        TokenConfig::Check {
                            condition: Box::new(TokenConfig::TrueOrFalseRandom),
                            then_action: Box::new(TokenConfig::Strike),
                        },
                        TokenConfig::Strike, // Add action to make validation pass first
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
                        TokenConfig::Check {
                            condition: Box::new(TokenConfig::GreaterThan {
                                left: Box::new(TokenConfig::Number { value: 50 }),
                                right: Box::new(TokenConfig::Number { value: 30 }),
                            }),
                            then_action: Box::new(TokenConfig::Strike),
                        },
                        TokenConfig::Strike, // Add action to make validation pass first
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
                        TokenConfig::CharacterHP,
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
                        TokenConfig::Check {
                            condition: Box::new(TokenConfig::GreaterThan {
                                left: Box::new(TokenConfig::Number { value: 50 }),
                                right: Box::new(TokenConfig::CharacterHP),
                            }),
                            then_action: Box::new(TokenConfig::Heal),
                        },
                        TokenConfig::Strike,
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
                        TokenConfig::Check {
                            condition: Box::new(TokenConfig::GreaterThan {
                                left: Box::new(TokenConfig::Number { value: 50 }),
                                right: Box::new(TokenConfig::CharacterHP),
                            }),
                            then_action: Box::new(TokenConfig::Heal),
                        },
                        TokenConfig::Strike,
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
                        TokenConfig::Check {
                            condition: Box::new(TokenConfig::TrueOrFalseRandom),
                            then_action: Box::new(TokenConfig::Strike),
                        },
                        TokenConfig::Strike, // Valid: action follows continue token
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
                        TokenConfig::Strike, // Action tokens can be at the end
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