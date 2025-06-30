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
    fn evaluate(&self, _battle_context: &crate::BattleContext, _rng: &mut dyn rand::RngCore) -> crate::core::NodeResult<i32> {
        Ok(self.value)
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
        let player = Character::new("Player".to_string(), 100, 50, 25);
        let enemy = Character::new("Enemy".to_string(), 80, 30, 20);
        let acting_character = Character::new("Test".to_string(), 100, 50, 25);
        let battle_context = crate::BattleContext::new(&acting_character, &player, &enemy);
        
        let mut rng = StdRng::from_entropy();
        
        // Test Constant value node
        let value_node = ConstantValueNode::new(42);
        assert_eq!(value_node.evaluate(&battle_context, &mut rng), Ok(42));
    }
}