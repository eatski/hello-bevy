use crate::{StructuredTokenInput, typed_node_converter::{TypedNodeConverter, TypedConverterRegistry, convert_child}};
use crate::type_system::{TypedAst, Type};
use action_system::*;
use action_system::nodes::condition::CharacterTeamNode;

// Type alias for Node trait with action-system's EvaluationContext
type ActionSystemNode<T> = dyn for<'a> Node<T, EvaluationContext<'a>> + Send + Sync;

/// 型情報を活用するCharacterTeamコンバーター
pub struct TypedCharacterTeamConverter;

impl TypedNodeConverter<TeamSide> for TypedCharacterTeamConverter {
    fn can_convert(&self, token: &StructuredTokenInput, expected_type: &Type) -> bool {
        matches!(token, StructuredTokenInput::CharacterTeam { .. }) && 
        matches!(expected_type, Type::TeamSide)
    }
    
    fn convert(&self, 
               typed_ast: &TypedAst, 
               registry: &dyn TypedConverterRegistry) -> Result<Box<ActionSystemNode<TeamSide>>, String> {
        if let StructuredTokenInput::CharacterTeam { .. } = &typed_ast.token {
            // 子要素の型情報を確認
            let character_ast = typed_ast.children.get("character")
                .ok_or_else(|| "CharacterTeam requires a character argument".to_string())?;
            let character_type = &character_ast.ty;
            
            // 型チェック
            if !matches!(character_type, Type::Character) {
                return Err(format!("CharacterTeam requires Character, got {:?}", character_type));
            }
            
            // 型情報に基づいて変換
            let character_node = convert_child::<Character>(registry, typed_ast, "character")?;
            Ok(Box::new(CharacterTeamNode::new(character_node)))
        } else {
            Err("Expected CharacterTeam token".to_string())
        }
    }
}