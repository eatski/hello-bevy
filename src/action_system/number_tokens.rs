// Number tokens - tokens that evaluate to numeric values

// Trait for tokens that evaluate to numbers
pub trait NumberToken: Send + Sync + std::fmt::Debug {
    fn evaluate(&self, character: &crate::battle_system::Character, rng: &mut dyn rand::RngCore) -> i32;
}

impl NumberToken for Box<dyn NumberToken> {
    fn evaluate(&self, character: &crate::battle_system::Character, rng: &mut dyn rand::RngCore) -> i32 {
        (**self).evaluate(character, rng)
    }
}

// Concrete number token implementations
#[derive(Debug)]
pub struct ConstantToken {
    value: i32,
}

impl ConstantToken {
    pub fn new(value: i32) -> Self {
        Self { value: value.clamp(1, 100) }
    }
}

impl NumberToken for ConstantToken {
    fn evaluate(&self, _character: &crate::battle_system::Character, _rng: &mut dyn rand::RngCore) -> i32 {
        self.value
    }
}

#[derive(Debug)]
pub struct CharacterHPToken;

impl NumberToken for CharacterHPToken {
    fn evaluate(&self, character: &crate::battle_system::Character, _rng: &mut dyn rand::RngCore) -> i32 {
        character.hp
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::battle_system::Character;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_constant_token() {
        let character = Character::new("Test".to_string(), 100, 50, 25);
        let mut rng = StdRng::from_entropy();
        
        // Test Constant token
        let number_token = ConstantToken::new(42);
        assert_eq!(number_token.evaluate(&character, &mut rng), 42);
    }

    #[test]
    fn test_character_hp_token() {
        let character = Character::new("Test".to_string(), 100, 50, 25);
        let mut rng = StdRng::from_entropy();
        
        // Test CharacterHP token
        let char_hp_token = CharacterHPToken;
        assert_eq!(char_hp_token.evaluate(&character, &mut rng), 100);
    }
}