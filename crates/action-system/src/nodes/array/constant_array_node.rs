// ConstantArrayNode - returns a fixed array of values

use crate::core::NodeResult;
use crate::nodes::evaluation_context::EvaluationContext;
use crate::nodes::unified_node::Node;

/// Node that returns a constant array of values
#[derive(Debug)]
pub struct ConstantArrayNode {
    values: Vec<i32>,
}

impl ConstantArrayNode {
    pub fn new(values: Vec<i32>) -> Self {
        Self { values }
    }
}

impl Node<Vec<i32>> for ConstantArrayNode {
    fn evaluate(&self, _eval_context: &EvaluationContext, _rng: &mut dyn rand::RngCore) -> NodeResult<Vec<i32>> {
        Ok(self.values.clone())
    }
}

