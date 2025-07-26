use crate::Character;
use crate::core::character_hp::CharacterHP;

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