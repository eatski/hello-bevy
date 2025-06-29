// UI converter - converts UI tokens directly to nodes

use action_system::{RuleNode, ConditionCheckNode, ActionResolver, ConditionNode, ValueNode, ConstantValueNode, ActingCharacterNode, RandomCharacterNode, CharacterHpFromNode, RandomConditionNode, GreaterThanConditionNode, StrikeActionNode, HealActionNode};

// パース結果を表すEnum - 統一されたパーサーの戻り値
#[derive(Debug)]
pub enum ParsedResolver {
    Action(Box<dyn ActionResolver>),
    Condition(Box<dyn ConditionNode>),
    Value(Box<dyn ValueNode>),
}

// UI側のトークンタイプの定義
#[derive(Clone, Debug, PartialEq)]
pub enum UITokenType {
    Check,
    Strike,
    Heal,
    Number(u32),
    ActingCharacter,  // 行動するキャラクター
    RandomCharacter,  // ランダムなキャラクター
    HP,               // HP値
    GreaterThan,
    TrueOrFalse,
}

// Note: Display text logic moved to bevy-ui crate

// 型マッチング関数群 - ParsedResolverから特定の型を安全に取得
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
}

// 統一されたパーサー関数 - 全てのトークンを単一の関数でパース
pub fn parse_ui_token(tokens: &[UITokenType], index: usize) -> Result<(ParsedResolver, usize), String> {
    if index >= tokens.len() {
        return Err("No tokens to parse".to_string());
    }
    
    match &tokens[index] {
        // Action tokens
        UITokenType::Strike => Ok((ParsedResolver::Action(Box::new(StrikeActionNode)), 1)),
        UITokenType::Heal => Ok((ParsedResolver::Action(Box::new(HealActionNode)), 1)),
        
        // Condition tokens
        UITokenType::TrueOrFalse => Ok((ParsedResolver::Condition(Box::new(RandomConditionNode)), 1)),
        UITokenType::GreaterThan => parse_greater_than_condition(tokens, index),
        
        // Value tokens
        UITokenType::Number(n) => Ok((ParsedResolver::Value(Box::new(ConstantValueNode::new(*n as i32))), 1)),
        UITokenType::HP => parse_hp_value(tokens, index),
        
        // Complex tokens requiring additional context
        UITokenType::Check => parse_check_action(tokens, index),
        
        // Character tokens that must be used in context
        UITokenType::ActingCharacter => Err("ActingCharacter must be preceded by HP".to_string()),
        UITokenType::RandomCharacter => Err("RandomCharacter must be preceded by HP".to_string()),
    }
}

// GreaterThan条件をパース
fn parse_greater_than_condition(tokens: &[UITokenType], index: usize) -> Result<(ParsedResolver, usize), String> {
    if index + 2 >= tokens.len() {
        return Err("GreaterThan requires two operands".to_string());
    }
    
    let (left_parsed, left_consumed) = parse_ui_token(tokens, index + 1)?;
    let left_value = left_parsed.require_value()?;
    
    let (right_parsed, right_consumed) = parse_ui_token(tokens, index + 1 + left_consumed)?;
    let right_value = right_parsed.require_value()?;
    
    Ok((
        ParsedResolver::Condition(Box::new(GreaterThanConditionNode::new(left_value, right_value))),
        1 + left_consumed + right_consumed
    ))
}

// HP値をパース
fn parse_hp_value(tokens: &[UITokenType], index: usize) -> Result<(ParsedResolver, usize), String> {
    if index + 1 >= tokens.len() {
        return Err("HP must be followed by a Character token".to_string());
    }
    
    match &tokens[index + 1] {
        UITokenType::ActingCharacter => {
            Ok((ParsedResolver::Value(Box::new(CharacterHpFromNode::new(Box::new(ActingCharacterNode)))), 2))
        }
        UITokenType::RandomCharacter => {
            Ok((ParsedResolver::Value(Box::new(CharacterHpFromNode::new(Box::new(RandomCharacterNode::new())))), 2))
        }
        _ => Err("HP must be followed by a Character token (ActingCharacter or RandomCharacter)".to_string())
    }
}

// Check条件付きアクションをパース
fn parse_check_action(tokens: &[UITokenType], index: usize) -> Result<(ParsedResolver, usize), String> {
    if index + 1 >= tokens.len() {
        return Err("Check token requires condition".to_string());
    }
    
    let (condition_parsed, condition_consumed) = parse_ui_token(tokens, index + 1)?;
    let condition_node = condition_parsed.require_condition()?;
    
    let action_index = index + 1 + condition_consumed;
    if action_index >= tokens.len() {
        return Err("Check token requires an action".to_string());
    }
    
    let (action_parsed, action_consumed) = parse_ui_token(tokens, action_index)?;
    let action_node = action_parsed.require_action()?;
    
    Ok((
        ParsedResolver::Action(Box::new(ConditionCheckNode::new(condition_node, action_node))),
        1 + condition_consumed + action_consumed
    ))
}

