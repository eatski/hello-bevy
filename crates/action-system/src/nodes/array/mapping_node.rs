// MappingNode - applies a transformation function to each element of an array
// Truly generic implementation that works with any types

use crate::core::NodeResult;
use crate::nodes::unified_node::{CoreNode as Node, BoxedNode};
use crate::nodes::evaluation_context::EvaluationContext;
use crate::nodes::unknown_value::UnknownValue;
use crate::Character;
use crate::core::character_hp::CharacterHP;
use crate::TeamSide;
use std::any::Any;

/// Trait for types that can be converted to UnknownValue for array operations
pub trait AsUnknownValue: Any + Send + Sync {
    fn as_unknown_value(&self) -> UnknownValue;
}

impl AsUnknownValue for Character {
    fn as_unknown_value(&self) -> UnknownValue {
        UnknownValue::Character(self.clone())
    }
}

impl AsUnknownValue for i32 {
    fn as_unknown_value(&self) -> UnknownValue {
        UnknownValue::Value(*self)
    }
}

impl AsUnknownValue for CharacterHP {
    fn as_unknown_value(&self) -> UnknownValue {
        UnknownValue::CharacterHP(self.clone())
    }
}

impl AsUnknownValue for TeamSide {
    fn as_unknown_value(&self) -> UnknownValue {
        UnknownValue::TeamSide(*self)
    }
}

// For now, we'll implement AsUnknownValue only for known types
// Future enhancement: Add Generic(Box<dyn Any>) variant to UnknownValue

/// Truly generic MappingNode that can map between any types
pub struct MappingNode<TInput, TOutput> 
where
    TInput: Clone + Send + Sync + 'static,
    TOutput: Clone + Send + Sync + 'static,
{
    /// The array node to map over
    array_node: BoxedNode<Vec<TInput>>,
    /// The transformation function to apply to each element
    transform_node: BoxedNode<TOutput>,
}

impl<TInput, TOutput> MappingNode<TInput, TOutput>
where
    TInput: Clone + Send + Sync + 'static,
    TOutput: Clone + Send + Sync + 'static,
{
    pub fn new(
        array_node: BoxedNode<Vec<TInput>>,
        transform_node: BoxedNode<TOutput>,
    ) -> Self {
        Self {
            array_node,
            transform_node,
        }
    }
}

impl<'a, TInput, TOutput> Node<Vec<TOutput>, EvaluationContext<'a>> for MappingNode<TInput, TOutput>
where
    TInput: Clone + Send + Sync + AsUnknownValue + 'static,
    TOutput: Clone + Send + Sync + 'static,
{
    fn evaluate(&self, eval_context: &mut EvaluationContext) -> NodeResult<Vec<TOutput>> {
        // Get the input array
        let input_array = self.array_node.evaluate(eval_context)?;
        
        let mut output_array = Vec::new();
        
        // Apply the transformation to each element
        for element in input_array {
            // Create an evaluation context with the current element
            let unknown_value = element.as_unknown_value();
            let mut element_eval_context = eval_context.with_current_element_from_context(unknown_value);
            
            // Apply the transformation function
            let transformed_element = self.transform_node.evaluate(&mut element_eval_context)?;
            
            output_array.push(transformed_element);
        }
        
        Ok(output_array)
    }
}