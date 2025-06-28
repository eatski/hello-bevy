use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RuleSet {
    pub rules: Vec<RuleChain>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RuleChain {
    pub tokens: Vec<TokenConfig>,
}


#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(tag = "type")]
pub enum TokenConfig {
    Strike,
    Heal,
    TrueOrFalseRandom,
    Check {
        condition: Box<TokenConfig>,
        then_action: Box<TokenConfig>,
    },
    GreaterThan {
        left: Box<TokenConfig>,
        right: Box<TokenConfig>,
    },
    Number {
        value: i32,
    },
    CharacterHP,
}

