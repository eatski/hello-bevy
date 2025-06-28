// Heal action node - resolves to heal action

use super::core::{ActionResolver, ActionResolverResult, ActionType};

#[derive(Debug)]
pub struct HealActionNode;

impl ActionResolver for HealActionNode {
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
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_heal_action_node() {
        let character = Character::new("Test".to_string(), 100, 50, 25);
        let heal = HealActionNode;
        let mut rng = StdRng::from_entropy();
        
        match heal.resolve(&character, &mut rng) {
            ActionResolverResult::Action(ActionType::Heal) => assert!(true),
            _ => panic!("HealActionNode should return Action(Heal) for alive character"),
        }
        
        let dead_character = Character::new("Dead".to_string(), 0, 0, 25);
        match heal.resolve(&dead_character, &mut rng) {
            ActionResolverResult::Break => assert!(true),
            _ => panic!("HealActionNode should return Break for dead character"),
        }
    }
}