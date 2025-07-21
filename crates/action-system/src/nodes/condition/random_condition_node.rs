// Random condition node - randomly returns true or false

use rand::Rng;
use node_core::Node;
use crate::nodes::evaluation_context::EvaluationContext;

#[derive(Debug)]
pub struct RandomConditionNode;

// Unified implementation
impl<'a> Node<bool, EvaluationContext<'a>> for RandomConditionNode {
    fn evaluate(&self, eval_context: &mut EvaluationContext<'a>) -> crate::core::NodeResult<bool> {
        Ok(eval_context.rng.gen_bool(0.5))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Character, Team, TeamSide};
    use crate::nodes::evaluation_context::EvaluationContext;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_random_condition_node() {
        let player = Character::new(1, "Player".to_string(), 100, 50, 25);
        let enemy = Character::new(2, "Enemy".to_string(), 80, 30, 20);
        let acting_character = Character::new(3, "Test".to_string(), 100, 50, 25);
        
        let player_team = Team::new("Player Team".to_string(), vec![player.clone(), acting_character.clone()]);
        let enemy_team = Team::new("Enemy Team".to_string(), vec![enemy.clone()]);
        let battle_context = crate::BattleContext::new(&acting_character, TeamSide::Player, &player_team, &enemy_team);
        
        let random = RandomConditionNode;
        
        // Test with seeded RNG for deterministic behavior
        let mut rng1 = StdRng::seed_from_u64(42);
        let mut rng2 = StdRng::seed_from_u64(42);
        let mut eval_context1 = EvaluationContext::new(&battle_context, &mut rng1);
        let mut eval_context2 = EvaluationContext::new(&battle_context, &mut rng2);
        let result1 = Node::<bool>::evaluate(&random, &mut eval_context1).unwrap();
        let result2 = Node::<bool>::evaluate(&random, &mut eval_context2).unwrap();
        
        // Same seed should produce same result
        assert_eq!(result1, result2);
        
        // Test with random RNG for variety - use different RNG instances
        let mut true_count = 0;
        let mut false_count = 0;
        
        for i in 0..100 {
            let mut local_rng = StdRng::seed_from_u64(42 + i as u64);
            let mut eval_context = EvaluationContext::new(&battle_context, &mut local_rng);
            if Node::<bool>::evaluate(&random, &mut eval_context).unwrap() {
                true_count += 1;
            } else {
                false_count += 1;
            }
        }
        
        assert_ne!(true_count, 0, "Should have some true results");
        assert_ne!(false_count, 0, "Should have some false results");
    }

    #[test]
    fn test_single_rng_multiple_evaluations_differ() {
        // 1つのRNGで複数回評価し、結果が変わることを検証
        let player = Character::new(4, "Player".to_string(), 100, 50, 25);
        let enemy = Character::new(5, "Enemy".to_string(), 80, 30, 20);
        let acting_character = Character::new(6, "Test".to_string(), 100, 50, 25);
        
        let player_team = Team::new("Player Team".to_string(), vec![player.clone(), acting_character.clone()]);
        let enemy_team = Team::new("Enemy Team".to_string(), vec![enemy.clone()]);
        let battle_context = crate::BattleContext::new(&acting_character, TeamSide::Player, &player_team, &enemy_team);
        
        let random = RandomConditionNode;
        let mut rng = StdRng::seed_from_u64(42);
        
        let mut results = Vec::new();
        
        // 同一RNGコンテキストで20回評価
        for _ in 0..20 {
            let mut eval_context = EvaluationContext::new(&battle_context, &mut rng);
            let result = Node::<bool>::evaluate(&random, &mut eval_context).unwrap();
            results.push(result);
        }
        
        // 全て同じ結果ではないことを確認
        let first_result = results[0];
        let has_different_result = results.iter().any(|&result| result != first_result);
        
        assert_eq!(has_different_result, true, "Multiple evaluations with same RNG should produce different results");
    }

    #[test]
    fn test_random_condition_node_unified() {
        let player = Character::new(1, "Player".to_string(), 100, 50, 25);
        let enemy = Character::new(2, "Enemy".to_string(), 80, 30, 20);
        let acting_character = Character::new(3, "Test".to_string(), 100, 50, 25);
        
        let player_team = Team::new("Player Team".to_string(), vec![player.clone(), acting_character.clone()]);
        let enemy_team = Team::new("Enemy Team".to_string(), vec![enemy.clone()]);
        let battle_context = crate::BattleContext::new(&acting_character, TeamSide::Player, &player_team, &enemy_team);
        
        let random = RandomConditionNode;
        
        // Test with seeded RNG for deterministic behavior
        let mut rng1 = StdRng::seed_from_u64(42);
        let mut rng2 = StdRng::seed_from_u64(42);
        // Test unified implementation
        let mut eval_context1 = EvaluationContext::new(&battle_context, &mut rng1);
        let mut eval_context2 = EvaluationContext::new(&battle_context, &mut rng2);
        let result1 = Node::<bool>::evaluate(&random, &mut eval_context1).unwrap();
        let result2 = Node::<bool>::evaluate(&random, &mut eval_context2).unwrap();
        
        // Same seed should produce same result
        assert_eq!(result1, result2);
        
        // Test as boxed trait object
        let boxed_node: Box<dyn Node<bool>> = Box::new(RandomConditionNode);
        let mut rng3 = StdRng::seed_from_u64(42);
        let mut eval_context3 = EvaluationContext::new(&battle_context, &mut rng3);
        let boxed_result = boxed_node.evaluate(&mut eval_context3).unwrap();
        assert_eq!(boxed_result, result1);
        
        // Test variety with different RNG instances
        let mut true_count = 0;
        let mut false_count = 0;
        
        for i in 0..100 {
            let mut local_rng = StdRng::seed_from_u64(100 + i as u64);
            let mut eval_context = EvaluationContext::new(&battle_context, &mut local_rng);
            if Node::<bool>::evaluate(&random, &mut eval_context).unwrap() {
                true_count += 1;
            } else {
                false_count += 1;
            }
        }
        
        assert_ne!(true_count, 0, "Should have some true results");
        assert_ne!(false_count, 0, "Should have some false results");
    }
}