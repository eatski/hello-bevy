// Strike action node - resolves to strike action

use super::core::{ActionResolver, ActionResolverResult, ActionType};

#[derive(Debug)]
pub struct StrikeActionNode;

impl ActionResolver for StrikeActionNode {
    fn resolve(&self, battle_context: &crate::BattleContext, _rng: &mut dyn rand::RngCore) -> ActionResolverResult {
        if battle_context.get_acting_character().hp > 0 {
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
        let player = Character::new("Player".to_string(), 100, 50, 25);
        let enemy = Character::new("Enemy".to_string(), 80, 30, 20);
        
        let acting_character = Character::new("Test".to_string(), 100, 50, 25);
        let battle_context = crate::BattleContext::new(&acting_character, &player, &enemy);
        let strike = StrikeActionNode;
        let mut rng = StdRng::from_entropy();
        
        let result = strike.resolve(&battle_context, &mut rng);
        assert_eq!(result, ActionResolverResult::Action(ActionType::Strike), "StrikeActionNode should return Action(Strike) for alive character");
        
        let dead_character = Character::new("Dead".to_string(), 0, 0, 25);
        let dead_battle_context = crate::BattleContext::new(&dead_character, &player, &enemy);
        let result = strike.resolve(&dead_battle_context, &mut rng);
        assert_eq!(result, ActionResolverResult::Break, "StrikeActionNode should return Break for dead character");
    }
}