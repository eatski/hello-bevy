// Heal action node - resolves to heal action with target character

use crate::core::{ActionResolver, NodeResult, NodeError, Action, HealAction};
use crate::nodes::character::CharacterNode;

#[derive(Debug)]
pub struct HealActionNode {
    target: Box<dyn CharacterNode>,
}

impl HealActionNode {
    pub fn new(target: Box<dyn CharacterNode>) -> Self {
        Self { target }
    }
}

impl ActionResolver for HealActionNode {
    fn resolve(&self, battle_context: &crate::BattleContext, rng: &mut dyn rand::RngCore) -> NodeResult<Box<dyn Action>> {
        let acting_character = battle_context.get_acting_character();
        
        // Check if acting character can perform heal (alive and has MP)
        if acting_character.hp <= 0 || acting_character.mp < 10 {
            return Err(NodeError::Break);
        }
        
        // Evaluate target character ID
        let target_id = self.target.evaluate(battle_context, rng)?;
        
        // Create and return HealAction with the evaluated target ID
        Ok(Box::new(HealAction::new(target_id)))
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
        use crate::nodes::character::ActingCharacterNode;
        
        let player = Character::new(1, "Player".to_string(), 100, 50, 25);
        let enemy = Character::new(2, "Enemy".to_string(), 80, 30, 20);
        
        let acting_character = Character::new(3, "Test".to_string(), 100, 50, 25);
        let battle_context = crate::BattleContext::new(&acting_character, &player, &enemy);
        
        // Create heal action with acting character as target
        let target = Box::new(ActingCharacterNode);
        let heal = HealActionNode::new(target);
        let mut rng = StdRng::from_entropy();
        
        let result = heal.resolve(&battle_context, &mut rng);
        assert!(result.is_ok(), "HealActionNode should return HealAction for alive character");
        if let Ok(action) = result {
            assert_eq!(action.get_action_name(), "Heal");
        }
        
        let dead_character = Character::new(4, "Dead".to_string(), 0, 0, 25);
        let dead_battle_context = crate::BattleContext::new(&dead_character, &player, &enemy);
        let target_dead = Box::new(ActingCharacterNode);
        let heal_dead = HealActionNode::new(target_dead);
        let result = heal_dead.resolve(&dead_battle_context, &mut rng);
        assert!(matches!(result, Err(NodeError::Break)), "HealActionNode should return Break error for dead character");
    }
}