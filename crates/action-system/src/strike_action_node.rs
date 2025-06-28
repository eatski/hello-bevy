// Strike action node - resolves to strike action

use super::core::{ActionResolver, ActionResolverResult, ActionType};

#[derive(Debug)]
pub struct StrikeActionNode;

impl ActionResolver for StrikeActionNode {
    fn resolve(&self, character: &crate::Character, _rng: &mut dyn rand::RngCore) -> ActionResolverResult {
        if character.hp > 0 {
            ActionResolverResult::Action(ActionType::Strike)
        } else {
            ActionResolverResult::Break
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Character;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_strike_action_node() {
        let character = Character::new("Test".to_string(), 100, 50, 25);
        let strike = StrikeActionNode;
        let mut rng = StdRng::from_entropy();
        
        match strike.resolve(&character, &mut rng) {
            ActionResolverResult::Action(ActionType::Strike) => assert!(true),
            _ => panic!("StrikeActionNode should return Action(Strike) for alive character"),
        }
        
        let dead_character = Character::new("Dead".to_string(), 0, 0, 25);
        match strike.resolve(&dead_character, &mut rng) {
            ActionResolverResult::Break => assert!(true),
            _ => panic!("StrikeActionNode should return Break for dead character"),
        }
    }
}