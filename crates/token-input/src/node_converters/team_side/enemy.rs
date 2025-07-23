use crate::{StructuredTokenInput, typed_node_converter::{TypedNodeConverter, TypedConverterRegistry}};
use crate::type_system::{TypedAst, Type};
use action_system::*;

// Type alias for Node trait with action-system's EvaluationContext
type ActionSystemNode<T> = dyn for<'a> Node<T, EvaluationContext<'a>> + Send + Sync;

// Simple constant node for TeamSide
#[derive(Debug)]
struct TeamSideConstantNode {
    value: TeamSide,
}

impl TeamSideConstantNode {
    fn new(value: TeamSide) -> Self {
        Self { value }
    }
}

impl<'a> Node<TeamSide, EvaluationContext<'a>> for TeamSideConstantNode {
    fn evaluate(&self, _eval_context: &mut EvaluationContext<'a>) -> NodeResult<TeamSide> {
        Ok(self.value)
    }
}

/// 型情報を活用するEnemyコンバーター
pub struct TypedEnemyConverter;

impl TypedNodeConverter<TeamSide> for TypedEnemyConverter {
    fn can_convert(&self, token: &StructuredTokenInput, expected_type: &Type) -> bool {
        matches!(token, StructuredTokenInput::Enemy) && 
        matches!(expected_type, Type::TeamSide)
    }
    
    fn convert(&self, 
               _typed_ast: &TypedAst, 
               _registry: &dyn TypedConverterRegistry) -> Result<Box<ActionSystemNode<TeamSide>>, String> {
        Ok(Box::new(TeamSideConstantNode::new(TeamSide::Enemy)))
    }
}