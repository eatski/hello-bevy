// Greater than condition node - compares two values

use crate::nodes::evaluation_context::EvaluationContext;
use crate::nodes::unified_node::Node;

pub struct GreaterThanConditionNode {
    pub left: Box<dyn Node<i32>>,
    pub right: Box<dyn Node<i32>>,
}

impl GreaterThanConditionNode {
    pub fn new(left: Box<dyn Node<i32>>, right: Box<dyn Node<i32>>) -> Self {
        Self { left, right }
    }
}

impl Node<bool> for GreaterThanConditionNode {
    fn evaluate(&self, eval_context: &EvaluationContext, rng: &mut dyn rand::RngCore) -> crate::core::NodeResult<bool> {
        let left_value = self.left.evaluate(eval_context, rng)?;
        let right_value = self.right.evaluate(eval_context, rng)?;
        Ok(left_value > right_value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Character, Team, TeamSide};
    use crate::ConstantValueNode;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_greater_than_condition_node() {
        let player = Character::new(1, "Player".to_string(), 100, 50, 25);
        let enemy = Character::new(2, "Enemy".to_string(), 80, 30, 20);
        let acting_character = Character::new(3, "Test".to_string(), 100, 50, 25);
        
        let player_team = Team::new("Player Team".to_string(), vec![player.clone(), acting_character.clone()]);
        let enemy_team = Team::new("Enemy Team".to_string(), vec![enemy.clone()]);
        let battle_context = crate::BattleContext::new(&acting_character, TeamSide::Player, &player_team, &enemy_team);
        
        let mut rng = StdRng::from_entropy();
        
        // Test GreaterThanConditionNode
        let greater_than_node = GreaterThanConditionNode::new(
            Box::new(ConstantValueNode::new(60)),
            Box::new(ConstantValueNode::new(40)),
        );
        let eval_context = EvaluationContext::new(&battle_context);
        assert_eq!(Node::<bool>::evaluate(&greater_than_node, &eval_context, &mut rng), Ok(true));
        
        let greater_than_node_false = GreaterThanConditionNode::new(
            Box::new(ConstantValueNode::new(30)),
            Box::new(ConstantValueNode::new(50)),
        );
        assert_eq!(Node::<bool>::evaluate(&greater_than_node_false, &eval_context, &mut rng), Ok(false));
    }

    #[test]
    fn test_greater_than_condition_node_unified() {
        let player = Character::new(1, "Player".to_string(), 100, 50, 25);
        let enemy = Character::new(2, "Enemy".to_string(), 80, 30, 20);
        let acting_character = Character::new(3, "Test".to_string(), 100, 50, 25);
        
        let player_team = Team::new("Player Team".to_string(), vec![player.clone(), acting_character.clone()]);
        let enemy_team = Team::new("Enemy Team".to_string(), vec![enemy.clone()]);
        let battle_context = crate::BattleContext::new(&acting_character, TeamSide::Player, &player_team, &enemy_team);
        
        let mut rng = StdRng::from_entropy();
        
        // Test GreaterThanConditionNode using unified Node<bool>
        let greater_than_node = GreaterThanConditionNode::new(
            Box::new(ConstantValueNode::new(60)),
            Box::new(ConstantValueNode::new(40)),
        );
        let eval_context = EvaluationContext::new(&battle_context);
        assert_eq!(Node::<bool>::evaluate(&greater_than_node, &eval_context, &mut rng), Ok(true));
        
        let greater_than_node_false = GreaterThanConditionNode::new(
            Box::new(ConstantValueNode::new(30)),
            Box::new(ConstantValueNode::new(50)),
        );
        assert_eq!(Node::<bool>::evaluate(&greater_than_node_false, &eval_context, &mut rng), Ok(false));
    }
}