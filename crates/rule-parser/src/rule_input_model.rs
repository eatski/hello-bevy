use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RuleSet {
    pub rules: Vec<RuleChain>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RuleChain {
    pub tokens: Vec<JsonTokenInput>,
}


#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(tag = "type")]
pub enum JsonTokenInput {
    Strike,
    Heal,
    TrueOrFalseRandom,
    Check {
        condition: Box<JsonTokenInput>,
        then_action: Box<JsonTokenInput>,
    },
    GreaterThan {
        left: Box<JsonTokenInput>,
        right: Box<JsonTokenInput>,
    },
    Number {
        value: i32,
    },
    CharacterHP,
}

