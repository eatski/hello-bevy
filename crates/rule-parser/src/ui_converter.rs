// UI converter - converts UI tokens to rule-system format

use crate::rule_input_model::{RuleSet, RuleChain, TokenConfig};
use crate::rule_loader::convert_to_node_rules;
use combat_engine::RuleNode;

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

// UIルールをaction-systemのRuleNodeに変換
pub fn convert_ui_rules_to_nodes(ui_rules: &[Vec<UITokenType>]) -> Vec<RuleNode> {
    let rule_chains: Vec<RuleChain> = ui_rules
        .iter()
        .filter(|rule_row| !rule_row.is_empty())
        .filter_map(|rule_row| convert_ui_token_row_to_rule_chain(rule_row))
        .collect();
    
    let rule_set = RuleSet { rules: rule_chains };
    
    convert_to_node_rules(&rule_set).unwrap_or_default()
}

// UIトークン行をrule-systemのRuleChainに変換（再帰実装）
fn convert_ui_token_row_to_rule_chain(token_row: &[UITokenType]) -> Option<RuleChain> {
    if token_row.is_empty() {
        return None;
    }
    
    let mut token_configs = Vec::new();
    convert_tokens_recursive(token_row, 0, &mut token_configs)?;
    
    if token_configs.is_empty() {
        None
    } else {
        Some(RuleChain { tokens: token_configs })
    }
}

// トークン変換を再帰的に実行
fn convert_tokens_recursive(
    token_row: &[UITokenType], 
    index: usize, 
    token_configs: &mut Vec<TokenConfig>
) -> Option<()> {
    if index >= token_row.len() {
        return Some(());
    }
    
    match &token_row[index] {
        UITokenType::Check => {
            if index + 1 < token_row.len() {
                if let Some((condition_config, consumed)) = 
                    convert_ui_condition_tokens_recursive(&token_row[index + 1..]) {
                    token_configs.push(TokenConfig::Check {
                        args: vec![condition_config],
                    });
                    convert_tokens_recursive(token_row, index + consumed + 1, token_configs)
                } else {
                    None
                }
            } else {
                None
            }
        }
        UITokenType::Strike => {
            token_configs.push(TokenConfig::Strike);
            convert_tokens_recursive(token_row, index + 1, token_configs)
        }
        UITokenType::Heal => {
            token_configs.push(TokenConfig::Heal);
            convert_tokens_recursive(token_row, index + 1, token_configs)
        }
        _ => None,
    }
}

// UI条件部分をTokenConfigに変換し、消費したトークン数も返す（再帰実装）
fn convert_ui_condition_tokens_recursive(tokens: &[UITokenType]) -> Option<(TokenConfig, usize)> {
    parse_condition_recursive(tokens, 0)
}

// 条件トークンを再帰的に解析
fn parse_condition_recursive(tokens: &[UITokenType], index: usize) -> Option<(TokenConfig, usize)> {
    if index >= tokens.len() {
        return None;
    }
    
    match &tokens[index] {
        UITokenType::TrueOrFalse => {
            Some((TokenConfig::TrueOrFalseRandom, 1))
        }
        UITokenType::GreaterThan => {
            parse_binary_operator_recursive(tokens, index)
        }
        _ => None,
    }
}

// 二項演算子を再帰的に解析
fn parse_binary_operator_recursive(tokens: &[UITokenType], index: usize) -> Option<(TokenConfig, usize)> {
    if index + 2 >= tokens.len() {
        return None;
    }
    
    let (left, left_consumed) = parse_value_token_recursive(tokens, index + 1)?;
    let (right, right_consumed) = parse_value_token_recursive(tokens, index + 1 + left_consumed)?;
    
    Some((
        TokenConfig::GreaterThan {
            args: vec![left, right],
        },
        1 + left_consumed + right_consumed
    ))
}

// 値トークンを再帰的に解析（消費トークン数も返す）
fn parse_value_token_recursive(tokens: &[UITokenType], index: usize) -> Option<(TokenConfig, usize)> {
    if index >= tokens.len() {
        return None;
    }
    
    match &tokens[index] {
        UITokenType::Number(n) => Some((TokenConfig::Number { value: *n as i32 }, 1)),
        UITokenType::HP => Some((TokenConfig::CharacterHP, 1)),
        _ => None,
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