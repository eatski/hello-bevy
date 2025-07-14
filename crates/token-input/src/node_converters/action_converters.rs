use crate::{StructuredTokenInput, node_converter::{NodeConverter, ConverterRegistry, matches_token}};
use action_system::*;

pub struct StrikeActionConverter;

impl NodeConverter<Box<dyn Action>> for StrikeActionConverter {
    fn can_convert(&self, token: &StructuredTokenInput) -> bool {
        matches_token(token, "Strike")
    }
    
    fn convert(&self, token: &StructuredTokenInput, registry: &ConverterRegistry) -> Result<Box<dyn Node<Box<dyn Action>>>, String> {
        if let StructuredTokenInput::Strike { target } = token {
            let target_node = registry.convert::<Character>(target)?;
            Ok(Box::new(StrikeActionNode::new(target_node)))
        } else {
            Err("Expected Strike token".to_string())
        }
    }
}

pub struct HealActionConverter;

impl NodeConverter<Box<dyn Action>> for HealActionConverter {
    fn can_convert(&self, token: &StructuredTokenInput) -> bool {
        matches_token(token, "Heal")
    }
    
    fn convert(&self, token: &StructuredTokenInput, registry: &ConverterRegistry) -> Result<Box<dyn Node<Box<dyn Action>>>, String> {
        if let StructuredTokenInput::Heal { target } = token {
            let target_node = registry.convert::<Character>(target)?;
            Ok(Box::new(HealActionNode::new(target_node)))
        } else {
            Err("Expected Heal token".to_string())
        }
    }
}

pub struct CheckActionConverter;

impl NodeConverter<Box<dyn Action>> for CheckActionConverter {
    fn can_convert(&self, token: &StructuredTokenInput) -> bool {
        matches_token(token, "Check")
    }
    
    fn convert(&self, token: &StructuredTokenInput, registry: &ConverterRegistry) -> Result<Box<dyn Node<Box<dyn Action>>>, String> {
        if let StructuredTokenInput::Check { condition, then_action } = token {
            let condition_node = registry.convert::<bool>(condition)?;
            let action_node = registry.convert::<Box<dyn Action>>(then_action)?;
            Ok(Box::new(ConditionCheckNode::new(condition_node, action_node)))
        } else {
            Err("Expected Check token".to_string())
        }
    }
}