// StructuredTokenInput - JSON入力用の構造化されたトークン定義（rule-parserから移行）

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RuleSet {
    pub rules: Vec<StructuredTokenInput>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(tag = "type")]
pub enum StructuredTokenInput {
    Strike {
        target: Box<StructuredTokenInput>,
    },
    Heal {
        target: Box<StructuredTokenInput>,
    },
    TrueOrFalseRandom,
    Check {
        condition: Box<StructuredTokenInput>,
        then_action: Box<StructuredTokenInput>,
    },
    GreaterThan {
        left: Box<StructuredTokenInput>,
        right: Box<StructuredTokenInput>,
    },
    Number {
        value: i32,
    },
    CharacterHP {
        character: Box<StructuredTokenInput>,
    },
    HP {
        character: Box<StructuredTokenInput>,
    },
    // Character types
    ActingCharacter,
    // Array types
    AllCharacters,
    RandomPick {
        array: Box<StructuredTokenInput>,
    },
    FilterList {
        array: Box<StructuredTokenInput>,
        condition: Box<StructuredTokenInput>,
    },
    Map {
        array: Box<StructuredTokenInput>,
        transform: Box<StructuredTokenInput>,
    },
    Eq {
        left: Box<StructuredTokenInput>,
        right: Box<StructuredTokenInput>,
    },
    CharacterTeam {
        character: Box<StructuredTokenInput>,
    },
    Element,
    Enemy,
    Hero,
}