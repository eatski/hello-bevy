// Constant value node - returns a fixed numeric value

use super::value_nodes::ValueNode;

#[derive(Debug)]
pub struct ConstantValueNode {
    value: i32,
}

impl ConstantValueNode {
    pub fn new(value: i32) -> Self {
        Self { value: value.clamp(1, 100) }
    }
}

impl ValueNode for ConstantValueNode {
    fn evaluate(&self, _character: &crate::Character, _rng: &mut dyn rand::RngCore) -> i32 {
        self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Character;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_constant_value_node() {
        let character = Character::new("Test".to_string(), 100, 50, 25);
        let mut rng = StdRng::from_entropy();
        
        // Test Constant value node
        let value_node = ConstantValueNode::new(42);
        assert_eq!(value_node.evaluate(&character, &mut rng), 42);
    }
}