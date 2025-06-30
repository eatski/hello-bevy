// Heal action node - resolves to heal action with target character

use crate::core::{ActionResolver, ActionType, NodeResult, NodeError};
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
    fn resolve(&self, battle_context: &crate::BattleContext, rng: &mut dyn rand::RngCore) -> NodeResult<ActionType> {
        let acting_character = battle_context.get_acting_character();
        
        // Check if acting character can perform heal (alive and has MP)
        if acting_character.hp <= 0 || acting_character.mp < 10 {
            return Err(NodeError::Break);
        }
        
        // Evaluate target character
        let _target_character = self.target.evaluate(battle_context, rng)?;
        
        // If we can successfully evaluate the target, return Heal action
        Ok(ActionType::Heal)
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
        
        let player = Character::new("Player".to_string(), 100, 50, 25);
        let enemy = Character::new("Enemy".to_string(), 80, 30, 20);
        
        let acting_character = Character::new("Test".to_string(), 100, 50, 25);
        let battle_context = crate::BattleContext::new(&acting_character, &player, &enemy);
        
        // Create heal action with acting character as target
        let target = Box::new(ActingCharacterNode);
        let heal = HealActionNode::new(target);
        let mut rng = StdRng::from_entropy();
        
        let result = heal.resolve(&battle_context, &mut rng);
        assert_eq!(result, Ok(ActionType::Heal), "HealActionNode should return Heal for alive character");
        
        let dead_character = Character::new("Dead".to_string(), 0, 0, 25);
        let dead_battle_context = crate::BattleContext::new(&dead_character, &player, &enemy);
        let target_dead = Box::new(ActingCharacterNode);
        let heal_dead = HealActionNode::new(target_dead);
        let result = heal_dead.resolve(&dead_battle_context, &mut rng);
        assert_eq!(result, Err(NodeError::Break), "HealActionNode should return Break error for dead character");
    }
}