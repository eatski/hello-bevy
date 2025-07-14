use crate::{StructuredTokenInput, node_converter::{NodeConverter, ConverterRegistry, matches_token}};
use action_system::*;

pub struct NumberConverter;

impl NodeConverter<i32> for NumberConverter {
    fn can_convert(&self, token: &StructuredTokenInput) -> bool {
        matches_token(token, "Number")
    }
    
    fn convert(&self, token: &StructuredTokenInput, _registry: &ConverterRegistry) -> Result<Box<dyn Node<i32>>, String> {
        if let StructuredTokenInput::Number { value } = token {
            Ok(Box::new(ConstantValueNode::new(*value)))
        } else {
            Err("Expected Number token".to_string())
        }
    }
}