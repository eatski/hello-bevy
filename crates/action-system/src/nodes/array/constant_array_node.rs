// ConstantArrayNode - returns a fixed array of values

use crate::core::NodeResult;
use crate::nodes::evaluation_context::EvaluationContext;
use crate::nodes::unified_node::Node;

/// Node that returns a constant array of values
#[derive(Debug)]
pub struct ConstantArrayNode {
    values: Vec<i32>,
}

impl ConstantArrayNode {
    pub fn new(values: Vec<i32>) -> Self {
        Self { values }
    }
}

impl Node<Vec<i32>> for ConstantArrayNode {
    fn evaluate(&self, _eval_context: &EvaluationContext, _rng: &mut dyn rand::RngCore) -> NodeResult<Vec<i32>> {
        Ok(self.values.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{BattleContext, Character, Team, TeamSide};
    use rand::SeedableRng;

    #[test]
    fn test_constant_array_node() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(12345);
        
        let char1 = Character::new(1, "Test".to_string(), 100, 100, 10);
        let player_team = Team::new("Player".to_string(), vec![char1.clone()]);
        let enemy_team = Team::new("Enemy".to_string(), vec![]);
        
        let battle_context = BattleContext::new(&char1, TeamSide::Player, &player_team, &enemy_team);
        
        let values = vec![10, 20, 30, 40, 50];
        let array_node = ConstantArrayNode::new(values.clone());
        
        let eval_context = EvaluationContext::new(&battle_context);
        let result = Node::<Vec<i32>>::evaluate(&array_node, &eval_context, &mut rng).unwrap();
        
        assert_eq!(result, values);
    }
    
    #[test]
    fn test_empty_constant_array() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(12345);
        
        let char1 = Character::new(1, "Test".to_string(), 100, 100, 10);
        let player_team = Team::new("Player".to_string(), vec![char1.clone()]);
        let enemy_team = Team::new("Enemy".to_string(), vec![]);
        
        let battle_context = BattleContext::new(&char1, TeamSide::Player, &player_team, &enemy_team);
        
        let array_node = ConstantArrayNode::new(vec![]);
        
        let eval_context = EvaluationContext::new(&battle_context);
        let result = Node::<Vec<i32>>::evaluate(&array_node, &eval_context, &mut rng).unwrap();
        
        assert_eq!(result, Vec::<i32>::new());
    }
}