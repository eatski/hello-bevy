// Greater than condition node - compares two values

use super::condition_nodes::ConditionNode;
use super::value_nodes::ValueNode;

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
    fn evaluate(&self, character: &crate::Character, rng: &mut dyn rand::RngCore) -> bool {
        self.left.evaluate(character, rng) > self.right.evaluate(character, rng)
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
        let character = Character::new("Test".to_string(), 100, 50, 25);
        let mut rng = StdRng::from_entropy();
        
        // Test GreaterThanConditionNode
        let greater_than_node = GreaterThanConditionNode::new(
            Box::new(ConstantValueNode::new(60)),
            Box::new(ConstantValueNode::new(40)),
        );
        assert_eq!(greater_than_node.evaluate(&character, &mut rng), true);
        
        let greater_than_node_false = GreaterThanConditionNode::new(
            Box::new(ConstantValueNode::new(30)),
            Box::new(ConstantValueNode::new(50)),
        );
        assert_eq!(greater_than_node_false.evaluate(&character, &mut rng), false);
    }
}