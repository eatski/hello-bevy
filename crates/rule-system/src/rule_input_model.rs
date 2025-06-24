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

