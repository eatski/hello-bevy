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
    ActingCharacter,
    Check {
        #[serde(default)]
        condition: Option<Box<TokenConfig>>,
        #[serde(default)]
        args: Vec<TokenConfig>,
    },
    GreaterThan {
        #[serde(default)]
        left: Option<Box<TokenConfig>>,
        #[serde(default)]
        right: Option<Box<TokenConfig>>,
        #[serde(default)]
        args: Vec<TokenConfig>,
    },
    Number {
        value: i32,
    },
    CharacterHP {
        #[serde(default)]
        character: Option<String>,
        #[serde(default)]
        args: Vec<TokenConfig>,
    },
}