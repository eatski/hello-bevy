use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

pub trait Token: Send + Sync {
    fn evaluate(&self, character: &crate::battle_system::Character, rng: &mut dyn rand::RngCore) -> TokenResult;
}

#[derive(Clone, Debug)]
pub enum TokenResult {
    Continue(bool),
    Action(ActionType),
    Break,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ActionType {
    Strike,
    Heal,
}

pub struct Check<T: Token> {
    condition: T,
}

impl<T: Token> Check<T> {
    pub fn new(condition: T) -> Self {
        Self { condition }
    }
}

impl<T: Token> Token for Check<T> {
    fn evaluate(&self, character: &crate::battle_system::Character, rng: &mut dyn rand::RngCore) -> TokenResult {
        match self.condition.evaluate(character, rng) {
            TokenResult::Continue(true) => TokenResult::Continue(true),
            _ => TokenResult::Break,
        }
    }
}

pub struct TrueOrFalseRandom;

impl Token for TrueOrFalseRandom {
    fn evaluate(&self, _character: &crate::battle_system::Character, rng: &mut dyn rand::RngCore) -> TokenResult {
        TokenResult::Continue(rng.gen_bool(0.5))
    }
}

pub struct Strike;

impl Token for Strike {
    fn evaluate(&self, character: &crate::battle_system::Character, _rng: &mut dyn rand::RngCore) -> TokenResult {
        if character.hp > 0 {
            TokenResult::Action(ActionType::Strike)
        } else {
            TokenResult::Break
        }
    }
}

pub struct Heal;

impl Token for Heal {
    fn evaluate(&self, character: &crate::battle_system::Character, _rng: &mut dyn rand::RngCore) -> TokenResult {
        if character.hp > 0 && character.mp >= 10 {
            TokenResult::Action(ActionType::Heal)
        } else {
            TokenResult::Break
        }
    }
}

pub struct ActionCalculationSystem {
    pub rules: Vec<Vec<Box<dyn Token>>>,
    pub rng: StdRng,
}

impl ActionCalculationSystem {
    pub fn new() -> Self {
        Self {
            rules: vec![
                vec![
                    Box::new(Check::new(TrueOrFalseRandom)),
                    Box::new(Heal),
                ],
                vec![Box::new(Strike)],
            ],
            rng: StdRng::from_entropy(),
        }
    }

    pub fn with_seed(seed: u64) -> Self {
        Self {
            rules: vec![
                vec![
                    Box::new(Check::new(TrueOrFalseRandom)),
                    Box::new(Heal),
                ],
                vec![Box::new(Strike)],
            ],
            rng: StdRng::seed_from_u64(seed),
        }
    }

