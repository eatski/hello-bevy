// Action calculation system - manages rule execution

use rand::rngs::StdRng;
use super::core::{ActionType, ActionResolverResult, RuleNode};

pub struct ActionCalculationSystem {
    pub rules: Vec<RuleNode>,
    pub rng: StdRng,
}

impl ActionCalculationSystem {
    pub fn new(rules: Vec<RuleNode>, rng: StdRng) -> Self {
        Self {
            rules,
            rng,
        }
    }

    pub fn calculate_action(&mut self, character: &crate::Character) -> Option<ActionType> {
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
    use crate::Character;
    use crate::{ConditionCheckNode, StrikeActionNode, HealActionNode, RandomConditionNode, GreaterThanConditionNode, ConstantValueNode, ActingCharacterNode, CharacterHpFromNode};
    use rand::SeedableRng;

    #[test]
    fn test_action_calculation_system() {
        let rules: Vec<RuleNode> = vec![
            Box::new(ConditionCheckNode::new(
                Box::new(RandomConditionNode),
                Box::new(HealActionNode),
            )),
            Box::new(StrikeActionNode),
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
    fn test_action_system_with_seed() {
        let character = Character::new("Test".to_string(), 100, 50, 25);
        let mut damaged_character = character.clone();
        damaged_character.take_damage(50); // HP: 50/100
        
        let create_rules = || -> Vec<RuleNode> {
            vec![
                Box::new(ConditionCheckNode::new(
                    Box::new(RandomConditionNode),
                    Box::new(HealActionNode),
                )),
                Box::new(StrikeActionNode),
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
    fn test_hp_based_action_logic() {
        let mut low_hp_character = Character::new("LowHP".to_string(), 100, 50, 25);
        low_hp_character.take_damage(70); // HP: 30
        
        let high_hp_character = Character::new("HighHP".to_string(), 100, 50, 25);
        // HP: 100
        
        // Create HP-based rules
        let hp_rules: Vec<RuleNode> = vec![
            Box::new(ConditionCheckNode::new(
                Box::new(GreaterThanConditionNode::new(
                    Box::new(ConstantValueNode::new(50)),
                    Box::new(CharacterHpFromNode::new(Box::new(ActingCharacterNode))),
                )),
                Box::new(HealActionNode),
            )),
            Box::new(StrikeActionNode),
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