// UIルールを直接RuleNodeに変換 - 新しい統一パーサーを使用
pub fn convert_ui_rules_to_nodes(ui_rules: &[Vec<UITokenType>]) -> Vec<RuleNode> {
    ui_rules
        .iter()
        .filter(|rule_row| !rule_row.is_empty())
        .filter_map(|rule_row| convert_ui_token_row_to_node(rule_row))
        .collect()
}

// UIトークン行を直接RuleNodeに変換 - 新しい統一パーサーを使用
fn convert_ui_token_row_to_node(token_row: &[UITokenType]) -> Option<RuleNode> {
    if token_row.is_empty() {
        return None;
    }
    
    // 統一パーサーを使用してActionResolverを取得
    match parse_ui_token(token_row, 0) {
        Ok((parsed, _consumed)) => {
            match parsed.require_action() {
                Ok(action_resolver) => Some(action_resolver),
                Err(_) => None, // ActionResolverでない場合はNone
            }
        }
        Err(_) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_action_token() {
        let tokens = vec![UITokenType::Strike];
        let result = parse_ui_token(&tokens, 0);
        
        assert!(result.is_ok());
        let (parsed, consumed) = result.unwrap();
        assert_eq!(consumed, 1);
        assert!(parsed.require_action().is_ok());
    }

    #[test]
    fn test_parse_condition_token() {
        let tokens = vec![UITokenType::TrueOrFalse];
        let result = parse_ui_token(&tokens, 0);
        
        assert!(result.is_ok());
        let (parsed, consumed) = result.unwrap();
        assert_eq!(consumed, 1);
        assert!(parsed.require_condition().is_ok());
    }

    #[test]
    fn test_parse_value_token() {
        let tokens = vec![UITokenType::Number(42)];
        let result = parse_ui_token(&tokens, 0);
        
        assert!(result.is_ok());
        let (parsed, consumed) = result.unwrap();
        assert_eq!(consumed, 1);
        assert!(parsed.require_value().is_ok());
    }

    #[test]
    fn test_parse_hp_value_with_character() {
        let tokens = vec![UITokenType::HP, UITokenType::ActingCharacter];
        let result = parse_ui_token(&tokens, 0);
        
        assert!(result.is_ok());
        let (parsed, consumed) = result.unwrap();
        assert_eq!(consumed, 2);
        assert!(parsed.require_value().is_ok());
    }

    #[test]
    fn test_parse_greater_than_condition() {
        let tokens = vec![
            UITokenType::GreaterThan,
            UITokenType::Number(50),
            UITokenType::HP,
            UITokenType::ActingCharacter,
        ];
        let result = parse_ui_token(&tokens, 0);
        
        assert!(result.is_ok());
        let (parsed, consumed) = result.unwrap();
        assert_eq!(consumed, 4);
        assert!(parsed.require_condition().is_ok());
    }

    #[test]
    fn test_parse_check_action() {
        let tokens = vec![
            UITokenType::Check,
            UITokenType::TrueOrFalse,
            UITokenType::Strike,
        ];
        let result = parse_ui_token(&tokens, 0);
        
        assert!(result.is_ok());
        let (parsed, consumed) = result.unwrap();
        assert_eq!(consumed, 3);
        assert!(parsed.require_action().is_ok());
    }

    #[test]
    fn test_require_type_mismatch_error() {
        let tokens = vec![UITokenType::Strike];
        let result = parse_ui_token(&tokens, 0);
        
        assert!(result.is_ok());
        let (parsed, _) = result.unwrap();
        
        // Strikeはアクションなので、条件として要求するとエラー
        assert!(parsed.require_condition().is_err());
    }

    #[test]
    fn test_convert_simple_ui_rule() {
        let ui_rules = vec![
            vec![UITokenType::Strike],
        ];
        
        let rule_nodes = convert_ui_rules_to_nodes(&ui_rules);
        assert_eq!(rule_nodes.len(), 1);
    }

    #[test]
    fn test_convert_complex_ui_rule() {
        let ui_rules = vec![
            vec![
                UITokenType::Check,
                UITokenType::GreaterThan,
                UITokenType::Number(50),
                UITokenType::HP,
                UITokenType::ActingCharacter,
                UITokenType::Heal,
            ],
        ];
        
        let rule_nodes = convert_ui_rules_to_nodes(&ui_rules);
        assert_eq!(rule_nodes.len(), 1);
    }

    #[test]
    fn test_convert_random_character_rule() {
        let ui_rules = vec![
            vec![
                UITokenType::Check,
                UITokenType::GreaterThan,
                UITokenType::Number(30),
                UITokenType::HP,
                UITokenType::RandomCharacter,
                UITokenType::Heal,
            ],
        ];
        
        let rule_nodes = convert_ui_rules_to_nodes(&ui_rules);
        assert_eq!(rule_nodes.len(), 1);
    }
}