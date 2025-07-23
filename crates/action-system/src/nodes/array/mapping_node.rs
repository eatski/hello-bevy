// MappingNode - applies a transformation function to each element of an array
// Truly generic implementation that works with any types

use crate::core::NodeResult;
use crate::nodes::unified_node::{CoreNode as Node, BoxedNode};
use crate::nodes::evaluation_context::{EvaluationContext, CurrentElement};
use crate::Character;
use crate::core::character_hp::CharacterHP;
use crate::TeamSide;
use std::any::Any;

/// Trait for types that can be used as current element in evaluation context
pub trait AsCurrentElement: Any + Send + Sync {
    fn as_current_element(&self) -> CurrentElement;
}

impl AsCurrentElement for Character {
    fn as_current_element(&self) -> CurrentElement {
        CurrentElement::Character(self.clone())
    }
}

impl AsCurrentElement for i32 {
    fn as_current_element(&self) -> CurrentElement {
        CurrentElement::Value(*self)
    }
}

impl AsCurrentElement for CharacterHP {
    fn as_current_element(&self) -> CurrentElement {
        CurrentElement::CharacterHP(self.clone())
    }
}

impl AsCurrentElement for TeamSide {
    fn as_current_element(&self) -> CurrentElement {
        CurrentElement::TeamSide(*self)
    }
}

// For now, we'll implement AsCurrentElement only for known types
// Future enhancement: Add Generic(Box<dyn Any>) variant to CurrentElement

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
    TInput: Clone + Send + Sync + AsCurrentElement + 'static,
    TOutput: Clone + Send + Sync + 'static,
{
    fn evaluate(&self, eval_context: &mut EvaluationContext) -> NodeResult<Vec<TOutput>> {
        // Get the input array
        let input_array = self.array_node.evaluate(eval_context)?;
        
        let mut output_array = Vec::new();
        
        // Apply the transformation to each element
        for element in input_array {
            // Create an evaluation context with the current element
            let current_element = element.as_current_element();
            let mut element_eval_context = eval_context.with_current_element_from_context(current_element);
            
            // Apply the transformation function
            let transformed_element = self.transform_node.evaluate(&mut element_eval_context)?;
            
            output_array.push(transformed_element);
        }
        
        Ok(output_array)
    }
}