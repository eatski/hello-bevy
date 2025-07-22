use crate::{StructuredTokenInput, node_converter::{NodeConverter, ConverterRegistry, matches_token}};
use action_system::*;

// Type alias for Node trait with action-system's EvaluationContext
type ActionSystemNode<T> = dyn for<'a> Node<T, EvaluationContext<'a>> + Send + Sync;

pub struct ActingCharacterConverter;

impl NodeConverter<Character> for ActingCharacterConverter {
    fn can_convert(&self, token: &StructuredTokenInput) -> bool {
        matches_token(token, "ActingCharacter")
    }
    
    fn convert(&self, _token: &StructuredTokenInput, _registry: &ConverterRegistry) -> Result<Box<ActionSystemNode<Character>>, String> {
        Ok(Box::new(ActingCharacterNode))
    }
}

pub struct ElementConverter;

impl NodeConverter<Character> for ElementConverter {
    fn can_convert(&self, token: &StructuredTokenInput) -> bool {
        matches_token(token, "Element")
    }
    
    fn convert(&self, _token: &StructuredTokenInput, _registry: &ConverterRegistry) -> Result<Box<ActionSystemNode<Character>>, String> {
        Ok(Box::new(ElementNode::new()))
    }
}

pub struct CharacterHpToCharacterConverter;

impl NodeConverter<Character> for CharacterHpToCharacterConverter {
    fn can_convert(&self, token: &StructuredTokenInput) -> bool {
        matches_token(token, "CharacterHpToCharacter")
    }
    
    fn convert(&self, token: &StructuredTokenInput, registry: &ConverterRegistry) -> Result<Box<ActionSystemNode<Character>>, String> {
        if let StructuredTokenInput::CharacterHpToCharacter { character_hp } = token {
            let hp_node = registry.convert::<CharacterHP>(character_hp)?;
            Ok(Box::new(CharacterHpToCharacterNode::new(hp_node)))
        } else {
            Err("Expected CharacterHpToCharacter token".to_string())
        }
    }
}

pub struct CharacterToHpConverter;

impl NodeConverter<CharacterHP> for CharacterToHpConverter {
    fn can_convert(&self, token: &StructuredTokenInput) -> bool {
        matches_token(token, "CharacterToHp")
    }
    
    fn convert(&self, token: &StructuredTokenInput, registry: &ConverterRegistry) -> Result<Box<ActionSystemNode<CharacterHP>>, String> {
        if let StructuredTokenInput::CharacterToHp { character } = token {
            let char_node = registry.convert::<Character>(character)?;
            Ok(Box::new(CharacterToHpNode::new(char_node)))
        } else {
            Err("Expected CharacterToHp token".to_string())
        }
    }
}