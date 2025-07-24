use std::fs;
use std::path::Path;
use token_input::RuleSet;

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

#[cfg(test)]
mod tests {
    use super::*;
    use token_input::{StructuredTokenInput, convert_to_rule_node};

    #[test]
    fn test_parse_simple_rule_json() {
        let rule_json = r#"{
            "rules": [
                {
                    "type": "Strike",
                    "target": {
                        "type": "ActingCharacter"
                    }
                }
            ]
        }"#;
        
        let rule_set = parse_rules_from_json(rule_json).unwrap();
        assert_eq!(rule_set.rules.len(), 1);
    }

    #[test]
    fn test_load_json_rules_functionality() {
        // Test JSON loading functionality using a test rule instead of player_rules.json
        let test_json = r#"{
            "rules": [
                {
                    "type": "Strike",
                    "target": {
                        "type": "ActingCharacter"
                    }
                }
            ]
        }"#;
        
        let rule_set = parse_rules_from_json(test_json).unwrap();
        assert_ne!(rule_set.rules.len(), 0);
        assert_eq!(rule_set.rules.len(), 1);
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
                StructuredTokenInput::Strike { target: Box::new(StructuredTokenInput::ActingCharacter) },
            ],
        };
        
        let node_rules: Vec<_> = rule_set.rules.iter()
            .filter_map(|token| convert_to_rule_node(token))
            .collect();
        assert_eq!(node_rules.len(), 1);
    }

    #[test]
    fn test_convert_complex_nodes() {
        let rule_set = RuleSet {
            rules: vec![
                StructuredTokenInput::Check {
                    condition: Box::new(StructuredTokenInput::GreaterThan {
                        left: Box::new(StructuredTokenInput::CharacterToHp { character: Box::new(StructuredTokenInput::ActingCharacter) }),
                        right: Box::new(StructuredTokenInput::CharacterToHp { character: Box::new(StructuredTokenInput::ActingCharacter) }),
                    }),
                    then_action: Box::new(StructuredTokenInput::Heal { target: Box::new(StructuredTokenInput::ActingCharacter) }),
                },
            ],
        };
        
        let node_rules: Vec<_> = rule_set.rules.iter()
            .filter_map(|token| convert_to_rule_node(token))
            .collect();
        assert_eq!(node_rules.len(), 1);
    }
}