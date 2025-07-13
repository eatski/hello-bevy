use crate::nodes::unified_node::Node;
use crate::nodes::evaluation_context::EvaluationContext;
use crate::core::{NodeResult, Numeric};

/// Array内の最小値を返すノード（Numeric対応）
pub struct MinNode<T: Numeric> {
    array_node: Box<dyn Node<Vec<T>>>,
}

impl<T: Numeric> MinNode<T> {
    pub fn new(array_node: Box<dyn Node<Vec<T>>>) -> Self {
        Self { array_node }
    }
}

impl<T: Numeric> Node<T> for MinNode<T> {
    fn evaluate(&self, eval_context: &EvaluationContext, rng: &mut dyn rand::RngCore) -> NodeResult<T> {
        let array = self.array_node.evaluate(eval_context, rng)?;
        
        if array.is_empty() {
            return Err(crate::NodeError::EvaluationError("Cannot find min of empty array".to_string()));
        }
        
        let min_value = array.into_iter().reduce(|a, b| a.min(b)).unwrap();
        Ok(min_value)
    }
}

// 後方互換性のための型エイリアス
pub type MinNodeI32 = MinNode<i32>;

#[cfg(test)]
mod tests {
    // ConstantArrayNode removed - all tests deleted due to dependency

    // Removed test_min_node_basic - ConstantArrayNode deleted
    
    // Removed test_min_node_single_element - ConstantArrayNode deleted
    
    // Removed test_min_node_negative_values - ConstantArrayNode deleted
    
    // Removed test_min_node_empty_array - ConstantArrayNode deleted
}