use crate::nodes::unified_node::{CoreNode as Node, BoxedNode};
use crate::core::{NodeResult, Numeric};
use crate::nodes::evaluation_context::EvaluationContext;

/// Array内の最小値を返すノード（Numeric対応）
pub struct MinNode<T: Numeric> {
    array_node: BoxedNode<Vec<T>>,
}

impl<T: Numeric> MinNode<T> {
    pub fn new(array_node: BoxedNode<Vec<T>>) -> Self {
        Self { array_node }
    }
}

impl<'a, T: Numeric> Node<T, EvaluationContext<'a>> for MinNode<T> {
    fn evaluate(&self, eval_context: &mut EvaluationContext) -> NodeResult<T> {
        let array = self.array_node.evaluate(eval_context)?;
        
        if array.is_empty() {
            return Err(crate::NodeError::EvaluationError("Cannot find min of empty array".to_string()));
        }
        
        // Use Numeric trait's min method
        let min_value = array.into_iter().reduce(|a, b| a.min(b)).unwrap();
        Ok(min_value)
    }
}

