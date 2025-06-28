// Heal action node - resolves to heal action

use super::core::{ActionResolver, ActionResolverResult, ActionType};

#[derive(Debug)]
pub struct HealActionNode;

impl ActionResolver for HealActionNode {
    fn resolve(&self, battle_context: &crate::BattleContext, _rng: &mut dyn rand::RngCore) -> ActionResolverResult {
        let acting_character = battle_context.get_acting_character();
        if acting_character.hp > 0 && acting_character.mp >= 10 {
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
        let player = Character::new("Player".to_string(), 100, 50, 25);
        let enemy = Character::new("Enemy".to_string(), 80, 30, 20);
        
        let acting_character = Character::new("Test".to_string(), 100, 50, 25);
        let battle_context = crate::BattleContext::new(&acting_character, &player, &enemy);
        let heal = HealActionNode;
        let mut rng = StdRng::from_entropy();
        
        match heal.resolve(&battle_context, &mut rng) {
            ActionResolverResult::Action(ActionType::Heal) => assert!(true),
            _ => panic!("HealActionNode should return Action(Heal) for alive character"),
        }
        
        let dead_character = Character::new("Dead".to_string(), 0, 0, 25);
        let dead_battle_context = crate::BattleContext::new(&dead_character, &player, &enemy);
        match heal.resolve(&dead_battle_context, &mut rng) {
            ActionResolverResult::Break => assert!(true),
            _ => panic!("HealActionNode should return Break for dead character"),
        }
    }
}