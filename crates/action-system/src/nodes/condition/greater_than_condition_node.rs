// Greater than condition node - compares two values

use super::condition_nodes::ConditionNode;
use crate::nodes::value::ValueNode;

#[derive(Debug)]
pub struct GreaterThanConditionNode {
    pub left: Box<dyn ValueNode>,
    pub right: Box<dyn ValueNode>,
}

impl GreaterThanConditionNode {
    pub fn new(left: Box<dyn ValueNode>, right: Box<dyn ValueNode>) -> Self {
        Self { left, right }
    }
}

impl ConditionNode for GreaterThanConditionNode {
    fn evaluate(&self, battle_context: &crate::BattleContext, rng: &mut dyn rand::RngCore) -> crate::core::NodeResult<bool> {
        let left_value = self.left.evaluate(battle_context, rng)?;
        let right_value = self.right.evaluate(battle_context, rng)?;
        Ok(left_value > right_value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Character;
    use crate::ConstantValueNode;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_greater_than_condition_node() {
        let player = Character::new("Player".to_string(), 100, 50, 25);
        let enemy = Character::new("Enemy".to_string(), 80, 30, 20);
        let acting_character = Character::new("Test".to_string(), 100, 50, 25);
        let battle_context = crate::BattleContext::new(&acting_character, &player, &enemy);
        
        let mut rng = StdRng::from_entropy();
        
        // Test GreaterThanConditionNode
        let greater_than_node = GreaterThanConditionNode::new(
            Box::new(ConstantValueNode::new(60)),
            Box::new(ConstantValueNode::new(40)),
        );
        assert_eq!(greater_than_node.evaluate(&battle_context, &mut rng), Ok(true));
        
        let greater_than_node_false = GreaterThanConditionNode::new(
            Box::new(ConstantValueNode::new(30)),
            Box::new(ConstantValueNode::new(50)),
        );
        assert_eq!(greater_than_node_false.evaluate(&battle_context, &mut rng), Ok(false));
    }
}