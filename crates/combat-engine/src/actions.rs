// Action nodes - nodes that resolve to specific actions

use super::core::{ActionResolver, ActionResolverResult, ActionType};
use super::bool_nodes::BoolNode;

// Check node type - evaluates condition and delegates to next node or breaks
#[derive(Debug)]
pub struct CheckNode {
    condition: Box<dyn BoolNode>,
    next: Box<dyn ActionResolver>,
}

impl CheckNode {
    pub fn new(condition: Box<dyn BoolNode>, next: Box<dyn ActionResolver>) -> Self {
        Self { condition, next }
    }
}

impl ActionResolver for CheckNode {
    fn resolve(&self, character: &crate::Character, rng: &mut dyn rand::RngCore) -> ActionResolverResult {
        if self.condition.evaluate(character, rng) {
            // Continue: delegate to next node
            self.next.resolve(character, rng)
        } else {
            ActionResolverResult::Break
        }
    }
}

// Action node types
#[derive(Debug)]
pub struct StrikeAction;

impl ActionResolver for StrikeAction {
    fn resolve(&self, character: &crate::Character, _rng: &mut dyn rand::RngCore) -> ActionResolverResult {
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
    fn resolve(&self, character: &crate::Character, _rng: &mut dyn rand::RngCore) -> ActionResolverResult {
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
    use crate::Character;
    use crate::{TrueOrFalseRandomNode};
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_strike_action() {
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
    fn test_heal_action() {
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
    fn test_check_node() {
        let character = Character::new("Test".to_string(), 100, 50, 25);
        let check_random = CheckNode::new(
            Box::new(TrueOrFalseRandomNode),
            Box::new(StrikeAction),
        );
        let mut rng = StdRng::from_entropy();
        
        match check_random.resolve(&character, &mut rng) {
            ActionResolverResult::Action(_) | ActionResolverResult::Break => assert!(true),
        }
    }
}