use crate::nodes::unified_node::Node;
use crate::nodes::evaluation_context::EvaluationContext;
use crate::core::{NodeResult, Numeric};

/// Array内の最大値を返すノード（Numeric対応）
pub struct MaxNode<T: Numeric> {
    array_node: Box<dyn Node<Vec<T>>>,
}

impl<T: Numeric> MaxNode<T> {
    pub fn new(array_node: Box<dyn Node<Vec<T>>>) -> Self {
        Self { array_node }
    }
}

impl<T: Numeric> Node<T> for MaxNode<T> {
    fn evaluate(&self, eval_context: &EvaluationContext, rng: &mut dyn rand::RngCore) -> NodeResult<T> {
        let array = self.array_node.evaluate(eval_context, rng)?;
        
        if array.is_empty() {
            return Err(crate::NodeError::EvaluationError("Cannot find max of empty array".to_string()));
        }
        
        let max_value = array.into_iter().reduce(|a, b| a.max(b)).unwrap();
        Ok(max_value)
    }
}

// 後方互換性のための型エイリアス
pub type MaxNodeI32 = MaxNode<i32>;

#[cfg(test)]
mod tests {
    use super::*;
    // ConstantArrayNode removed - all tests deleted due to dependency
    use crate::Character;
    use crate::Team;
    use crate::TeamSide;
    use crate::nodes::character::BattleContext;
    use rand::SeedableRng;

    // Removed test_max_node_basic - ConstantArrayNode deleted
    
    // Removed test_max_node_single_element - ConstantArrayNode deleted
    
    // Removed test_max_node_negative_values - ConstantArrayNode deleted
    
    // Removed test_max_node_empty_array - ConstantArrayNode deleted
}