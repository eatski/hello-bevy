// Heal action node - resolves to heal action

use crate::core::{ActionResolver, ActionType, NodeResult, NodeError};

#[derive(Debug)]
pub struct HealActionNode;

impl ActionResolver for HealActionNode {
    fn resolve(&self, battle_context: &crate::BattleContext, _rng: &mut dyn rand::RngCore) -> NodeResult<ActionType> {
        let acting_character = battle_context.get_acting_character();
        if acting_character.hp > 0 && acting_character.mp >= 10 {
            Ok(ActionType::Heal)
        } else {
            Err(NodeError::Break)
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
        
        let result = heal.resolve(&battle_context, &mut rng);
        assert_eq!(result, Ok(ActionType::Heal), "HealActionNode should return Heal for alive character");
        
        let dead_character = Character::new("Dead".to_string(), 0, 0, 25);
        let dead_battle_context = crate::BattleContext::new(&dead_character, &player, &enemy);
        let result = heal.resolve(&dead_battle_context, &mut rng);
        assert_eq!(result, Err(NodeError::Break), "HealActionNode should return Break error for dead character");
    }
}