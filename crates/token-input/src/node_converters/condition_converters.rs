use crate::{StructuredTokenInput, node_converter::{NodeConverter, ConverterRegistry, matches_token}};
use action_system::*;

pub struct TrueOrFalseRandomConverter;

impl NodeConverter<bool> for TrueOrFalseRandomConverter {
    fn can_convert(&self, token: &StructuredTokenInput) -> bool {
        matches_token(token, "TrueOrFalseRandom")
    }
    
    fn convert(&self, _token: &StructuredTokenInput, _registry: &ConverterRegistry) -> Result<Box<dyn Node<bool>>, String> {
        Ok(Box::new(RandomConditionNode))
    }
}

pub struct GreaterThanConverter;

impl NodeConverter<bool> for GreaterThanConverter {
    fn can_convert(&self, token: &StructuredTokenInput) -> bool {
        matches_token(token, "GreaterThan")
    }
    
    fn convert(&self, token: &StructuredTokenInput, registry: &ConverterRegistry) -> Result<Box<dyn Node<bool>>, String> {
        if let StructuredTokenInput::GreaterThan { left, right } = token {
            // Try different type combinations
            // Both i32
            if let (Ok(left_i32), Ok(right_i32)) = (registry.convert::<i32>(left), registry.convert::<i32>(right)) {
                return Ok(Box::new(GreaterThanConditionNode::new(left_i32, right_i32)));
            }
            
            // CharacterHP vs i32
            if let (Ok(left_hp), Ok(right_i32)) = (registry.convert::<CharacterHP>(left), registry.convert::<i32>(right)) {
                return Ok(Box::new(CharacterHpVsValueConditionNode::new(left_hp, right_i32)));
            }
            
            // i32 vs CharacterHP
            if let (Ok(left_i32), Ok(right_hp)) = (registry.convert::<i32>(left), registry.convert::<CharacterHP>(right)) {
                return Ok(Box::new(ValueVsCharacterHpConditionNode::new(left_i32, right_hp)));
            }
            
            // Both CharacterHP
            if let (Ok(left_hp), Ok(right_hp)) = (registry.convert::<CharacterHP>(left), registry.convert::<CharacterHP>(right)) {
                return Ok(Box::new(GreaterThanNode::<CharacterHP>::new(left_hp, right_hp)));
            }
            
            Err("GreaterThan requires numeric types (i32 or CharacterHP)".to_string())
        } else {
            Err("Expected GreaterThan token".to_string())
        }
    }
}

pub struct EqConverter;

impl NodeConverter<bool> for EqConverter {
    fn can_convert(&self, token: &StructuredTokenInput) -> bool {
        matches_token(token, "Eq")
    }
    
    fn convert(&self, token: &StructuredTokenInput, registry: &ConverterRegistry) -> Result<Box<dyn Node<bool>>, String> {
        if let StructuredTokenInput::Eq { left, right } = token {
            // Try TeamSide comparison
            if let (Ok(left_team), Ok(right_team)) = (registry.convert::<TeamSide>(left), registry.convert::<TeamSide>(right)) {
                return Ok(Box::new(TeamSideEqNode::new(left_team, right_team)));
            }
            
            // Add more type comparisons as needed
            
            Err("Eq comparison not implemented for these types".to_string())
        } else {
            Err("Expected Eq token".to_string())
        }
    }
}