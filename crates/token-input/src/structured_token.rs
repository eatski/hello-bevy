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
    LessThan {
        left: Box<StructuredTokenInput>,
        right: Box<StructuredTokenInput>,
    },
    Number {
        value: i32,
    },
    CharacterToHp {
        character: Box<StructuredTokenInput>,
    },
    CharacterHpToCharacter {
        character_hp: Box<StructuredTokenInput>,
    },
    // Character types
    ActingCharacter,
    // Array types
    AllCharacters,
    TeamMembers {
        team_side: Box<StructuredTokenInput>,
    },
    AllTeamSides,
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
    Max {
        array: Box<StructuredTokenInput>,
    },
    Min {
        array: Box<StructuredTokenInput>,
    },
    // Unified Numeric operations
    NumericMax {
        array: Box<StructuredTokenInput>,
    },
    NumericMin {
        array: Box<StructuredTokenInput>,
    },
}

impl StructuredTokenInput {
    
    /// トークンの出力型を取得
    pub fn output_type(&self) -> crate::type_system::Type {
        use crate::type_system::Type;
        match self {
            Self::Strike { .. } | Self::Heal { .. } | Self::Check { .. } => Type::Action,
            Self::TrueOrFalseRandom | Self::GreaterThan { .. } | Self::LessThan { .. } | Self::Eq { .. } => Type::Bool,
            Self::ActingCharacter => Type::Character,
            Self::AllCharacters | Self::TeamMembers { .. } => Type::Vec(Box::new(Type::Character)),
            Self::AllTeamSides => Type::Vec(Box::new(Type::TeamSide)),
            Self::Enemy | Self::Hero => Type::TeamSide,
            Self::Number { .. } => Type::I32,
            Self::CharacterToHp { .. } => Type::CharacterHP,
            Self::CharacterHpToCharacter { .. } | Self::Max { .. } | Self::Min { .. } => Type::Character,
            Self::NumericMax { .. } | Self::NumericMin { .. } => Type::Numeric,
            Self::CharacterTeam { .. } => Type::TeamSide,
            // 動的な型は後で特殊処理
            Self::RandomPick { .. } | Self::FilterList { .. } | Self::Map { .. } | Self::Element => Type::Any,
        }
    }
    
    /// 引数を取得（引数名とトークンのペア）
    pub fn arguments(&self) -> Vec<(&'static str, &StructuredTokenInput)> {
        match self {
            Self::Strike { target } | Self::Heal { target } => 
                vec![("target", target.as_ref())],
            Self::Check { condition, then_action } => 
                vec![("condition", condition.as_ref()), ("then_action", then_action.as_ref())],
            Self::GreaterThan { left, right } | Self::LessThan { left, right } | Self::Eq { left, right } => 
                vec![("left", left.as_ref()), ("right", right.as_ref())],
            Self::CharacterToHp { character } | Self::CharacterTeam { character } => 
                vec![("character", character.as_ref())],
            Self::CharacterHpToCharacter { character_hp } => 
                vec![("character_hp", character_hp.as_ref())],
            Self::TeamMembers { team_side } => 
                vec![("team_side", team_side.as_ref())],
            Self::RandomPick { array } | Self::Max { array } | Self::Min { array } | 
            Self::NumericMax { array } | Self::NumericMin { array } => 
                vec![("array", array.as_ref())],
            Self::FilterList { array, condition } => 
                vec![("array", array.as_ref()), ("condition", condition.as_ref())],
            Self::Map { array, transform } => 
                vec![("array", array.as_ref()), ("transform", transform.as_ref())],
            _ => vec![],
        }
    }
    
    /// 数値フィールドを取得（Numberトークン用）
    pub fn get_number_value(&self) -> Option<i32> {
        match self {
            Self::Number { value } => Some(*value),
            _ => None,
        }
    }
    
    /// 引数の期待される型を取得
    pub fn expected_argument_types(&self) -> Vec<(&'static str, crate::type_system::Type)> {
        use crate::type_system::Type;
        match self {
            Self::Strike { .. } | Self::Heal { .. } => 
                vec![("target", Type::Character)],
            Self::Check { .. } => 
                vec![("condition", Type::Bool), ("then_action", Type::Action)],
            Self::GreaterThan { .. } | Self::LessThan { .. } => 
                vec![("left", Type::Numeric), ("right", Type::Numeric)],
            Self::Eq { .. } => 
                vec![("left", Type::Any), ("right", Type::Any)],
            Self::CharacterToHp { .. } | Self::CharacterTeam { .. } => 
                vec![("character", Type::Character)],
            Self::CharacterHpToCharacter { .. } => 
                vec![("character_hp", Type::CharacterHP)],
            Self::TeamMembers { .. } => 
                vec![("team_side", Type::TeamSide)],
            Self::RandomPick { .. } => 
                vec![("array", Type::Vec(Box::new(Type::Any)))],
            Self::FilterList { .. } => 
                vec![("array", Type::Vec(Box::new(Type::Any))), ("condition", Type::Bool)],
            Self::Map { .. } => 
                vec![("array", Type::Vec(Box::new(Type::Any))), ("transform", Type::Any)],
            Self::Max { .. } | Self::Min { .. } => 
                vec![("array", Type::Vec(Box::new(Type::Character)))],
            Self::NumericMax { .. } | Self::NumericMin { .. } => 
                vec![("array", Type::Vec(Box::new(Type::Numeric)))],
            _ => vec![],
        }
    }
}