    pub fn calculate_action(&mut self, character: &crate::battle_system::Character) -> Option<ActionType> {
        let rng = &mut self.rng;

        for rule_line in &self.rules {
            let mut should_continue = true;
            let mut action_result = None;

            for token in rule_line {
                if !should_continue {
                    break;
                }

                match token.evaluate(character, rng) {
                    TokenResult::Continue(true) => {
                        should_continue = true;
                    }
                    TokenResult::Continue(false) => {
                        should_continue = false;
                        break;
                    }
                    TokenResult::Action(action) => {
                        action_result = Some(action);
                        break;
                    }
                    TokenResult::Break => {
                        should_continue = false;
                        break;
                    }
                }
            }

            if let Some(action) = action_result {
                return Some(action);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::battle_system::Character;

    #[test]
    fn test_strike_token() {
        let character = Character::new("Test".to_string(), 100, 50, 25, true);
        let strike = Strike;
        let mut rng = StdRng::from_entropy();
        
        match strike.evaluate(&character, &mut rng) {
            TokenResult::Action(ActionType::Strike) => assert!(true),
            _ => panic!("Strike should return Action(Strike) for alive character"),
        }
        
        let dead_character = Character::new("Dead".to_string(), 0, 0, 25, true);
        match strike.evaluate(&dead_character, &mut rng) {
            TokenResult::Break => assert!(true),
            _ => panic!("Strike should return Break for dead character"),
        }
    }

    #[test]
    fn test_heal_token() {
        let character = Character::new("Test".to_string(), 100, 50, 25, true);
        let heal = Heal;
        let mut rng = StdRng::from_entropy();
        
        match heal.evaluate(&character, &mut rng) {
            TokenResult::Action(ActionType::Heal) => assert!(true),
            _ => panic!("Heal should return Action(Heal) for alive character"),
        }
        
        let dead_character = Character::new("Dead".to_string(), 0, 0, 25, true);
        match heal.evaluate(&dead_character, &mut rng) {
            TokenResult::Break => assert!(true),
            _ => panic!("Heal should return Break for dead character"),
        }
    }

    #[test]
    fn test_true_or_false_random() {
        let character = Character::new("Test".to_string(), 100, 50, 25, true);
        let random = TrueOrFalseRandom;
        
        // Test with seeded RNG for deterministic behavior
        let mut rng1 = StdRng::seed_from_u64(42);
        let mut rng2 = StdRng::seed_from_u64(42);
        let result1 = random.evaluate(&character, &mut rng1);
        let result2 = random.evaluate(&character, &mut rng2);
        
        // Same seed should produce same result
        assert_eq!(
            std::mem::discriminant(&result1),
            std::mem::discriminant(&result2)
        );
        
        // Test with random RNG for variety
        let mut rng = StdRng::from_entropy();
        let mut true_count = 0;
        let mut false_count = 0;
        
        for _ in 0..100 {
            match random.evaluate(&character, &mut rng) {
                TokenResult::Continue(true) => true_count += 1,
                TokenResult::Continue(false) => false_count += 1,
                _ => panic!("TrueOrFalseRandom should only return Continue(bool)"),
            }
        }
        
        assert!(true_count > 0, "Should have some true results");
        assert!(false_count > 0, "Should have some false results");
    }

    #[test]
    fn test_check_token() {
        let character = Character::new("Test".to_string(), 100, 50, 25, true);
        let check_random = Check::new(TrueOrFalseRandom);
        let mut rng = StdRng::from_entropy();
        
        match check_random.evaluate(&character, &mut rng) {
            TokenResult::Continue(_) | TokenResult::Break => assert!(true),
            _ => panic!("Check(TrueOrFalseRandom) should return Continue or Break"),
        }
        
        let check_strike = Check::new(Strike);
        match check_strike.evaluate(&character, &mut rng) {
            TokenResult::Break => assert!(true),
            _ => panic!("Check(Strike) should return Break since Strike returns Action"),
        }
    }

    #[test]
    fn test_action_calculation_system() {
        let mut system = ActionCalculationSystem::new();
        let character = Character::new("Test".to_string(), 100, 50, 25, true);
        
        let action = system.calculate_action(&character);
        assert!(action.is_some(), "Should return some action");
        
        match action.unwrap() {
            ActionType::Strike | ActionType::Heal => assert!(true),
        }
    }

    #[test]
    fn test_seeded_random_deterministic() {
        let character = Character::new("Test".to_string(), 100, 50, 25, true);
        let random = TrueOrFalseRandom;
        
        // Test deterministic behavior with seed
        let seed = 12345;
        let mut rng1 = StdRng::seed_from_u64(seed);
        let mut rng2 = StdRng::seed_from_u64(seed);
        
        let result1 = random.evaluate(&character, &mut rng1);
        let result2 = random.evaluate(&character, &mut rng2);
        
        match (result1, result2) {
            (TokenResult::Continue(a), TokenResult::Continue(b)) => assert_eq!(a, b),
            _ => panic!("Both should return Continue with same boolean value"),
        }
    }

    #[test]
    fn test_action_system_with_seed() {
        let character = Character::new("Test".to_string(), 100, 50, 25, true);
        let mut damaged_character = character.clone();
        damaged_character.take_damage(50); // HP: 50/100
        
        // Test deterministic behavior with same seed
        let mut system1 = ActionCalculationSystem::with_seed(42);
        let mut system2 = ActionCalculationSystem::with_seed(42);
        
        // Same seed should produce same action
        let action1 = system1.calculate_action(&damaged_character);
        let action2 = system2.calculate_action(&damaged_character);
        assert_eq!(action1, action2, "Same seed should produce same action");
        
        // Test with different seeds to verify both Strike and Heal can occur
        let mut strike_count = 0;
        let mut heal_count = 0;
        
        // Test 10 different seeds
        for seed in 0..10 {
            let mut system = ActionCalculationSystem::with_seed(seed);
            if let Some(action) = system.calculate_action(&damaged_character) {
                match action {
                    ActionType::Strike => strike_count += 1,
                    ActionType::Heal => heal_count += 1,
                }
            }
        }
        
        assert!(strike_count >= 1, "Should have at least one Strike action across 10 seeds, got {}", strike_count);
        assert!(heal_count >= 1, "Should have at least one Heal action across 10 seeds, got {}", heal_count);
        assert_eq!(strike_count + heal_count, 10, "Should have 10 total actions");
    }
}