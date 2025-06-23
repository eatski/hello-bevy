use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RuleSet {
    pub rules: Vec<RuleChain>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RuleChain {
    pub tokens: Vec<TokenConfig>,
}

#[derive(Debug, Clone)]
pub struct ValidatedRuleChain {
    pub tokens: Vec<TokenConfig>,
}

impl ValidatedRuleChain {
    /// Validates that continue tokens are followed by at least one more token
    pub fn from_rule_chain(rule_chain: &RuleChain) -> Result<Self, String> {
        Self::validate_token_sequence(&rule_chain.tokens)?;
        Ok(ValidatedRuleChain {
            tokens: rule_chain.tokens.clone(),
        })
    }
    
    fn validate_token_sequence(tokens: &[TokenConfig]) -> Result<(), String> {
        for (i, token) in tokens.iter().enumerate() {
            match token {
                TokenConfig::Check { args } => {
                    // Check token can continue, validate it's not at the end
                    if i == tokens.len() - 1 {
                        return Err(format!(
                            "Check token at position {} cannot be the last token in a rule chain. \
                            Check tokens may continue execution and require subsequent tokens.",
                            i
                        ));
                    }
                    
                    // Recursively validate nested tokens
                    for arg in args {
                        Self::validate_single_token(arg)?;
                    }
                },
                TokenConfig::GreaterThan { args } => {
                    // GreaterThan token can continue, validate it's not at the end
                    if i == tokens.len() - 1 {
                        return Err(format!(
                            "GreaterThan token at position {} cannot be the last token in a rule chain. \
                            GreaterThan tokens may continue execution and require subsequent tokens.",
                            i
                        ));
                    }
                    
                    // Recursively validate nested tokens
                    for arg in args {
                        Self::validate_single_token(arg)?;
                    }
                },
                TokenConfig::TrueOrFalseRandom => {
                    // TrueOrFalseRandom can continue, validate it's not at the end
                    if i == tokens.len() - 1 {
                        return Err(format!(
                            "TrueOrFalseRandom token at position {} cannot be the last token in a rule chain. \
                            TrueOrFalseRandom tokens may continue execution and require subsequent tokens.",
                            i
                        ));
                    }
                },
                // Other tokens don't continue, so they're fine anywhere
                _ => {}
            }
        }
        Ok(())
    }
    
    fn validate_single_token(token: &TokenConfig) -> Result<(), String> {
        match token {
            TokenConfig::Check { args } => {
                for arg in args {
                    Self::validate_single_token(arg)?;
                }
            },
            TokenConfig::GreaterThan { args } => {
                for arg in args {
                    Self::validate_single_token(arg)?;
                }
            },
            // Other single tokens are always valid
            _ => {}
        }
        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(tag = "type")]
pub enum TokenConfig {
    Strike,
    Heal,
    TrueOrFalseRandom,
    ActingCharacter,
    Check {
        args: Vec<TokenConfig>,
    },
    GreaterThan {
        args: Vec<TokenConfig>,
    },
    Number {
        value: i32,
    },
    CharacterHP,
}

