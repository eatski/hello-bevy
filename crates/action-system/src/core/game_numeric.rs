use crate::core::character_hp::CharacterHP;
use std::cmp::Ordering;

/// Trait for numeric values that can be compared and used in game calculations
/// This allows CharacterHP and i32 to be used uniformly in Max, Min, GreaterThan operations
pub trait GameNumeric: Clone + PartialEq + PartialOrd + Send + Sync + 'static {
    /// Convert to i32 for comparison operations
    fn to_i32(&self) -> i32;
    
    /// Compare two GameNumeric values
    fn compare(&self, other: &Self) -> Ordering {
        self.to_i32().cmp(&other.to_i32())
    }
    
    /// Get maximum of two values
    fn max(self, other: Self) -> Self {
        if self.to_i32() >= other.to_i32() { self } else { other }
    }
    
    /// Get minimum of two values
    fn min(self, other: Self) -> Self {
        if self.to_i32() <= other.to_i32() { self } else { other }
    }
}

impl GameNumeric for i32 {
    fn to_i32(&self) -> i32 {
        *self
    }
}

impl GameNumeric for CharacterHP {
    fn to_i32(&self) -> i32 {
        self.get_hp()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Character;

    #[test]
    fn test_i32_game_numeric() {
        let a = 10i32;
        let b = 20i32;
        
        assert_eq!(a.to_i32(), 10);
        assert_eq!(b.to_i32(), 20);
        assert_eq!(GameNumeric::max(a, b), 20);
        assert_eq!(GameNumeric::min(a, b), 10);
        assert_eq!(a.compare(&b), Ordering::Less);
    }
    
    #[test]
    fn test_character_hp_game_numeric() {
        let char1 = Character::new(1, "Test1".to_string(), 100, 100, 10);
        let char2 = Character::new(2, "Test2".to_string(), 100, 100, 10);
        
        let hp1 = CharacterHP::from_character_with_hp(char1, 80);
        let hp2 = CharacterHP::from_character_with_hp(char2, 60);
        
        assert_eq!(hp1.to_i32(), 80);
        assert_eq!(hp2.to_i32(), 60);
        assert_eq!(GameNumeric::max(hp1.clone(), hp2.clone()).to_i32(), 80);
        assert_eq!(GameNumeric::min(hp1.clone(), hp2.clone()).to_i32(), 60);
        assert_eq!(hp1.compare(&hp2), Ordering::Greater);
    }
    
    #[test]
    fn test_mixed_comparisons() {
        let char1 = Character::new(1, "Test1".to_string(), 100, 100, 10);
        let hp1 = CharacterHP::from_character_with_hp(char1, 75);
        let value1 = 50i32;
        
        // Test that we can compare the underlying values
        assert_eq!(hp1.to_i32().cmp(&value1.to_i32()), Ordering::Greater);
        assert_eq!(value1.to_i32().cmp(&hp1.to_i32()), Ordering::Less);
    }
}