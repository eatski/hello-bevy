// UI converter - converts UI tokens directly to nodes

use combat_engine::{RuleNode, ConditionCheckNode, ActionResolver, ConditionNode, ValueNode, ConstantValueNode, ActingCharacterNode, CharacterHpFromNode, RandomConditionNode, GreaterThanConditionNode, StrikeActionNode, HealActionNode};

// UI側のトークンタイプの定義
#[derive(Clone, Debug, PartialEq)]
pub enum UITokenType {
    Check,
    Strike,
    Heal,
    Number(u32),
    HP,
    GreaterThan,
    TrueOrFalse,
}

impl UITokenType {
    pub fn display_text(&self) -> &str {
        match self {
            UITokenType::Check => "Check",
            UITokenType::Strike => "Strike",
            UITokenType::Heal => "Heal",
            UITokenType::Number(n) => match n {
                50 => "50",
                _ => "Num",
            },
            UITokenType::HP => "HP",
            UITokenType::GreaterThan => "L-gt-R",
            UITokenType::TrueOrFalse => "50/50",
        }
    }
}

// UIルールを直接RuleNodeに変換
pub fn convert_ui_rules_to_nodes(ui_rules: &[Vec<UITokenType>]) -> Vec<RuleNode> {
    ui_rules
        .iter()
        .filter(|rule_row| !rule_row.is_empty())
        .filter_map(|rule_row| convert_ui_token_row_to_node(rule_row))
        .collect()
}

// UIトークン行を直接RuleNodeに変換
fn convert_ui_token_row_to_node(token_row: &[UITokenType]) -> Option<RuleNode> {
    if token_row.is_empty() {
        return None;
    }
    
    convert_ui_tokens_to_resolver(token_row, 0).ok()
}

// UIトークンを直接ActionResolverに変換（再帰実装）
fn convert_ui_tokens_to_resolver(token_row: &[UITokenType], index: usize) -> Result<Box<dyn ActionResolver>, String> {
    if index >= token_row.len() {
        return Err("No tokens to convert".to_string());
    }
    
    match &token_row[index] {
        UITokenType::Check => {
            if index + 1 < token_row.len() {
                let (condition_node, condition_consumed) = 
                    convert_ui_condition_tokens(&token_row[index + 1..])?;
                
                let action_index = index + 1 + condition_consumed;
                if action_index < token_row.len() {
                    let (action_node, _action_consumed) = 
                        convert_ui_action_token(&token_row[action_index])?;
                    Ok(Box::new(ConditionCheckNode::new(condition_node, action_node)))
                } else {
                    Err("Check token requires an action".to_string())
                }
            } else {
                Err("Check token requires condition".to_string())
            }
        }
        UITokenType::Strike => {
            Ok(Box::new(StrikeActionNode))
        }
        UITokenType::Heal => {
            Ok(Box::new(HealActionNode))
        }
        _ => Err(format!("Token {:?} cannot be used as action", token_row[index])),
    }
}

// UI条件部分を直接ConditionNodeに変換し、消費したトークン数も返す
fn convert_ui_condition_tokens(tokens: &[UITokenType]) -> Result<(Box<dyn ConditionNode>, usize), String> {
    parse_ui_condition(tokens, 0)
}

// 条件トークンを解析
fn parse_ui_condition(tokens: &[UITokenType], index: usize) -> Result<(Box<dyn ConditionNode>, usize), String> {
    if index >= tokens.len() {
        return Err("No condition tokens".to_string());
    }
    
    match &tokens[index] {
        UITokenType::TrueOrFalse => {
            Ok((Box::new(RandomConditionNode), 1))
        }
        UITokenType::GreaterThan => {
            parse_ui_binary_operator(tokens, index)
        }
        _ => Err(format!("Token {:?} cannot be used as condition", tokens[index])),
    }
}

// 二項演算子を解析
fn parse_ui_binary_operator(tokens: &[UITokenType], index: usize) -> Result<(Box<dyn ConditionNode>, usize), String> {
    if index + 2 >= tokens.len() {
        return Err("GreaterThan requires two operands".to_string());
    }
    
    let (left, left_consumed) = parse_ui_value_token(tokens, index + 1)?;
    let (right, right_consumed) = parse_ui_value_token(tokens, index + 1 + left_consumed)?;
    
    Ok((
        Box::new(GreaterThanConditionNode::new(left, right)),
        1 + left_consumed + right_consumed
    ))
}

// 値トークンを解析（消費トークン数も返す）
fn parse_ui_value_token(tokens: &[UITokenType], index: usize) -> Result<(Box<dyn ValueNode>, usize), String> {
    if index >= tokens.len() {
        return Err("No value tokens".to_string());
    }
    
    match &tokens[index] {
        UITokenType::Number(n) => Ok((Box::new(ConstantValueNode::new(*n as i32)), 1)),
        UITokenType::HP => Ok((Box::new(CharacterHpFromNode::new(Box::new(ActingCharacterNode))), 1)),
        _ => Err(format!("Token {:?} cannot be used as value", tokens[index])),
    }
}

// UIアクショントークンを変換（消費トークン数も返す）
fn convert_ui_action_token(token: &UITokenType) -> Result<(Box<dyn ActionResolver>, usize), String> {
    match token {
        UITokenType::Strike => Ok((Box::new(StrikeActionNode), 1)),
        UITokenType::Heal => Ok((Box::new(HealActionNode), 1)),
        _ => Err(format!("Token {:?} cannot be used as action", token)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
                UITokenType::Heal,
            ],
        ];
        
        let rule_nodes = convert_ui_rules_to_nodes(&ui_rules);
        assert_eq!(rule_nodes.len(), 1);
    }
}