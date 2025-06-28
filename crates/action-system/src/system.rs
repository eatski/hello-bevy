// Action calculation system - manages rule execution

use rand::rngs::StdRng;
use super::core::{ActionType, ActionResolverResult, RuleNode};
use crate::BattleContext;

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

    pub fn calculate_action(&mut self, battle_context: &BattleContext) -> Option<ActionType> {
        let rng = &mut self.rng;

        for rule in &self.rules {
            match rule.resolve(battle_context, rng) {
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
        let player = Character::new("Player".to_string(), 100, 50, 25);
        let enemy = Character::new("Enemy".to_string(), 80, 30, 20);
        let acting_character = Character::new("Test".to_string(), 100, 50, 25);
        let battle_context = BattleContext::new(&acting_character, &player, &enemy);
        
        let action = system.calculate_action(&battle_context);
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
        
        let player = Character::new("Player".to_string(), 100, 50, 25);
        let enemy = Character::new("Enemy".to_string(), 80, 30, 20);
        
        // Test with multiple attempts to verify both Strike and Heal can occur
        let mut strike_count = 0;
        let mut heal_count = 0;
        
        // Test 20 attempts to get both actions
        for _ in 0..20 {
            let battle_context = BattleContext::new(&damaged_character, &player, &enemy);
            if let Some(action) = system1.calculate_action(&battle_context) {
                match action {
                    ActionType::Strike => strike_count += 1,
                    ActionType::Heal => heal_count += 1,
                }
            }
            let battle_context = BattleContext::new(&damaged_character, &player, &enemy);
            if let Some(action) = system2.calculate_action(&battle_context) {
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
        
        let player = Character::new("Player".to_string(), 100, 50, 25);
        let enemy = Character::new("Enemy".to_string(), 80, 30, 20);
        
        // Low HP character should heal
        let low_hp_battle_context = BattleContext::new(&low_hp_character, &player, &enemy);
        let low_hp_action = system.calculate_action(&low_hp_battle_context);
        assert_eq!(low_hp_action, Some(ActionType::Heal), "Low HP character should choose Heal");
        
        // High HP character should strike
        let high_hp_battle_context = BattleContext::new(&high_hp_character, &player, &enemy);
        let high_hp_action = system.calculate_action(&high_hp_battle_context);
        assert_eq!(high_hp_action, Some(ActionType::Strike), "High HP character should choose Strike");
    }

    #[test]
    fn test_multiple_seeds_produce_different_results() {
        // 複数のseedで異なる結果が出ることを検証
        let seeds = [12345u64, 67890u64, 11111u64, 99999u64];
        let character = Character::new("Test".to_string(), 100, 50, 25);
        let player = Character::new("Player".to_string(), 100, 50, 25);
        let enemy = Character::new("Enemy".to_string(), 80, 30, 20);

        let create_random_rules = || -> Vec<RuleNode> {
            vec![
                Box::new(ConditionCheckNode::new(
                    Box::new(RandomConditionNode),
                    Box::new(HealActionNode),
                )),
                Box::new(StrikeActionNode),
            ]
        };

        let mut all_results = Vec::new();

        // 各seedで10回ずつ実行して結果を収集
        for &seed in &seeds {
            let rng = StdRng::seed_from_u64(seed);
            let mut system = ActionCalculationSystem::new(create_random_rules(), rng);
            
            let mut seed_results = Vec::new();
            for _ in 0..10 {
                let battle_context = BattleContext::new(&character, &player, &enemy);
                if let Some(action) = system.calculate_action(&battle_context) {
                    seed_results.push(action);
                }
            }
            all_results.push(seed_results);
        }

        // 少なくとも2つのseedで異なる結果が出ることを確認
        let mut found_difference = false;
        for i in 0..all_results.len() {
            for j in i+1..all_results.len() {
                if all_results[i] != all_results[j] {
                    found_difference = true;
                    break;
                }
            }
            if found_difference {
                break;
            }
        }

        assert!(found_difference, "Different seeds should produce different results");

        // 各seedで最低1つのアクションが返されることを確認
        for (i, results) in all_results.iter().enumerate() {
            assert!(!results.is_empty(), "Seed {} should produce at least one action", seeds[i]);
        }
    }

    #[test]
    fn test_same_seed_multiple_executions_can_differ() {
        // 同一seedで複数回実行し、状態変化により結果が異なることを検証
        let seed = 42u64;
        let character = Character::new("Test".to_string(), 100, 50, 25);
        let player = Character::new("Player".to_string(), 100, 50, 25);
        let enemy = Character::new("Enemy".to_string(), 80, 30, 20);

        let create_random_rules = || -> Vec<RuleNode> {
            vec![
                Box::new(ConditionCheckNode::new(
                    Box::new(RandomConditionNode),
                    Box::new(HealActionNode),
                )),
                Box::new(StrikeActionNode),
            ]
        };

        let rng = StdRng::seed_from_u64(seed);
        let mut system = ActionCalculationSystem::new(create_random_rules(), rng);

        let mut results = Vec::new();
        
        // 同一システムで20回実行（RNGの状態が変化する）
        for _ in 0..20 {
            let battle_context = BattleContext::new(&character, &player, &enemy);
            if let Some(action) = system.calculate_action(&battle_context) {
                results.push(action);
            }
        }

        // 結果が全て同じではないことを確認
        if results.len() > 1 {
            let first_action = &results[0];
            let has_different_action = results.iter().any(|action| action != first_action);
            assert!(has_different_action, "Multiple executions with same seed should produce different results due to RNG state changes");
        }

        // 少なくとも1つのアクションが返されることを確認
        assert!(!results.is_empty(), "Should produce at least one action");
    }

}