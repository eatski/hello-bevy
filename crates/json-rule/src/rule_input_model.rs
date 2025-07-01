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
    Strike {
        target: Box<JsonTokenInput>,
    },
    Heal {
        target: Box<JsonTokenInput>,
    },
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
    HP {
        character: Box<JsonTokenInput>,
    },
    // Character types
    ActingCharacter,
    RandomCharacter,
}

