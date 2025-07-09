use crate::Character;
use std::ops::{Add, Sub, Mul, Div};
use std::cmp::{PartialEq, Eq, PartialOrd, Ord, Ordering};
use std::fmt;

#[derive(Clone, Debug)]
pub struct CharacterHP {
    pub character: Character,
    pub hp_value: i32,
}

impl CharacterHP {
    pub fn new(character: Character) -> Self {
        let hp_value = character.hp;
        Self { character, hp_value }
    }

    pub fn from_character_with_hp(character: Character, hp_value: i32) -> Self {
        Self { character, hp_value }
    }

    pub fn get_character(&self) -> &Character {
        &self.character
    }

    pub fn get_hp(&self) -> i32 {
        self.hp_value
    }

    pub fn set_hp(&mut self, hp: i32) {
        self.hp_value = hp;
    }
}

// 数値としての基本操作
impl From<CharacterHP> for i32 {
    fn from(char_hp: CharacterHP) -> Self {
        char_hp.hp_value
    }
}

impl From<&CharacterHP> for i32 {
    fn from(char_hp: &CharacterHP) -> Self {
        char_hp.hp_value
    }
}

// 四則演算のサポート
impl Add<i32> for CharacterHP {
    type Output = CharacterHP;

    fn add(mut self, rhs: i32) -> Self::Output {
        self.hp_value += rhs;
        self
    }
}

impl Add<CharacterHP> for CharacterHP {
    type Output = CharacterHP;

    fn add(mut self, rhs: CharacterHP) -> Self::Output {
        self.hp_value += rhs.hp_value;
        self
    }
}

impl Sub<i32> for CharacterHP {
    type Output = CharacterHP;

    fn sub(mut self, rhs: i32) -> Self::Output {
        self.hp_value -= rhs;
        self
    }
}

impl Sub<CharacterHP> for CharacterHP {
    type Output = CharacterHP;

    fn sub(mut self, rhs: CharacterHP) -> Self::Output {
        self.hp_value -= rhs.hp_value;
        self
    }
}

impl Mul<i32> for CharacterHP {
    type Output = CharacterHP;

    fn mul(mut self, rhs: i32) -> Self::Output {
        self.hp_value *= rhs;
        self
    }
}

impl Div<i32> for CharacterHP {
    type Output = CharacterHP;

    fn div(mut self, rhs: i32) -> Self::Output {
        self.hp_value /= rhs;
        self
    }
}

// 比較演算
impl PartialEq for CharacterHP {
    fn eq(&self, other: &Self) -> bool {
        self.hp_value == other.hp_value
    }
}

impl Eq for CharacterHP {}

impl PartialOrd for CharacterHP {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CharacterHP {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hp_value.cmp(&other.hp_value)
    }
}

impl PartialEq<i32> for CharacterHP {
    fn eq(&self, other: &i32) -> bool {
        self.hp_value == *other
    }
}

impl PartialOrd<i32> for CharacterHP {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.hp_value.partial_cmp(other)
    }
}

impl fmt::Display for CharacterHP {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.hp_value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_character_hp_creation() {
        let character = Character::new(1, "Test".to_string(), 100, 50, 25);
        let char_hp = CharacterHP::new(character.clone());
        
        assert_eq!(char_hp.get_hp(), 100);
        assert_eq!(char_hp.get_character().id, 1);
        assert_eq!(char_hp.get_character().name, "Test");
    }

    #[test]
    fn test_character_hp_from_character_with_hp() {
        let character = Character::new(1, "Test".to_string(), 100, 50, 25);
        let char_hp = CharacterHP::from_character_with_hp(character.clone(), 80);
        
        assert_eq!(char_hp.get_hp(), 80);
        assert_eq!(char_hp.get_character().id, 1);
    }

    #[test]
    fn test_character_hp_as_i32() {
        let character = Character::new(1, "Test".to_string(), 100, 50, 25);
        let char_hp = CharacterHP::new(character);
        
        let value: i32 = char_hp.into();
        assert_eq!(value, 100);
    }

    #[test]
    fn test_character_hp_arithmetic() {
        let character = Character::new(1, "Test".to_string(), 100, 50, 25);
        let char_hp = CharacterHP::new(character);
        
        let result = char_hp + 10;
        assert_eq!(result.get_hp(), 110);
        
        let result = result - 30;
        assert_eq!(result.get_hp(), 80);
        
        let result = result * 2;
        assert_eq!(result.get_hp(), 160);
        
        let result = result / 4;
        assert_eq!(result.get_hp(), 40);
    }

    #[test]
    fn test_character_hp_comparison() {
        let character1 = Character::new(1, "Test1".to_string(), 100, 50, 25);
        let character2 = Character::new(2, "Test2".to_string(), 80, 40, 20);
        
        let char_hp1 = CharacterHP::new(character1);
        let char_hp2 = CharacterHP::new(character2);
        
        assert!(char_hp1 > char_hp2);
        assert!(char_hp2 < char_hp1);
        assert!(char_hp1 == 100);
        assert!(char_hp2 == 80);
    }

    #[test]
    fn test_character_hp_display() {
        let character = Character::new(1, "Test".to_string(), 100, 50, 25);
        let char_hp = CharacterHP::new(character);
        
        assert_eq!(format!("{}", char_hp), "100");
    }
}