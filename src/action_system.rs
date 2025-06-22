use rand::Rng;
use rand::rngs::StdRng;

// Trait for tokens that can resolve to actions or break
pub trait ActionResolver: Send + Sync + std::fmt::Debug {
    fn resolve(&self, character: &crate::battle_system::Character, rng: &mut dyn rand::RngCore) -> ActionResolverResult;
}

impl ActionResolver for Box<dyn ActionResolver> {
    fn resolve(&self, character: &crate::battle_system::Character, rng: &mut dyn rand::RngCore) -> ActionResolverResult {
        (**self).resolve(character, rng)
    }
}

#[derive(Clone, Debug)]
pub enum ActionResolverResult {
    Action(ActionType),  // 行はActionを決定
    Break,               // 行を中断
}



#[derive(Clone, Debug, PartialEq)]
pub enum ActionType {
    Strike,
    Heal,
}

// Check token type - evaluates condition and delegates to next token or breaks
#[derive(Debug)]
pub struct CheckToken {
    condition: Box<dyn BoolToken>,
    next: Box<dyn ActionResolver>,
}

impl CheckToken {
    pub fn new(condition: Box<dyn BoolToken>, next: Box<dyn ActionResolver>) -> Self {
        Self { condition, next }
    }
}

impl ActionResolver for CheckToken {
    fn resolve(&self, character: &crate::battle_system::Character, rng: &mut dyn rand::RngCore) -> ActionResolverResult {
        if self.condition.evaluate(character, rng) {
            // Continue: delegate to next token
            self.next.resolve(character, rng)
        } else {
            ActionResolverResult::Break
        }
    }
}


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



// Action token types
#[derive(Debug)]
pub struct StrikeAction;

impl ActionResolver for StrikeAction {
    fn resolve(&self, character: &crate::battle_system::Character, _rng: &mut dyn rand::RngCore) -> ActionResolverResult {
        if character.hp > 0 {
            ActionResolverResult::Action(ActionType::Strike)
        } else {
            ActionResolverResult::Break
        }
    }
}

#[derive(Debug)]
pub struct HealAction;

impl ActionResolver for HealAction {
    fn resolve(&self, character: &crate::battle_system::Character, _rng: &mut dyn rand::RngCore) -> ActionResolverResult {
        if character.hp > 0 && character.mp >= 10 {
            ActionResolverResult::Action(ActionType::Heal)
        } else {
            ActionResolverResult::Break
        }
    }
}





// Simplified rule system - all tokens are ActionResolvers
pub type RuleToken = Box<dyn ActionResolver>;

pub struct ActionCalculationSystem {
    pub rules: Vec<RuleToken>,
    pub rng: StdRng,
}

impl ActionCalculationSystem {
    pub fn new(rules: Vec<RuleToken>, rng: StdRng) -> Self {
        Self {
            rules,
            rng,
        }
    }

    pub fn calculate_action(&mut self, character: &crate::battle_system::Character) -> Option<ActionType> {
        let rng = &mut self.rng;

        for rule in &self.rules {
            match rule.resolve(character, rng) {
                ActionResolverResult::Action(action_type) => {
                    return Some(action_type);
                }
                ActionResolverResult::Break => {
                    continue; // Try next rule
                }
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
        let strike = StrikeAction;
        let mut rng = StdRng::from_entropy();
        
        match strike.resolve(&character, &mut rng) {
            ActionResolverResult::Action(ActionType::Strike) => assert!(true),
            _ => panic!("StrikeAction should return Action(Strike) for alive character"),
        }
        
        let dead_character = Character::new("Dead".to_string(), 0, 0, 25);
        match strike.resolve(&dead_character, &mut rng) {
            ActionResolverResult::Break => assert!(true),
            _ => panic!("StrikeAction should return Break for dead character"),
        }
    }

    #[test]
    fn test_heal_token() {
        let character = Character::new("Test".to_string(), 100, 50, 25);
        let heal = HealAction;
        let mut rng = StdRng::from_entropy();
        
        match heal.resolve(&character, &mut rng) {
            ActionResolverResult::Action(ActionType::Heal) => assert!(true),
            _ => panic!("HealAction should return Action(Heal) for alive character"),
        }
        
        let dead_character = Character::new("Dead".to_string(), 0, 0, 25);
        match heal.resolve(&dead_character, &mut rng) {
            ActionResolverResult::Break => assert!(true),
            _ => panic!("HealAction should return Break for dead character"),
        }
    }

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
    fn test_check_token() {
        let character = Character::new("Test".to_string(), 100, 50, 25);
        let check_random = CheckToken::new(
            Box::new(TrueOrFalseRandomToken),
            Box::new(StrikeAction),
        );
        let mut rng = StdRng::from_entropy();
        
        match check_random.resolve(&character, &mut rng) {
            ActionResolverResult::Action(_) | ActionResolverResult::Break => assert!(true),
        }
    }

    #[test]
    fn test_action_calculation_system() {
        let rules: Vec<RuleToken> = vec![
            Box::new(CheckToken::new(
                Box::new(TrueOrFalseRandomToken),
                Box::new(HealAction),
            )),
            Box::new(StrikeAction),
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
    fn test_action_system_with_seed() {
        let character = Character::new("Test".to_string(), 100, 50, 25);
        let mut damaged_character = character.clone();
        damaged_character.take_damage(50); // HP: 50/100
        
        let create_rules = || -> Vec<RuleToken> {
            vec![
                Box::new(CheckToken::new(
                    Box::new(TrueOrFalseRandomToken),
                    Box::new(HealAction),
                )),
                Box::new(StrikeAction),
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
        
        // Test Constant token
        let number_token = ConstantToken::new(42);
        assert_eq!(number_token.evaluate(&character, &mut rng), 42);
        
        // Test CharacterHP token
        let char_hp_token = CharacterHPToken;
        assert_eq!(char_hp_token.evaluate(&character, &mut rng), 100);
        
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

    #[test]
    fn test_hp_based_action_logic() {
        let mut low_hp_character = Character::new("LowHP".to_string(), 100, 50, 25);
        low_hp_character.take_damage(70); // HP: 30
        
        let high_hp_character = Character::new("HighHP".to_string(), 100, 50, 25);
        // HP: 100
        
        // Create HP-based rules
        let hp_rules: Vec<RuleToken> = vec![
            Box::new(CheckToken::new(
                Box::new(GreaterThanToken::new(
                    Box::new(ConstantToken::new(50)),
                    Box::new(CharacterHPToken),
                )),
                Box::new(HealAction),
            )),
            Box::new(StrikeAction),
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

