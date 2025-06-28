// Random condition node - randomly returns true or false

use rand::Rng;
use super::condition_nodes::ConditionNode;

#[derive(Debug)]
pub struct RandomConditionNode;

impl ConditionNode for RandomConditionNode {
    fn evaluate(&self, _battle_context: &crate::BattleContext, rng: &mut dyn rand::RngCore) -> bool {
        rng.gen_bool(0.5)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Character;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_random_condition_node() {
        let player = Character::new("Player".to_string(), 100, 50, 25);
        let enemy = Character::new("Enemy".to_string(), 80, 30, 20);
        let acting_character = Character::new("Test".to_string(), 100, 50, 25);
        let battle_context = crate::BattleContext::new(&acting_character, &player, &enemy);
        
        let random = RandomConditionNode;
        
        // Test with seeded RNG for deterministic behavior
        let mut rng1 = StdRng::seed_from_u64(42);
        let mut rng2 = StdRng::seed_from_u64(42);
        let result1 = random.evaluate(&battle_context, &mut rng1);
        let result2 = random.evaluate(&battle_context, &mut rng2);
        
        // Same seed should produce same result
        assert_eq!(result1, result2);
        
        // Test with random RNG for variety
        let mut rng = StdRng::from_entropy();
        let mut true_count = 0;
        let mut false_count = 0;
        
        for _ in 0..100 {
            if random.evaluate(&battle_context, &mut rng) {
                true_count += 1;
            } else {
                false_count += 1;
            }
        }
        
        assert!(true_count > 0, "Should have some true results");
        assert!(false_count > 0, "Should have some false results");
    }

    #[test]
    fn test_single_rng_multiple_evaluations_differ() {
        // 1つのRNGで複数回評価し、結果が変わることを検証
        let player = Character::new("Player".to_string(), 100, 50, 25);
        let enemy = Character::new("Enemy".to_string(), 80, 30, 20);
        let acting_character = Character::new("Test".to_string(), 100, 50, 25);
        let battle_context = crate::BattleContext::new(&acting_character, &player, &enemy);
        
        let random = RandomConditionNode;
        let mut rng = StdRng::seed_from_u64(42);
        
        let mut results = Vec::new();
        
        // 同一RNGで20回評価
        for _ in 0..20 {
            let result = random.evaluate(&battle_context, &mut rng);
            results.push(result);
        }
        
        // 全て同じ結果ではないことを確認
        let first_result = results[0];
        let has_different_result = results.iter().any(|&result| result != first_result);
        
        assert!(has_different_result, "Multiple evaluations with same RNG should produce different results");
    }
}