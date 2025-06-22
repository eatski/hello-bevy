use rand::Rng;
use rand::rngs::StdRng;

pub trait Token: Send + Sync {
    fn evaluate(&self, character: &crate::battle_system::Character, rng: &mut dyn rand::RngCore) -> TokenResult;
}

#[derive(Clone, Debug)]
pub enum TokenResult {
    Continue(bool),
    Action(ActionType),
    Break,
    Value(TokenValue),
}

#[derive(Clone, Debug, PartialEq)]
pub enum TokenValue {
    Number(i32),
    Character,
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

pub struct GreaterThanToken<A: Token, B: Token> {
    left: A,
    right: B,
}

impl<A: Token, B: Token> GreaterThanToken<A, B> {
    pub fn new(left: A, right: B) -> Self {
        Self { left, right }
    }
}

impl<A: Token, B: Token> Token for GreaterThanToken<A, B> {
    fn evaluate(&self, character: &crate::battle_system::Character, rng: &mut dyn rand::RngCore) -> TokenResult {
        let left_val = match self.left.evaluate(character, rng) {
            TokenResult::Value(TokenValue::Number(n)) => n,
            _ => return TokenResult::Break,
        };
        
        let right_val = match self.right.evaluate(character, rng) {
            TokenResult::Value(TokenValue::Number(n)) => n,
            _ => return TokenResult::Break,
        };
        
        TokenResult::Continue(left_val > right_val)
    }
}

pub struct Number {
    value: i32,
}

impl Number {
    pub fn new(value: i32) -> Self {
        Self { value: value.clamp(1, 100) }
    }
}

impl Token for Number {
    fn evaluate(&self, _character: &crate::battle_system::Character, _rng: &mut dyn rand::RngCore) -> TokenResult {
        TokenResult::Value(TokenValue::Number(self.value))
    }
}

pub struct CharacterHP<C: Token> {
    character_token: C,
}

impl<C: Token> CharacterHP<C> {
    pub fn new(character_token: C) -> Self {
        Self { character_token }
    }
}

impl<C: Token> Token for CharacterHP<C> {
    fn evaluate(&self, character: &crate::battle_system::Character, rng: &mut dyn rand::RngCore) -> TokenResult {
        match self.character_token.evaluate(character, rng) {
            TokenResult::Value(TokenValue::Character) => TokenResult::Value(TokenValue::Number(character.hp)),
            _ => TokenResult::Break,
        }
    }
}

pub struct SelfCharacter;

impl Token for SelfCharacter {
    fn evaluate(&self, _character: &crate::battle_system::Character, _rng: &mut dyn rand::RngCore) -> TokenResult {
        TokenResult::Value(TokenValue::Character)
    }
}

pub struct ActionCalculationSystem {
    pub rules: Vec<Vec<Box<dyn Token>>>,
    pub rng: StdRng,
}

impl ActionCalculationSystem {
    pub fn new(rules: Vec<Vec<Box<dyn Token>>>, rng: StdRng) -> Self {
        Self {
            rules,
            rng,
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
                        break;
                    }
                    TokenResult::Action(action) => {
                        action_result = Some(action);
                        break;
                    }
                    TokenResult::Break => {
                        break;
                    }
                    TokenResult::Value(_) => {
                        should_continue = true;
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
    use rand::SeedableRng;

    #[test]
    fn test_strike_token() {
        let character = Character::new("Test".to_string(), 100, 50, 25);
        let strike = Strike;
        let mut rng = StdRng::from_entropy();
        
        match strike.evaluate(&character, &mut rng) {
            TokenResult::Action(ActionType::Strike) => assert!(true),
            _ => panic!("Strike should return Action(Strike) for alive character"),
        }
        
        let dead_character = Character::new("Dead".to_string(), 0, 0, 25);
        match strike.evaluate(&dead_character, &mut rng) {
            TokenResult::Break => assert!(true),
            _ => panic!("Strike should return Break for dead character"),
        }
    }

    #[test]
    fn test_heal_token() {
        let character = Character::new("Test".to_string(), 100, 50, 25);
        let heal = Heal;
        let mut rng = StdRng::from_entropy();
        
        match heal.evaluate(&character, &mut rng) {
            TokenResult::Action(ActionType::Heal) => assert!(true),
            _ => panic!("Heal should return Action(Heal) for alive character"),
        }
        
        let dead_character = Character::new("Dead".to_string(), 0, 0, 25);
        match heal.evaluate(&dead_character, &mut rng) {
            TokenResult::Break => assert!(true),
            _ => panic!("Heal should return Break for dead character"),
        }
    }

    #[test]
    fn test_true_or_false_random() {
        let character = Character::new("Test".to_string(), 100, 50, 25);
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
        let character = Character::new("Test".to_string(), 100, 50, 25);
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
        let rules: Vec<Vec<Box<dyn Token>>> = vec![
            vec![
                Box::new(Check::new(TrueOrFalseRandom)),
                Box::new(Heal),
            ],
            vec![Box::new(Strike)],
        ];
        let rng = StdRng::from_entropy();
        let mut system = ActionCalculationSystem::new(rules, rng);
        let character = Character::new("Test".to_string(), 100, 50, 25);
        
        let action = system.calculate_action(&character);
        assert!(action.is_some(), "Should return some action");
        
        match action.unwrap() {
            ActionType::Strike | ActionType::Heal => assert!(true),
        }
    }

    #[test]
    fn test_seeded_random_deterministic() {
        let character = Character::new("Test".to_string(), 100, 50, 25);
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
        let character = Character::new("Test".to_string(), 100, 50, 25);
        let mut damaged_character = character.clone();
        damaged_character.take_damage(50); // HP: 50/100
        
        let create_rules = || -> Vec<Vec<Box<dyn Token>>> {
            vec![
                vec![
                    Box::new(Check::new(TrueOrFalseRandom)),
                    Box::new(Heal),
                ],
                vec![Box::new(Strike)],
            ]
        };
        
        // Test that the system can produce different actions
        let rng1 = StdRng::from_entropy();
        let rng2 = StdRng::from_entropy();
        let mut system1 = ActionCalculationSystem::new(create_rules(), rng1);
        let mut system2 = ActionCalculationSystem::new(create_rules(), rng2);
        
        // Test with multiple attempts to verify both Strike and Heal can occur
        let mut strike_count = 0;
        let mut heal_count = 0;
        
        // Test 20 attempts to get both actions
        for _ in 0..20 {
            if let Some(action) = system1.calculate_action(&damaged_character) {
                match action {
                    ActionType::Strike => strike_count += 1,
                    ActionType::Heal => heal_count += 1,
                }
            }
            if let Some(action) = system2.calculate_action(&damaged_character) {
                match action {
                    ActionType::Strike => strike_count += 1,
                    ActionType::Heal => heal_count += 1,
                }
            }
        }
        
        assert!(strike_count >= 1, "Should have at least one Strike action across attempts, got {}", strike_count);
        assert!(heal_count >= 1, "Should have at least one Heal action across attempts, got {}", heal_count);
        assert_eq!(strike_count + heal_count, 40, "Should have 40 total actions from 20 attempts with 2 systems");
    }

    #[test]
    fn test_new_tokens() {
        let character = Character::new("Test".to_string(), 100, 50, 25);
        let mut rng = StdRng::from_entropy();
        
        // Test Number token
        let number_token = Number::new(42);
        match number_token.evaluate(&character, &mut rng) {
            TokenResult::Value(TokenValue::Number(42)) => assert!(true),
            _ => panic!("Number token should return Value(Number(42))"),
        }
        
        // Test SelfCharacter token
        let self_char_token = SelfCharacter;
        match self_char_token.evaluate(&character, &mut rng) {
            TokenResult::Value(TokenValue::Character) => assert!(true),
            _ => panic!("SelfCharacter token should return Value(Character)"),
        }
        
        // Test CharacterHP token
        let char_hp_token = CharacterHP::new(SelfCharacter);
        match char_hp_token.evaluate(&character, &mut rng) {
            TokenResult::Value(TokenValue::Number(100)) => assert!(true),
            _ => panic!("CharacterHP token should return Value(Number(100))"),
        }
        
        // Test GreaterThanToken
        let greater_than_token = GreaterThanToken::new(Number::new(60), Number::new(40));
        match greater_than_token.evaluate(&character, &mut rng) {
            TokenResult::Continue(true) => assert!(true),
            _ => panic!("GreaterThanToken(60, 40) should return Continue(true)"),
        }
        
        let greater_than_token_false = GreaterThanToken::new(Number::new(30), Number::new(50));
        match greater_than_token_false.evaluate(&character, &mut rng) {
            TokenResult::Continue(false) => assert!(true),
            _ => panic!("GreaterThanToken(30, 50) should return Continue(false)"),
        }
    }

    #[test]
    fn test_hp_based_action_logic() {
        let mut low_hp_character = Character::new("LowHP".to_string(), 100, 50, 25);
        low_hp_character.take_damage(70); // HP: 30
        
        let high_hp_character = Character::new("HighHP".to_string(), 100, 50, 25);
        // HP: 100
        
        // Create HP-based rules
        let hp_rules: Vec<Vec<Box<dyn Token>>> = vec![
            vec![
                Box::new(Check::new(
                    GreaterThanToken::new(
                        Number::new(50),
                        CharacterHP::new(SelfCharacter),
                    )
                )),
                Box::new(Heal),
            ],
            vec![Box::new(Strike)],
        ];
        
        let rng = StdRng::from_entropy();
        let mut system = ActionCalculationSystem::new(hp_rules, rng);
        
        // Low HP character should heal
        let low_hp_action = system.calculate_action(&low_hp_character);
        assert_eq!(low_hp_action, Some(ActionType::Heal), "Low HP character should choose Heal");
        
        // High HP character should strike
        let high_hp_action = system.calculate_action(&high_hp_character);
        assert_eq!(high_hp_action, Some(ActionType::Strike), "High HP character should choose Strike");
    }
}

