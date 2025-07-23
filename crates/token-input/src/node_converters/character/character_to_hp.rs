use crate::{StructuredTokenInput, typed_node_converter::{TypedNodeConverter, TypedConverterRegistry, convert_child}};
use crate::type_system::{TypedAst, Type};
use action_system::*;

// Type alias for Node trait with action-system's EvaluationContext
type ActionSystemNode<T> = dyn for<'a> Node<T, EvaluationContext<'a>> + Send + Sync;

/// Character -> CharacterHP 変換コンバーター
pub struct TypedCharacterToHpConverter;

impl TypedNodeConverter<CharacterHP> for TypedCharacterToHpConverter {
    fn can_convert(&self, token: &StructuredTokenInput, expected_type: &Type) -> bool {
        matches!(token, StructuredTokenInput::CharacterToHp { .. }) && 
        matches!(expected_type, Type::CharacterHP)
    }
    
    fn convert(&self, 
               typed_ast: &TypedAst, 
               registry: &dyn TypedConverterRegistry) -> Result<Box<ActionSystemNode<CharacterHP>>, String> {
        if let StructuredTokenInput::CharacterToHp { .. } = &typed_ast.token {
            // 子要素の型情報を確認
            let character_ast = typed_ast.children.get("character")
                .ok_or_else(|| "CharacterToHp requires a character".to_string())?;
            let character_type = &character_ast.ty;
            
            // 型チェック
            if !matches!(character_type, Type::Character) {
                return Err(format!("CharacterToHp character must be Character, got {:?}", character_type));
            }
            
            // 型情報に基づいて変換
            let character_node = convert_child::<Character>(registry, typed_ast, "character")?;
            Ok(Box::new(CharacterToHpNode::new(character_node)))
        } else {
            Err("Expected CharacterToHp token".to_string())
        }
    }
}