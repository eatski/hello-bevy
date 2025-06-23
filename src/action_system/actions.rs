// Action tokens - tokens that resolve to specific actions

use super::core::{ActionResolver, ActionResolverResult, ActionType};
use super::bool_tokens::BoolToken;

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::battle_system::Character;
    use crate::action_system::{TrueOrFalseRandomToken};
    use rand::rngs::StdRng;
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
}