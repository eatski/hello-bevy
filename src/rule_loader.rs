use std::fs;
use std::path::Path;
use crate::action_system::{Token, Check, TrueOrFalseRandom, Strike, Heal, GreaterThanToken, Number, CharacterHP, ActingCharacter};
use crate::rule_input_model::{RuleSet, TokenConfig, ValidatedRuleChain};

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

pub fn convert_to_token_rules(rule_set: &RuleSet) -> Result<Vec<Vec<Box<dyn Token>>>, String> {
    let mut token_rules = Vec::new();
    
    for rule_chain in &rule_set.rules {
        // Validate rule chain before conversion
        let validated_chain = ValidatedRuleChain::from_rule_chain(rule_chain)?;
        
        let mut token_chain = Vec::new();
        
        for token_config in &validated_chain.tokens {
            let token = convert_token_config(token_config)?;
            token_chain.push(token);
        }
        
        token_rules.push(token_chain);
    }
    
    Ok(token_rules)
}

fn convert_token_config(config: &TokenConfig) -> Result<Box<dyn Token>, String> {
    match config {
        TokenConfig::Strike => Ok(Box::new(Strike)),
        TokenConfig::Heal => Ok(Box::new(Heal)),
        TokenConfig::TrueOrFalseRandom => Ok(Box::new(TrueOrFalseRandom)),
        TokenConfig::ActingCharacter => Ok(Box::new(ActingCharacter)),
        TokenConfig::Check { condition, args } => {
            // Try args first, then fallback to condition
            if !args.is_empty() {
                let condition_token = convert_token_config(&args[0])?;
                Ok(Box::new(Check::new_boxed(condition_token)))
            } else if let Some(condition) = condition {
                let condition_token = convert_token_config(condition)?;
                Ok(Box::new(Check::new_boxed(condition_token)))
            } else {
                Err("Check token requires either args or condition".to_string())
            }
        },
        TokenConfig::GreaterThan { left, right, args } => {
            // Try args first, then fallback to left/right
            if args.len() >= 2 {
                let left_token = convert_token_config(&args[0])?;
                let right_token = convert_token_config(&args[1])?;
                Ok(Box::new(GreaterThanToken::new_boxed(left_token, right_token)))
            } else if let (Some(left), Some(right)) = (left, right) {
                let left_token = convert_token_config(left)?;
                let right_token = convert_token_config(right)?;
                Ok(Box::new(GreaterThanToken::new_boxed(left_token, right_token)))
            } else {
                Err("GreaterThan token requires either args array with 2 elements or left/right fields".to_string())
            }
        },
        TokenConfig::Number { value } => {
            Ok(Box::new(Number::new(*value)))
        },
        TokenConfig::CharacterHP { character, args } => {
            // Try args first, then fallback to character
            if !args.is_empty() {
                let character_token = convert_token_config(&args[0])?;
                Ok(Box::new(CharacterHP::new_boxed(character_token)))
            } else if let Some(character) = character {
                match character.as_str() {
                    "Self" => Ok(Box::new(CharacterHP::new(ActingCharacter))),
                    _ => Err(format!("Unknown character type: {}", character)),
                }
            } else {
                Err("CharacterHP token requires either args or character field".to_string())
            }
        },
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
        let rule_set = load_rules_from_file("rules/player_rules.json").unwrap();
        assert!(rule_set.rules.len() > 0);
    }

    #[test]
    fn test_load_enemy_rules_file() {
        let rule_set = load_rules_from_file("rules/enemy_rules.json").unwrap();
        assert!(rule_set.rules.len() > 0);
    }

    #[test]
    fn test_convert_simple_tokens() {
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
        
        let token_rules = convert_to_token_rules(&rule_set).unwrap();
        assert_eq!(token_rules.len(), 1);
        assert_eq!(token_rules[0].len(), 2);
    }

    #[test]
    fn test_convert_complex_tokens() {
        let rule_set = RuleSet {
            rules: vec![
                RuleChain {
                    tokens: vec![
                        TokenConfig::Check {
                            condition: None,
                            args: vec![TokenConfig::TrueOrFalseRandom],
                        },
                        TokenConfig::GreaterThan {
                            left: None,
                            right: None,
                            args: vec![
                                TokenConfig::Number { value: 50 },
                                TokenConfig::CharacterHP { 
                                    character: None,
                                    args: vec![TokenConfig::ActingCharacter],
                                },
                            ],
                        },
                        TokenConfig::Strike, // Add action token after continue tokens
                    ],
                },
            ],
        };
        
        let token_rules = convert_to_token_rules(&rule_set).unwrap();
        assert_eq!(token_rules.len(), 1);
        assert_eq!(token_rules[0].len(), 3); // Now 3 tokens including the Strike
    }

    #[test]
    fn test_convert_check_token_error_no_args_or_condition() {
        let rule_set = RuleSet {
            rules: vec![
                RuleChain {
                    tokens: vec![
                        TokenConfig::Check {
                            condition: None,
                            args: vec![],
                        },
                        TokenConfig::Strike, // Add action to make validation pass first
                    ],
                },
            ],
        };
        
        let result = convert_to_token_rules(&rule_set);
        assert!(result.is_err());
        if let Err(error_msg) = result {
            assert!(error_msg.contains("Check token requires either args or condition"));
        }
    }

    #[test]
    fn test_convert_greater_than_token_error_insufficient_args() {
        let rule_set = RuleSet {
            rules: vec![
                RuleChain {
                    tokens: vec![
                        TokenConfig::GreaterThan {
                            left: None,
                            right: None,
                            args: vec![TokenConfig::Number { value: 50 }], // Only 1 arg, need 2
                        },
                        TokenConfig::Strike, // Add action to make validation pass first
                    ],
                },
            ],
        };
        
        let result = convert_to_token_rules(&rule_set);
        assert!(result.is_err());
        if let Err(error_msg) = result {
            assert!(error_msg.contains("GreaterThan token requires either args array with 2 elements"));
        }
    }

    #[test]
    fn test_convert_character_hp_token_error_no_args_or_character() {
        let rule_set = RuleSet {
            rules: vec![
                RuleChain {
                    tokens: vec![
                        TokenConfig::CharacterHP {
                            character: None,
                            args: vec![],
                        },
                    ],
                },
            ],
        };
        
        let result = convert_to_token_rules(&rule_set);
        assert!(result.is_err());
        if let Err(error_msg) = result {
            assert!(error_msg.contains("CharacterHP token requires either args or character field"));
        }
    }

    #[test]
    fn test_convert_character_hp_token_error_unknown_character() {
        let rule_set = RuleSet {
            rules: vec![
                RuleChain {
                    tokens: vec![
                        TokenConfig::CharacterHP {
                            character: Some("UnknownCharacter".to_string()),
                            args: vec![],
                        },
                    ],
                },
            ],
        };
        
        let result = convert_to_token_rules(&rule_set);
        assert!(result.is_err());
        if let Err(error_msg) = result {
            assert!(error_msg.contains("Unknown character type: UnknownCharacter"));
        }
    }

    #[test]
    fn test_convert_nested_error_propagation() {
        let rule_set = RuleSet {
            rules: vec![
                RuleChain {
                    tokens: vec![
                        TokenConfig::Check {
                            condition: None,
                            args: vec![
                                TokenConfig::CharacterHP {
                                    character: Some("InvalidCharacter".to_string()),
                                    args: vec![],
                                }
                            ],
                        },
                        TokenConfig::Strike, // Add Strike to make it valid sequence
                    ],
                },
            ],
        };
        
        let result = convert_to_token_rules(&rule_set);
        assert!(result.is_err());
        if let Err(error_msg) = result {
            assert!(error_msg.contains("Unknown character type: InvalidCharacter"));
        }
    }

    #[test]
    fn test_continue_token_at_end_error_check() {
        let rule_set = RuleSet {
            rules: vec![
                RuleChain {
                    tokens: vec![
                        TokenConfig::Check {
                            condition: None,
                            args: vec![TokenConfig::TrueOrFalseRandom],
                        },
                        // No token after Check - should cause error
                    ],
                },
            ],
        };
        
        let result = convert_to_token_rules(&rule_set);
        assert!(result.is_err());
        if let Err(error_msg) = result {
            assert!(error_msg.contains("Check token at position 0 cannot be the last token"));
        }
    }

    #[test]
    fn test_continue_token_at_end_error_greater_than() {
        let rule_set = RuleSet {
            rules: vec![
                RuleChain {
                    tokens: vec![
                        TokenConfig::GreaterThan {
                            left: None,
                            right: None,
                            args: vec![
                                TokenConfig::Number { value: 50 },
                                TokenConfig::Number { value: 30 },
                            ],
                        },
                        // No token after GreaterThan - should cause error
                    ],
                },
            ],
        };
        
        let result = convert_to_token_rules(&rule_set);
        assert!(result.is_err());
        if let Err(error_msg) = result {
            assert!(error_msg.contains("GreaterThan token at position 0 cannot be the last token"));
        }
    }

    #[test]
    fn test_continue_token_at_end_error_true_or_false_random() {
        let rule_set = RuleSet {
            rules: vec![
                RuleChain {
                    tokens: vec![
                        TokenConfig::TrueOrFalseRandom,
                        // No token after TrueOrFalseRandom - should cause error
                    ],
                },
            ],
        };
        
        let result = convert_to_token_rules(&rule_set);
        assert!(result.is_err());
        if let Err(error_msg) = result {
            assert!(error_msg.contains("TrueOrFalseRandom token at position 0 cannot be the last token"));
        }
    }

    #[test]
    fn test_valid_continue_token_with_following_action() {
        let rule_set = RuleSet {
            rules: vec![
                RuleChain {
                    tokens: vec![
                        TokenConfig::Check {
                            condition: None,
                            args: vec![TokenConfig::TrueOrFalseRandom],
                        },
                        TokenConfig::Strike, // Valid: action follows continue token
                    ],
                },
            ],
        };
        
        let result = convert_to_token_rules(&rule_set);
        assert!(result.is_ok());
        let token_rules = result.unwrap();
        assert_eq!(token_rules.len(), 1);
        assert_eq!(token_rules[0].len(), 2);
    }

    #[test]
    fn test_action_token_at_end_is_valid() {
        let rule_set = RuleSet {
            rules: vec![
                RuleChain {
                    tokens: vec![
                        TokenConfig::Strike, // Action tokens can be at the end
                    ],
                },
            ],
        };
        
        let result = convert_to_token_rules(&rule_set);
        assert!(result.is_ok());
        let token_rules = result.unwrap();
        assert_eq!(token_rules.len(), 1);
        assert_eq!(token_rules[0].len(), 1);
    }
}