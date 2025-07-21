// Constant value node - returns a fixed numeric value

use crate::nodes::unified_node::Node;

#[derive(Debug)]
pub struct ConstantValueNode {
    value: i32,
}

impl ConstantValueNode {
    pub fn new(value: i32) -> Self {
        Self { value: value.clamp(1, 100) }
    }
}

// Unified implementation
impl Node<i32> for ConstantValueNode {
    fn evaluate(&self, _eval_context: &mut crate::nodes::evaluation_context::EvaluationContext) -> crate::core::NodeResult<i32> {
        Ok(self.value)
    }
}

