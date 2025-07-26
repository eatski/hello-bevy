use crate::nodes::unified_node::{CoreNode as Node, BoxedNode};
use crate::core::{NodeResult, Numeric};
use crate::nodes::evaluation_context::EvaluationContext;

/// Array内の最大値を返すノード（Numeric対応）
pub struct MaxNode<T: Numeric> {
    array_node: BoxedNode<Vec<T>>,
}

impl<T: Numeric> MaxNode<T> {
    pub fn new(array_node: BoxedNode<Vec<T>>) -> Self {
        Self { array_node }
    }
}

impl<'a, T: Numeric> Node<T, EvaluationContext<'a>> for MaxNode<T> {
    fn evaluate(&self, eval_context: &mut EvaluationContext) -> NodeResult<T> {
        let array = self.array_node.evaluate(eval_context)?;
        
        if array.is_empty() {
            return Err(crate::NodeError::EvaluationError("Cannot find max of empty array".to_string()));
        }
        
        // Use Numeric trait's max method
        let max_value = array.into_iter().reduce(|a, b| a.max(b)).unwrap();
        Ok(max_value)
    }
}

// 後方互換性のための型エイリアス
pub type MaxNodeI32 = MaxNode<i32>;

#[cfg(test)]
mod tests {
    // ConstantArrayNode removed - all tests deleted due to dependency

    // Removed test_max_node_basic - ConstantArrayNode deleted
    
    // Removed test_max_node_single_element - ConstantArrayNode deleted
    
    // Removed test_max_node_negative_values - ConstantArrayNode deleted
    
    // Removed test_max_node_empty_array - ConstantArrayNode deleted
}