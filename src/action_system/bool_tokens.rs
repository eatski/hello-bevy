// Boolean tokens - tokens that evaluate to true/false

use rand::Rng;
use super::number_tokens::NumberToken;

// Trait for tokens that evaluate to boolean
pub trait BoolToken: Send + Sync + std::fmt::Debug {
    fn evaluate(&self, character: &crate::battle_system::Character, rng: &mut dyn rand::RngCore) -> bool;
}

impl BoolToken for Box<dyn BoolToken> {
    fn evaluate(&self, character: &crate::battle_system::Character, rng: &mut dyn rand::RngCore) -> bool {
        (**self).evaluate(character, rng)
    }
}

// Concrete bool token implementations
#[derive(Debug)]
pub struct TrueOrFalseRandomToken;

impl BoolToken for TrueOrFalseRandomToken {
    fn evaluate(&self, _character: &crate::battle_system::Character, rng: &mut dyn rand::RngCore) -> bool {
        rng.gen_bool(0.5)
    }
}

#[derive(Debug)]
pub struct GreaterThanToken {
    pub left: Box<dyn NumberToken>,
    pub right: Box<dyn NumberToken>,
}

impl GreaterThanToken {
    pub fn new(left: Box<dyn NumberToken>, right: Box<dyn NumberToken>) -> Self {
        Self { left, right }
    }
}

impl BoolToken for GreaterThanToken {
    fn evaluate(&self, character: &crate::battle_system::Character, rng: &mut dyn rand::RngCore) -> bool {
        self.left.evaluate(character, rng) > self.right.evaluate(character, rng)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::battle_system::Character;
    use crate::action_system::{ConstantToken};
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_true_or_false_random() {
        let character = Character::new("Test".to_string(), 100, 50, 25);
        let random = TrueOrFalseRandomToken;
        
        // Test with seeded RNG for deterministic behavior
        let mut rng1 = StdRng::seed_from_u64(42);
        let mut rng2 = StdRng::seed_from_u64(42);
        let result1 = random.evaluate(&character, &mut rng1);
        let result2 = random.evaluate(&character, &mut rng2);
        
        // Same seed should produce same result
        assert_eq!(result1, result2);
        
        // Test with random RNG for variety
        let mut rng = StdRng::from_entropy();
        let mut true_count = 0;
        let mut false_count = 0;
        
        for _ in 0..100 {
            if random.evaluate(&character, &mut rng) {
                true_count += 1;
            } else {
                false_count += 1;
            }
        }
        
        assert!(true_count > 0, "Should have some true results");
        assert!(false_count > 0, "Should have some false results");
    }

    #[test]
    fn test_seeded_random_deterministic() {
        let character = Character::new("Test".to_string(), 100, 50, 25);
        let random = TrueOrFalseRandomToken;
        
        // Test deterministic behavior with seed
        let seed = 12345;
        let mut rng1 = StdRng::seed_from_u64(seed);
        let mut rng2 = StdRng::seed_from_u64(seed);
        
        let result1 = random.evaluate(&character, &mut rng1);
        let result2 = random.evaluate(&character, &mut rng2);
        
        assert_eq!(result1, result2);
    }

    #[test]
    fn test_greater_than_token() {
        let character = Character::new("Test".to_string(), 100, 50, 25);
        let mut rng = StdRng::from_entropy();
        
        // Test GreaterThanToken
        let greater_than_token = GreaterThanToken::new(
            Box::new(ConstantToken::new(60)),
            Box::new(ConstantToken::new(40)),
        );
        assert_eq!(greater_than_token.evaluate(&character, &mut rng), true);
        
        let greater_than_token_false = GreaterThanToken::new(
            Box::new(ConstantToken::new(30)),
            Box::new(ConstantToken::new(50)),
        );
        assert_eq!(greater_than_token_false.evaluate(&character, &mut rng), false);
    }
}