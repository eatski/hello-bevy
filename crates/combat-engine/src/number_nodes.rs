// Number nodes - nodes that evaluate to numeric values

// Trait for nodes that evaluate to numbers
pub trait NumberNode: Send + Sync + std::fmt::Debug {
    fn evaluate(&self, character: &crate::Character, rng: &mut dyn rand::RngCore) -> i32;
}

impl NumberNode for Box<dyn NumberNode> {
    fn evaluate(&self, character: &crate::Character, rng: &mut dyn rand::RngCore) -> i32 {
        (**self).evaluate(character, rng)
    }
}

// Concrete number node implementations
#[derive(Debug)]
pub struct ConstantNode {
    value: i32,
}

impl ConstantNode {
    pub fn new(value: i32) -> Self {
        Self { value: value.clamp(1, 100) }
    }
}

impl NumberNode for ConstantNode {
    fn evaluate(&self, _character: &crate::Character, _rng: &mut dyn rand::RngCore) -> i32 {
        self.value
    }
}

#[derive(Debug)]
pub struct CharacterHPNode;

impl NumberNode for CharacterHPNode {
    fn evaluate(&self, character: &crate::Character, _rng: &mut dyn rand::RngCore) -> i32 {
        character.hp
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Character;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_constant_node() {
        let character = Character::new("Test".to_string(), 100, 50, 25);
        let mut rng = StdRng::from_entropy();
        
        // Test Constant node
        let number_node = ConstantNode::new(42);
        assert_eq!(number_node.evaluate(&character, &mut rng), 42);
    }

    #[test]
    fn test_character_hp_node() {
        let character = Character::new("Test".to_string(), 100, 50, 25);
        let mut rng = StdRng::from_entropy();
        
        // Test CharacterHP node
        let char_hp_node = CharacterHPNode;
        assert_eq!(char_hp_node.evaluate(&character, &mut rng), 100);
    }
}