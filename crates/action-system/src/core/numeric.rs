use crate::core::character_hp::CharacterHP;

/// Trait for numeric values that can be compared and used in game calculations
/// This allows CharacterHP and i32 to be used uniformly in Max, Min, GreaterThan operations
pub trait Numeric: Send + Sync + 'static {
    /// Convert to i32 for comparison operations
    fn to_i32(&self) -> i32;
    
    /// Clone the value as a boxed trait object
    fn clone_box(&self) -> Box<dyn Numeric>;
}

impl Numeric for i32 {
    fn to_i32(&self) -> i32 {
        *self
    }
    
    fn clone_box(&self) -> Box<dyn Numeric> {
        Box::new(*self)
    }
}

impl Numeric for CharacterHP {
    fn to_i32(&self) -> i32 {
        self.get_hp()
    }
    
    fn clone_box(&self) -> Box<dyn Numeric> {
        Box::new(self.clone())
    }
}

impl Numeric for crate::Character {
    fn to_i32(&self) -> i32 {
        self.hp
    }
    
    fn clone_box(&self) -> Box<dyn Numeric> {
        Box::new(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Character;
    use std::cmp::Ordering;

    #[test]
    fn test_i32_numeric() {
        let a = 10i32;
        let b = 20i32;
        
        assert_eq!(a.to_i32(), 10);
        assert_eq!(b.to_i32(), 20);
        
        let a_box: Box<dyn Numeric> = a.clone_box();
        let b_box: Box<dyn Numeric> = b.clone_box();
        assert_eq!(a_box.to_i32(), 10);
        assert_eq!(b_box.to_i32(), 20);
    }
    
    #[test]
    fn test_character_hp_numeric() {
        let char1 = Character::new(1, "Test1".to_string(), 100, 100, 10);
        let char2 = Character::new(2, "Test2".to_string(), 100, 100, 10);
        
        let hp1 = CharacterHP::from_character_with_hp(char1, 80);
        let hp2 = CharacterHP::from_character_with_hp(char2, 60);
        
        assert_eq!(hp1.to_i32(), 80);
        assert_eq!(hp2.to_i32(), 60);
        
        let hp1_box: Box<dyn Numeric> = hp1.clone_box();
        let hp2_box: Box<dyn Numeric> = hp2.clone_box();
        assert_eq!(hp1_box.to_i32(), 80);
        assert_eq!(hp2_box.to_i32(), 60);
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