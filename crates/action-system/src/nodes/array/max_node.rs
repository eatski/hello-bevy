use crate::nodes::unified_node::Node;
use crate::nodes::evaluation_context::EvaluationContext;
use crate::core::NodeResult;

/// Array内の最大値を返すノード
pub struct MaxNode {
    array_node: Box<dyn Node<Vec<i32>>>,
}

impl MaxNode {
    pub fn new(array_node: Box<dyn Node<Vec<i32>>>) -> Self {
        Self { array_node }
    }
}

impl Node<i32> for MaxNode {
    fn evaluate(&self, eval_context: &EvaluationContext, rng: &mut dyn rand::RngCore) -> NodeResult<i32> {
        let array = self.array_node.evaluate(eval_context, rng)?;
        
        if array.is_empty() {
            return Err(crate::NodeError::EvaluationError("Cannot find max of empty array".to_string()));
        }
        
        Ok(*array.iter().max().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::nodes::array::ConstantArrayNode;
    use crate::Character;
    use crate::Team;
    use crate::TeamSide;
    use crate::nodes::character::BattleContext;
    use rand::SeedableRng;

    #[test]
    fn test_max_node_basic() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(12345);
        
        let character = Character::new(1, "Test".to_string(), 100, 100, 10);
        let team = Team::new("Test Team".to_string(), vec![character.clone()]);
        let battle_context = BattleContext::new(&character, TeamSide::Player, &team, &team);
        let eval_context = EvaluationContext::new(&battle_context);
        
        let array_node = Box::new(ConstantArrayNode::new(vec![10, 5, 30, 15, 20]));
        let max_node = MaxNode::new(array_node);
        
        let result = max_node.evaluate(&eval_context, &mut rng).unwrap();
        assert_eq!(result, 30);
    }
    
    #[test]
    fn test_max_node_single_element() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(12345);
        
        let character = Character::new(1, "Test".to_string(), 100, 100, 10);
        let team = Team::new("Test Team".to_string(), vec![character.clone()]);
        let battle_context = BattleContext::new(&character, TeamSide::Player, &team, &team);
        let eval_context = EvaluationContext::new(&battle_context);
        
        let array_node = Box::new(ConstantArrayNode::new(vec![42]));
        let max_node = MaxNode::new(array_node);
        
        let result = max_node.evaluate(&eval_context, &mut rng).unwrap();
        assert_eq!(result, 42);
    }
    
    #[test]
    fn test_max_node_negative_values() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(12345);
        
        let character = Character::new(1, "Test".to_string(), 100, 100, 10);
        let team = Team::new("Test Team".to_string(), vec![character.clone()]);
        let battle_context = BattleContext::new(&character, TeamSide::Player, &team, &team);
        let eval_context = EvaluationContext::new(&battle_context);
        
        let array_node = Box::new(ConstantArrayNode::new(vec![-10, -5, -30, -15]));
        let max_node = MaxNode::new(array_node);
        
        let result = max_node.evaluate(&eval_context, &mut rng).unwrap();
        assert_eq!(result, -5);
    }
    
    #[test]
    fn test_max_node_empty_array() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(12345);
        
        let character = Character::new(1, "Test".to_string(), 100, 100, 10);
        let team = Team::new("Test Team".to_string(), vec![character.clone()]);
        let battle_context = BattleContext::new(&character, TeamSide::Player, &team, &team);
        let eval_context = EvaluationContext::new(&battle_context);
        
        let array_node = Box::new(ConstantArrayNode::new(vec![]));
        let max_node = MaxNode::new(array_node);
        
        let result = max_node.evaluate(&eval_context, &mut rng);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Cannot find max of empty array"));
    }
}