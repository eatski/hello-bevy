use crate::nodes::unified_node::{CoreNode as Node, BoxedNode};
use crate::core::{NodeResult, Numeric};
use crate::nodes::evaluation_context::EvaluationContext;

/// Array内の最小値を返すノード（Numeric対応）
pub struct MinNode<T: Numeric + Clone> {
    array_node: BoxedNode<Vec<T>>,
}

impl<T: Numeric + Clone> MinNode<T> {
    pub fn new(array_node: BoxedNode<Vec<T>>) -> Self {
        Self { array_node }
    }
}

impl<'a, T: Numeric + Clone> Node<T, EvaluationContext<'a>> for MinNode<T> {
    fn evaluate(&self, eval_context: &mut EvaluationContext) -> NodeResult<T> {
        let array = self.array_node.evaluate(eval_context)?;
        
        if array.is_empty() {
            return Err(crate::NodeError::EvaluationError("Cannot find min of empty array".to_string()));
        }
        
        // Compare using to_i32() values and return the element with minimum value
        let min_value = array.into_iter().reduce(|a, b| {
            if a.to_i32() <= b.to_i32() {
                a
            } else {
                b
            }
        }).unwrap();
        Ok(min_value)
    }
}

