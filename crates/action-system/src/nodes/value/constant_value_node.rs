// Constant value node - returns a fixed numeric value

use crate::nodes::unified_node::CoreNode as Node;
use crate::nodes::evaluation_context::EvaluationContext;

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
impl<'a> Node<i32, EvaluationContext<'a>> for ConstantValueNode {
    fn evaluate(&self, _eval_context: &mut EvaluationContext<'a>) -> crate::core::NodeResult<i32> {
        Ok(self.value)
    }
}

