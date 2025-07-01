// Constant value node - returns a fixed numeric value

use crate::nodes::unified_node::Node;

#[derive(Debug)]
pub struct ConstantValueNode {
    value: i32,
}

impl ConstantValueNode {
    pub fn new(value: i32) -> Self {
        Self { value: value.clamp(1, 100) }
    }
}

// Unified implementation
impl Node<i32> for ConstantValueNode {
    fn evaluate(&self, _eval_context: &crate::nodes::evaluation_context::EvaluationContext, _rng: &mut dyn rand::RngCore) -> crate::core::NodeResult<i32> {
        Ok(self.value)
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
    fn test_constant_value_node() {
        let player = Character::new(1, "Player".to_string(), 100, 50, 25);
        let enemy = Character::new(2, "Enemy".to_string(), 80, 30, 20);
        let acting_character = Character::new(3, "Test".to_string(), 100, 50, 25);
        
        let player_team = Team::new("Player Team".to_string(), vec![player.clone(), acting_character.clone()]);
        let enemy_team = Team::new("Enemy Team".to_string(), vec![enemy.clone()]);
        let battle_context = crate::BattleContext::new(&acting_character, TeamSide::Player, &player_team, &enemy_team);
        
        let mut rng = StdRng::from_entropy();
        
        // Test unified Constant value node
        let value_node = ConstantValueNode::new(42);
        let eval_context = EvaluationContext::new(&battle_context);
        assert_eq!(Node::<i32>::evaluate(&value_node, &eval_context, &mut rng), Ok(42));
    }

    #[test]
    fn test_constant_value_node_unified() {
        let player = Character::new(1, "Player".to_string(), 100, 50, 25);
        let enemy = Character::new(2, "Enemy".to_string(), 80, 30, 20);
        let acting_character = Character::new(3, "Test".to_string(), 100, 50, 25);
        
        let player_team = Team::new("Player Team".to_string(), vec![player.clone(), acting_character.clone()]);
        let enemy_team = Team::new("Enemy Team".to_string(), vec![enemy.clone()]);
        let battle_context = crate::BattleContext::new(&acting_character, TeamSide::Player, &player_team, &enemy_team);
        
        let mut rng = StdRng::from_entropy();
        
        // Test unified Constant value node
        let value_node = ConstantValueNode::new(42);
        let eval_context = EvaluationContext::new(&battle_context);
        assert_eq!(Node::<i32>::evaluate(&value_node, &eval_context, &mut rng), Ok(42));

        // Test as boxed trait object
        let boxed_node: Box<dyn Node<i32>> = Box::new(ConstantValueNode::new(99));
        let boxed_result = boxed_node.evaluate(&eval_context, &mut rng).unwrap();
        assert_eq!(boxed_result, 99);

        // Test clamping behavior
        let clamped_low = ConstantValueNode::new(-10);
        let clamped_high = ConstantValueNode::new(200);
        assert_eq!(Node::<i32>::evaluate(&clamped_low, &eval_context, &mut rng), Ok(1));
        assert_eq!(Node::<i32>::evaluate(&clamped_high, &eval_context, &mut rng), Ok(100));
    }
}