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

// UIトークン行をrule-systemのRuleChainに変換
fn convert_ui_token_row_to_rule_chain(token_row: &[UITokenType]) -> Option<RuleChain> {
    if token_row.is_empty() {
        return None;
    }
    
    let mut token_configs = Vec::new();
    let mut i = 0;
    
    while i < token_row.len() {
        match &token_row[i] {
            UITokenType::Check => {
                if i + 1 < token_row.len() {
                    if let Some((condition_config, consumed)) = convert_ui_condition_tokens_with_count(&token_row[i+1..]) {
                        token_configs.push(TokenConfig::Check {
                            args: vec![condition_config],
                        });
                        i += consumed + 1;
                    } else {
                        return None;
                    }
                } else {
                    return None;
                }
            }
            UITokenType::Strike => {
                token_configs.push(TokenConfig::Strike);
                i += 1;
            }
            UITokenType::Heal => {
                token_configs.push(TokenConfig::Heal);
                i += 1;
            }
            _ => {
                return None;
            }
        }
    }
    
    if token_configs.is_empty() {
        None
    } else {
        Some(RuleChain {
            tokens: token_configs,
        })
    }
}

// UI条件部分をTokenConfigに変換し、消費したトークン数も返す
fn convert_ui_condition_tokens_with_count(tokens: &[UITokenType]) -> Option<(TokenConfig, usize)> {
    if tokens.is_empty() {
        return None;
    }
    
    match &tokens[0] {
        UITokenType::TrueOrFalse => {
            Some((TokenConfig::TrueOrFalseRandom, 1))
        }
        UITokenType::GreaterThan => {
            if tokens.len() >= 3 {
                let left = convert_ui_single_token_to_config(&tokens[1])?;
                let right = convert_ui_single_token_to_config(&tokens[2])?;
                Some((
                    TokenConfig::GreaterThan {
                        args: vec![left, right],
                    },
                    3
                ))
            } else {
                None
            }
        }
        _ => None,
    }
}

// 単一のUIトークンをTokenConfigに変換
fn convert_ui_single_token_to_config(token: &UITokenType) -> Option<TokenConfig> {
    match token {
        UITokenType::Number(n) => Some(TokenConfig::Number { value: *n as i32 }),
        UITokenType::HP => Some(TokenConfig::CharacterHP),
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