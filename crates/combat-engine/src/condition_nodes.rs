// Condition nodes - nodes that evaluate to true/false for decision making

use rand::Rng;
use super::value_nodes::ValueNode;

// Trait for nodes that evaluate to boolean conditions
pub trait ConditionNode: Send + Sync + std::fmt::Debug {
    fn evaluate(&self, character: &crate::Character, rng: &mut dyn rand::RngCore) -> bool;
}

impl ConditionNode for Box<dyn ConditionNode> {
    fn evaluate(&self, character: &crate::Character, rng: &mut dyn rand::RngCore) -> bool {
        (**self).evaluate(character, rng)
    }
}

// Random condition node - randomly returns true or false
#[derive(Debug)]
pub struct RandomConditionNode;

impl ConditionNode for RandomConditionNode {
    fn evaluate(&self, _character: &crate::Character, rng: &mut dyn rand::RngCore) -> bool {
        rng.gen_bool(0.5)
    }
}

// Greater than condition node - compares two values
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
    fn test_random_condition_node() {
        let character = Character::new("Test".to_string(), 100, 50, 25);
        let random = RandomConditionNode;
        
        // Test with seeded RNG for deterministic behavior
        let mut rng1 = StdRng::seed_from_u64(42);
        let mut rng2 = StdRng::seed_from_u64(42);
        let result1 = random.evaluate(&character, &mut rng1);
        let result2 = random.evaluate(&character, &mut rng2);
        
        // Same seed should produce same result
        assert_eq!(result1, result2);
        
        // Test with random RNG for variety
        let mut rng = StdRng::from_entropy();
        let mut true_count = 0;
        let mut false_count = 0;
        
        for _ in 0..100 {
            if random.evaluate(&character, &mut rng) {
                true_count += 1;
            } else {
                false_count += 1;
            }
        }
        
        assert!(true_count > 0, "Should have some true results");
        assert!(false_count > 0, "Should have some false results");
    }

    #[test]
    fn test_seeded_random_deterministic() {
        let character = Character::new("Test".to_string(), 100, 50, 25);
        let random = RandomConditionNode;
        
        // Test deterministic behavior with seed
        let seed = 12345;
        let mut rng1 = StdRng::seed_from_u64(seed);
        let mut rng2 = StdRng::seed_from_u64(seed);
        
        let result1 = random.evaluate(&character, &mut rng1);
        let result2 = random.evaluate(&character, &mut rng2);
        
        assert_eq!(result1, result2);
    }

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