use crate::{StructuredTokenInput, node_converter::{NodeConverter, ConverterRegistry, matches_token}};
use action_system::*;

// Type alias for Node trait with action-system's EvaluationContext
type ActionSystemNode<T> = dyn for<'a> Node<T, EvaluationContext<'a>> + Send + Sync;

pub struct EnemyConverter;

impl NodeConverter<TeamSide> for EnemyConverter {
    fn can_convert(&self, token: &StructuredTokenInput) -> bool {
        matches_token(token, "Enemy")
    }
    
    fn convert(&self, _token: &StructuredTokenInput, _registry: &ConverterRegistry) -> Result<Box<ActionSystemNode<TeamSide>>, String> {
        Ok(Box::new(EnemyNode))
    }
}

pub struct HeroConverter;

impl NodeConverter<TeamSide> for HeroConverter {
    fn can_convert(&self, token: &StructuredTokenInput) -> bool {
        matches_token(token, "Hero")
    }
    
    fn convert(&self, _token: &StructuredTokenInput, _registry: &ConverterRegistry) -> Result<Box<ActionSystemNode<TeamSide>>, String> {
        Ok(Box::new(HeroNode))
    }
}

pub struct CharacterTeamConverter;

impl NodeConverter<TeamSide> for CharacterTeamConverter {
    fn can_convert(&self, token: &StructuredTokenInput) -> bool {
        matches_token(token, "CharacterTeam")
    }
    
    fn convert(&self, token: &StructuredTokenInput, registry: &ConverterRegistry) -> Result<Box<ActionSystemNode<TeamSide>>, String> {
        if let StructuredTokenInput::CharacterTeam { character } = token {
            let char_node = registry.convert::<Character>(character)?;
            Ok(Box::new(CharacterTeamNode::new(char_node)))
        } else {
            Err("Expected CharacterTeam token".to_string())
        }
    }
}