// Action calculation system - manages rule execution

use rand::rngs::StdRng;
use super::core::{Action, RuleNode, NodeError};
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

    pub fn calculate_action(&mut self, battle_context: &BattleContext) -> Option<Box<dyn Action>> {
        let rng = &mut self.rng;

        for rule in &self.rules {
            match rule.resolve(battle_context, rng) {
                Ok(action) => {
                    return Some(action);
                }
                Err(NodeError::Break) => {
                    continue; // Try next rule
                }
                Err(_error) => {
                    // エラーが発生した場合は次のルールを試す
                    // 必要に応じてログ出力やエラーハンドリングを追加可能
                    continue;
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
                Box::new(HealActionNode::new(Box::new(ActingCharacterNode))),
            )),
            Box::new(StrikeActionNode::new(Box::new(ActingCharacterNode))),
        ];
        let rng = StdRng::from_entropy();
        let mut system = ActionCalculationSystem::new(rules, rng);
        let player = Character::new(1, "Player".to_string(), 100, 50, 25);
        let enemy = Character::new(2, "Enemy".to_string(), 80, 30, 20);
        let acting_character = Character::new(3, "Test".to_string(), 100, 50, 25);
        let battle_context = BattleContext::new(&acting_character, &player, &enemy);
        
        let action = system.calculate_action(&battle_context);
        assert!(action.is_some(), "Should return some action");
        
        let action_name = action.unwrap().get_action_name();
        assert!(action_name == "Strike" || action_name == "Heal");
    }

    #[test]
    fn test_action_system_with_seed() {
        let character = Character::new(4, "Test".to_string(), 100, 50, 25);
        let mut damaged_character = character.clone();
        damaged_character.take_damage(50); // HP: 50/100
        
        let create_rules = || -> Vec<RuleNode> {
            vec![
                Box::new(ConditionCheckNode::new(
                    Box::new(RandomConditionNode),
                    Box::new(HealActionNode::new(Box::new(ActingCharacterNode))),
                )),
                Box::new(StrikeActionNode::new(Box::new(ActingCharacterNode))),
            ]
        };
        
        // Test that the system can produce different actions
        let rng1 = StdRng::from_entropy();
        let rng2 = StdRng::from_entropy();
        let mut system1 = ActionCalculationSystem::new(create_rules(), rng1);
        let mut system2 = ActionCalculationSystem::new(create_rules(), rng2);
        
        let player = Character::new(5, "Player".to_string(), 100, 50, 25);
        let enemy = Character::new(6, "Enemy".to_string(), 80, 30, 20);
        
        // Test with multiple attempts to verify both Strike and Heal can occur
        let mut strike_count = 0;
        let mut heal_count = 0;
        
        // Test 20 attempts to get both actions
        for _ in 0..20 {
            let battle_context = BattleContext::new(&damaged_character, &player, &enemy);
            if let Some(action) = system1.calculate_action(&battle_context) {
                match action.get_action_name() {
                    "Strike" => strike_count += 1,
                    "Heal" => heal_count += 1,
                    _ => {},
                }
            }
            let battle_context = BattleContext::new(&damaged_character, &player, &enemy);
            if let Some(action) = system2.calculate_action(&battle_context) {
                match action.get_action_name() {
                    "Strike" => strike_count += 1,
                    "Heal" => heal_count += 1,
                    _ => {},
                }
            }
        }
        
        assert!(strike_count >= 1, "Should have at least one Strike action across attempts, got {}", strike_count);
        assert!(heal_count >= 1, "Should have at least one Heal action across attempts, got {}", heal_count);
        assert_eq!(strike_count + heal_count, 40, "Should have 40 total actions from 20 attempts with 2 systems");
    }

    #[test]
    fn test_hp_based_action_logic() {
        let mut low_hp_character = Character::new(7, "LowHP".to_string(), 100, 50, 25);
        low_hp_character.take_damage(70); // HP: 30
        
        let high_hp_character = Character::new(8, "HighHP".to_string(), 100, 50, 25);
        // HP: 100
        
        // Create HP-based rules
        let hp_rules: Vec<RuleNode> = vec![
            Box::new(ConditionCheckNode::new(
                Box::new(GreaterThanConditionNode::new(
                    Box::new(ConstantValueNode::new(50)),
                    Box::new(CharacterHpFromNode::new(Box::new(ActingCharacterNode))),
                )),
                Box::new(HealActionNode::new(Box::new(ActingCharacterNode))),
            )),
            Box::new(StrikeActionNode::new(Box::new(ActingCharacterNode))),
        ];
        
        let rng = StdRng::from_entropy();
        let mut system = ActionCalculationSystem::new(hp_rules, rng);
        
        let player = Character::new(9, "Player".to_string(), 100, 50, 25);
        let enemy = Character::new(10, "Enemy".to_string(), 80, 30, 20);
        
        // Low HP character should heal
        let low_hp_battle_context = BattleContext::new(&low_hp_character, &player, &enemy);
        let low_hp_action = system.calculate_action(&low_hp_battle_context);
        assert_eq!(low_hp_action.as_ref().map(|a| a.get_action_name()), Some("Heal"), "Low HP character should choose Heal");
        
        // High HP character should strike
        let high_hp_battle_context = BattleContext::new(&high_hp_character, &player, &enemy);
        let high_hp_action = system.calculate_action(&high_hp_battle_context);
        assert_eq!(high_hp_action.as_ref().map(|a| a.get_action_name()), Some("Strike"), "High HP character should choose Strike");
    }

    #[test]
    fn test_multiple_seeds_produce_different_results() {
        // 複数のseedで異なる結果が出ることを検証
        let seeds = [12345u64, 67890u64, 11111u64, 99999u64];
        let character = Character::new(11, "Test".to_string(), 100, 50, 25);
        let player = Character::new(12, "Player".to_string(), 100, 50, 25);
        let enemy = Character::new(13, "Enemy".to_string(), 80, 30, 20);

        let create_random_rules = || -> Vec<RuleNode> {
            vec![
                Box::new(ConditionCheckNode::new(
                    Box::new(RandomConditionNode),
                    Box::new(HealActionNode::new(Box::new(ActingCharacterNode))),
                )),
                Box::new(StrikeActionNode::new(Box::new(ActingCharacterNode))),
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
                // Compare action names instead of Action objects
                let names_i: Vec<&str> = all_results[i].iter().map(|a| a.get_action_name()).collect();
                let names_j: Vec<&str> = all_results[j].iter().map(|a| a.get_action_name()).collect();
                if names_i != names_j {
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
        let character = Character::new(14, "Test".to_string(), 100, 50, 25);
        let player = Character::new(15, "Player".to_string(), 100, 50, 25);
        let enemy = Character::new(16, "Enemy".to_string(), 80, 30, 20);

        let create_random_rules = || -> Vec<RuleNode> {
            vec![
                Box::new(ConditionCheckNode::new(
                    Box::new(RandomConditionNode),
                    Box::new(HealActionNode::new(Box::new(ActingCharacterNode))),
                )),
                Box::new(StrikeActionNode::new(Box::new(ActingCharacterNode))),
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
            let first_action_name = results[0].get_action_name();
            let has_different_action = results.iter().any(|action| action.get_action_name() != first_action_name);
            assert!(has_different_action, "Multiple executions with same seed should produce different results due to RNG state changes");
        }

        // 少なくとも1つのアクションが返されることを確認
        assert!(!results.is_empty(), "Should produce at least one action");
    }

    #[test]
    fn test_complex_condition_combinations() {
        // 複数の条件ノードを組み合わせたテスト
        let character = Character::new(17, "Test".to_string(), 100, 50, 25);
        let mut low_hp_character = character.clone();
        low_hp_character.take_damage(80); // HP: 20/100
        
        let player = Character::new(18, "Player".to_string(), 100, 50, 25);
        let enemy = Character::new(19, "Enemy".to_string(), 80, 30, 20);

        // 複雑な条件: HP < 30 AND ランダム条件が真の場合のみHeal
        let complex_rules: Vec<RuleNode> = vec![
            Box::new(ConditionCheckNode::new(
                Box::new(GreaterThanConditionNode::new(
                    Box::new(ConstantValueNode::new(30)),
                    Box::new(CharacterHpFromNode::new(Box::new(ActingCharacterNode))),
                )),
                Box::new(ConditionCheckNode::new(
                    Box::new(RandomConditionNode),
                    Box::new(HealActionNode::new(Box::new(ActingCharacterNode))),
                )),
            )),
            Box::new(StrikeActionNode::new(Box::new(ActingCharacterNode))),
        ];

        let rng = StdRng::from_entropy();
        let mut system = ActionCalculationSystem::new(complex_rules, rng);

        // 低HPキャラクターは条件を満たすのでHealまたはStrike
        let mut heal_count = 0;
        let mut strike_count = 0;
        
        for _ in 0..20 {
            let battle_context = BattleContext::new(&low_hp_character, &player, &enemy);
            if let Some(action) = system.calculate_action(&battle_context) {
                match action.get_action_name() {
                    "Heal" => heal_count += 1,
                    "Strike" => strike_count += 1,
                    _ => {},
                }
            }
        }

        assert!(heal_count + strike_count == 20, "Should execute 20 actions");
        assert!(heal_count > 0 || strike_count > 0, "Should have at least one action type");
    }

    #[test]
    fn test_nested_condition_fallback() {
        // ネストした条件で最終的にフォールバックアクションが実行されることをテスト
        let high_hp_character = Character::new(20, "HighHP".to_string(), 100, 50, 25);
        let player = Character::new(21, "Player".to_string(), 100, 50, 25);
        let enemy = Character::new(22, "Enemy".to_string(), 80, 30, 20);

        // HP > 90の場合のみランダム条件チェック、そうでなければStrike
        let nested_rules: Vec<RuleNode> = vec![
            Box::new(ConditionCheckNode::new(
                Box::new(GreaterThanConditionNode::new(
                    Box::new(CharacterHpFromNode::new(Box::new(ActingCharacterNode))),
                    Box::new(ConstantValueNode::new(90)),
                )),
                Box::new(ConditionCheckNode::new(
                    Box::new(RandomConditionNode),
                    Box::new(HealActionNode::new(Box::new(ActingCharacterNode))),
                )),
            )),
            Box::new(StrikeActionNode::new(Box::new(ActingCharacterNode))),
        ];

        let rng = StdRng::from_entropy();
        let mut system = ActionCalculationSystem::new(nested_rules, rng);

        let mut results = Vec::new();
        for _ in 0..10 {
            let battle_context = BattleContext::new(&high_hp_character, &player, &enemy);
            if let Some(action) = system.calculate_action(&battle_context) {
                results.push(action);
            }
        }

        assert_eq!(results.len(), 10, "Should execute 10 actions");
        // HP=100 > 90なので、最初の条件は満たし、ランダム条件でHealかフォールバックでStrike
        assert!(results.iter().all(|action| action.get_action_name() == "Heal" || action.get_action_name() == "Strike"));
    }

    #[test]
    fn test_multiple_hp_thresholds() {
        // 複数のHP閾値を使った段階的な行動選択テスト
        let player = Character::new(23, "Player".to_string(), 100, 50, 25);
        let enemy = Character::new(24, "Enemy".to_string(), 80, 30, 20);

        // HP > 70: Strike, HP > 30: Heal, その他: Strike
        let threshold_rules: Vec<RuleNode> = vec![
            Box::new(ConditionCheckNode::new(
                Box::new(GreaterThanConditionNode::new(
                    Box::new(CharacterHpFromNode::new(Box::new(ActingCharacterNode))),
                    Box::new(ConstantValueNode::new(70)),
                )),
                Box::new(StrikeActionNode::new(Box::new(ActingCharacterNode))),
            )),
            Box::new(ConditionCheckNode::new(
                Box::new(GreaterThanConditionNode::new(
                    Box::new(CharacterHpFromNode::new(Box::new(ActingCharacterNode))),
                    Box::new(ConstantValueNode::new(30)),
                )),
                Box::new(HealActionNode::new(Box::new(ActingCharacterNode))),
            )),
            Box::new(StrikeActionNode::new(Box::new(ActingCharacterNode))),
        ];

        let rng = StdRng::from_entropy();
        let mut system = ActionCalculationSystem::new(threshold_rules, rng);

        // テスト1: HP=100 (>70) -> Strike
        let high_hp_char = Character::new(25, "High".to_string(), 100, 50, 25);
        let battle_context = BattleContext::new(&high_hp_char, &player, &enemy);
        let action = system.calculate_action(&battle_context);
        assert_eq!(action.as_ref().map(|a| a.get_action_name()), Some("Strike"), "High HP character should Strike");

        // テスト2: HP=50 (30<HP<70) -> Heal
        let mut mid_hp_char = Character::new(26, "Mid".to_string(), 100, 50, 25);
        mid_hp_char.take_damage(50); // HP: 50
        let battle_context = BattleContext::new(&mid_hp_char, &player, &enemy);
        let action = system.calculate_action(&battle_context);
        assert_eq!(action.as_ref().map(|a| a.get_action_name()), Some("Heal"), "Mid HP character should Heal");

        // テスト3: HP=20 (<30) -> Strike
        let mut low_hp_char = Character::new(27, "Low".to_string(), 100, 50, 25);
        low_hp_char.take_damage(80); // HP: 20
        let battle_context = BattleContext::new(&low_hp_char, &player, &enemy);
        let action = system.calculate_action(&battle_context);
        assert_eq!(action.as_ref().map(|a| a.get_action_name()), Some("Strike"), "Low HP character should Strike (fallback)");
    }

    #[test]
    fn test_character_selection_combinations() {
        // 異なるキャラクター選択ノードの組み合わせテスト
        use crate::RandomCharacterNode;
        
        let acting_char = Character::new(28, "Actor".to_string(), 100, 50, 25);
        let player = Character::new(29, "Player".to_string(), 80, 40, 20);
        let enemy = Character::new(30, "Enemy".to_string(), 60, 30, 15);

        // ActingCharacterのHPと他キャラクターのHPを比較
        let char_comparison_rules: Vec<RuleNode> = vec![
            Box::new(ConditionCheckNode::new(
                Box::new(GreaterThanConditionNode::new(
                    Box::new(CharacterHpFromNode::new(Box::new(ActingCharacterNode))),
                    Box::new(CharacterHpFromNode::new(Box::new(RandomCharacterNode))),
                )),
                Box::new(StrikeActionNode::new(Box::new(ActingCharacterNode))),
            )),
            Box::new(HealActionNode::new(Box::new(ActingCharacterNode))),
        ];

        let rng = StdRng::seed_from_u64(12345);
        let mut system = ActionCalculationSystem::new(char_comparison_rules, rng);

        let mut results = Vec::new();
        for _ in 0..10 {
            let battle_context = BattleContext::new(&acting_char, &player, &enemy);
            if let Some(action) = system.calculate_action(&battle_context) {
                results.push(action);
            }
        }

        assert_eq!(results.len(), 10, "Should execute 10 actions");
        // ActingChar(HP=100)は他のキャラより高いHPを持つため、主にStrikeが選ばれるはず
        let strike_count = results.iter().filter(|&action| action.get_action_name() == "Strike").count();
        assert!(strike_count > 0, "Should have at least some Strike actions");
    }

    #[test]
    fn test_value_node_combinations() {
        // 異なる値ノードの組み合わせテスト
        let character = Character::new(31, "Test".to_string(), 100, 50, 25);
        let player = Character::new(32, "Player".to_string(), 100, 50, 25);
        let enemy = Character::new(33, "Enemy".to_string(), 80, 30, 20);

        // 定数値同士の比較
        let constant_comparison_rules: Vec<RuleNode> = vec![
            Box::new(ConditionCheckNode::new(
                Box::new(GreaterThanConditionNode::new(
                    Box::new(ConstantValueNode::new(100)),
                    Box::new(ConstantValueNode::new(50)),
                )),
                Box::new(HealActionNode::new(Box::new(ActingCharacterNode))),
            )),
            Box::new(StrikeActionNode::new(Box::new(ActingCharacterNode))),
        ];

        let rng = StdRng::from_entropy();
        let mut system = ActionCalculationSystem::new(constant_comparison_rules, rng);

        let battle_context = BattleContext::new(&character, &player, &enemy);
        let action = system.calculate_action(&battle_context);
        // 100 > 50は常に真なので、常にHeal
        assert_eq!(action.as_ref().map(|a| a.get_action_name()), Some("Heal"), "100 > 50 should always be true, so Heal");
    }

    #[test]
    fn test_empty_rules_fallback() {
        // 空のルールリストの場合のテスト
        let character = Character::new(34, "Test".to_string(), 100, 50, 25);
        let player = Character::new(35, "Player".to_string(), 100, 50, 25);
        let enemy = Character::new(36, "Enemy".to_string(), 80, 30, 20);

        let empty_rules: Vec<RuleNode> = vec![];
        let rng = StdRng::from_entropy();
        let mut system = ActionCalculationSystem::new(empty_rules, rng);

        let battle_context = BattleContext::new(&character, &player, &enemy);
        let action = system.calculate_action(&battle_context);
        assert!(action.is_none(), "Empty rules should return None");
    }

    #[test]
    fn test_all_conditions_fail() {
        // すべての条件が失敗する場合のテスト
        let character = Character::new(37, "Test".to_string(), 100, 50, 25);
        let player = Character::new(38, "Player".to_string(), 100, 50, 25);
        let enemy = Character::new(39, "Enemy".to_string(), 80, 30, 20);

        // 絶対に満たされない条件のみ
        let impossible_rules: Vec<RuleNode> = vec![
            Box::new(ConditionCheckNode::new(
                Box::new(GreaterThanConditionNode::new(
                    Box::new(ConstantValueNode::new(10)),
                    Box::new(ConstantValueNode::new(100)),
                )),
                Box::new(HealActionNode::new(Box::new(ActingCharacterNode))),
            )),
            Box::new(ConditionCheckNode::new(
                Box::new(GreaterThanConditionNode::new(
                    Box::new(CharacterHpFromNode::new(Box::new(ActingCharacterNode))),
                    Box::new(ConstantValueNode::new(200)),
                )),
                Box::new(StrikeActionNode::new(Box::new(ActingCharacterNode))),
            )),
        ];

        let rng = StdRng::from_entropy();
        let mut system = ActionCalculationSystem::new(impossible_rules, rng);

        let battle_context = BattleContext::new(&character, &player, &enemy);
        let action = system.calculate_action(&battle_context);
        assert!(action.is_none(), "All failing conditions should return None");
    }

}