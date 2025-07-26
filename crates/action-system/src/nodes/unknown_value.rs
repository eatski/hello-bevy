use crate::Character;
use crate::core::character_hp::CharacterHP;
use std::convert::TryFrom;

/// Represents different types of values that can be used as unknown values in node evaluation
/// This is independent from EvaluationContext and can be used for array operations
#[derive(Debug, Clone)]
pub enum UnknownValue {
    /// A character value
    Character(Character),
    /// A numeric value
    Value(i32),
    /// A team side
    TeamSide(crate::TeamSide),
    /// A character HP value
    CharacterHP(CharacterHP),
}

/// Error type for UnknownValue conversions
#[derive(Debug, Clone)]
pub struct UnknownValueConversionError {
    pub expected: &'static str,
    pub actual: &'static str,
}

impl std::fmt::Display for UnknownValueConversionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Expected {} but got {}", self.expected, self.actual)
    }
}

impl std::error::Error for UnknownValueConversionError {}

// TryFrom implementations for each type
impl TryFrom<UnknownValue> for Character {
    type Error = UnknownValueConversionError;
    
    fn try_from(value: UnknownValue) -> Result<Self, Self::Error> {
        match value {
            UnknownValue::Character(character) => Ok(character),
            _ => Err(UnknownValueConversionError {
                expected: "Character",
                actual: value.type_name(),
            }),
        }
    }
}

impl TryFrom<UnknownValue> for i32 {
    type Error = UnknownValueConversionError;
    
    fn try_from(value: UnknownValue) -> Result<Self, Self::Error> {
        match value {
            UnknownValue::Value(val) => Ok(val),
            _ => Err(UnknownValueConversionError {
                expected: "i32",
                actual: value.type_name(),
            }),
        }
    }
}

impl TryFrom<UnknownValue> for crate::TeamSide {
    type Error = UnknownValueConversionError;
    
    fn try_from(value: UnknownValue) -> Result<Self, Self::Error> {
        match value {
            UnknownValue::TeamSide(side) => Ok(side),
            _ => Err(UnknownValueConversionError {
                expected: "TeamSide",
                actual: value.type_name(),
            }),
        }
    }
}

impl TryFrom<UnknownValue> for CharacterHP {
    type Error = UnknownValueConversionError;
    
    fn try_from(value: UnknownValue) -> Result<Self, Self::Error> {
        match value {
            UnknownValue::CharacterHP(hp) => Ok(hp),
            _ => Err(UnknownValueConversionError {
                expected: "CharacterHP",
                actual: value.type_name(),
            }),
        }
    }
}

impl UnknownValue {
    /// Returns the name of the type this value contains
    pub fn type_name(&self) -> &'static str {
        match self {
            UnknownValue::Character(_) => "Character",
            UnknownValue::Value(_) => "Value",
            UnknownValue::TeamSide(_) => "TeamSide",
            UnknownValue::CharacterHP(_) => "CharacterHP",
        }
    }
}