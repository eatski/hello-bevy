// Boolean nodes - nodes that evaluate to true/false

use rand::Rng;
use super::number_nodes::NumberNode;

// Trait for nodes that evaluate to boolean
pub trait BoolNode: Send + Sync + std::fmt::Debug {
    fn evaluate(&self, character: &crate::Character, rng: &mut dyn rand::RngCore) -> bool;
}

impl BoolNode for Box<dyn BoolNode> {
    fn evaluate(&self, character: &crate::Character, rng: &mut dyn rand::RngCore) -> bool {
        (**self).evaluate(character, rng)
    }
}

// Concrete bool node implementations
#[derive(Debug)]
pub struct TrueOrFalseRandomNode;

impl BoolNode for TrueOrFalseRandomNode {
    fn evaluate(&self, _character: &crate::Character, rng: &mut dyn rand::RngCore) -> bool {
        rng.gen_bool(0.5)
    }
}

#[derive(Debug)]
pub struct GreaterThanNode {
    pub left: Box<dyn NumberNode>,
    pub right: Box<dyn NumberNode>,
}

impl GreaterThanNode {
    pub fn new(left: Box<dyn NumberNode>, right: Box<dyn NumberNode>) -> Self {
        Self { left, right }
    }
}

impl BoolNode for GreaterThanNode {
    fn evaluate(&self, character: &crate::Character, rng: &mut dyn rand::RngCore) -> bool {
        self.left.evaluate(character, rng) > self.right.evaluate(character, rng)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Character;
    use crate::{ConstantNode};
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_true_or_false_random() {
        let character = Character::new("Test".to_string(), 100, 50, 25);
        let random = TrueOrFalseRandomNode;
        
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
        let random = TrueOrFalseRandomNode;
        
        // Test deterministic behavior with seed
        let seed = 12345;
        let mut rng1 = StdRng::seed_from_u64(seed);
        let mut rng2 = StdRng::seed_from_u64(seed);
        
        let result1 = random.evaluate(&character, &mut rng1);
        let result2 = random.evaluate(&character, &mut rng2);
        
        assert_eq!(result1, result2);
    }

    #[test]
    fn test_greater_than_node() {
        let character = Character::new("Test".to_string(), 100, 50, 25);
        let mut rng = StdRng::from_entropy();
        
        // Test GreaterThanNode
        let greater_than_node = GreaterThanNode::new(
            Box::new(ConstantNode::new(60)),
            Box::new(ConstantNode::new(40)),
        );
        assert_eq!(greater_than_node.evaluate(&character, &mut rng), true);
        
        let greater_than_node_false = GreaterThanNode::new(
            Box::new(ConstantNode::new(30)),
            Box::new(ConstantNode::new(50)),
        );
        assert_eq!(greater_than_node_false.evaluate(&character, &mut rng), false);
    }
}