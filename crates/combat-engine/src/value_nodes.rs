// Value nodes - nodes that evaluate to numeric values for calculations

// Trait for nodes that evaluate to numeric values
pub trait ValueNode: Send + Sync + std::fmt::Debug {
    fn evaluate(&self, character: &crate::Character, rng: &mut dyn rand::RngCore) -> i32;
}

impl ValueNode for Box<dyn ValueNode> {
    fn evaluate(&self, character: &crate::Character, rng: &mut dyn rand::RngCore) -> i32 {
        (**self).evaluate(character, rng)
    }
}

// Constant value node - returns a fixed numeric value
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

// Character HP value node - returns character's current HP
#[derive(Debug)]
pub struct CharacterHpValueNode;

impl ValueNode for CharacterHpValueNode {
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
    fn test_constant_value_node() {
        let character = Character::new("Test".to_string(), 100, 50, 25);
        let mut rng = StdRng::from_entropy();
        
        // Test Constant value node
        let value_node = ConstantValueNode::new(42);
        assert_eq!(value_node.evaluate(&character, &mut rng), 42);
    }

    #[test]
    fn test_character_hp_value_node() {
        let character = Character::new("Test".to_string(), 100, 50, 25);
        let mut rng = StdRng::from_entropy();
        
        // Test CharacterHP value node
        let char_hp_node = CharacterHpValueNode;
        assert_eq!(char_hp_node.evaluate(&character, &mut rng), 100);
    }
}