use crate::{StructuredTokenInput, typed_node_converter::{TypedNodeConverter, TypedConverterRegistry, convert_child}};
use crate::type_system::{TypedAst, Type};
use action_system::*;

// Type alias for Node trait with action-system's EvaluationContext
type ActionSystemNode<T> = dyn for<'a> Node<T, EvaluationContext<'a>> + Send + Sync;

/// CharacterHP -> Character 変換コンバーター
pub struct TypedCharacterHpToCharacterConverter;

impl TypedCharacterHpToCharacterConverter {
    /// Numeric型の実際の型を推論
    fn infer_numeric_type(ast: &TypedAst) -> Type {
        match &ast.token {
            StructuredTokenInput::NumericMax { .. } | StructuredTokenInput::NumericMin { .. } => {
                // Check the array element type
                if let Some(array_ast) = ast.children.get("array") {
                    if let Type::Vec(elem_type) = &array_ast.ty {
                        return *elem_type.clone();
                    }
                }
                Type::CharacterHP // default for Max/Min
            }
            _ => Type::CharacterHP // default
        }
    }
}

impl TypedNodeConverter<Character> for TypedCharacterHpToCharacterConverter {
    fn can_convert(&self, token: &StructuredTokenInput, expected_type: &Type) -> bool {
        matches!(token, StructuredTokenInput::CharacterHpToCharacter { .. }) && 
        matches!(expected_type, Type::Character)
    }
    
    fn convert(&self, 
               typed_ast: &TypedAst, 
               registry: &dyn TypedConverterRegistry) -> Result<Box<ActionSystemNode<Character>>, String> {
        if let StructuredTokenInput::CharacterHpToCharacter { .. } = &typed_ast.token {
            // 子要素の型情報を確認
            let character_hp_ast = typed_ast.children.get("character_hp")
                .ok_or_else(|| "CharacterHpToCharacter requires a character_hp".to_string())?;
            let character_hp_type = &character_hp_ast.ty;
            
            // Numeric型の実際の型を推論
            let actual_type = if matches!(character_hp_type, Type::Numeric) {
                Self::infer_numeric_type(character_hp_ast)
            } else {
                character_hp_type.clone()
            };
            
            // 型チェック
            if !matches!(actual_type, Type::CharacterHP) {
                return Err(format!("CharacterHpToCharacter character_hp must be CharacterHP, got {:?}", actual_type));
            }
            
            // 型情報に基づいて変換
            let character_hp_node = convert_child::<CharacterHP>(registry, typed_ast, "character_hp")?;
            Ok(Box::new(CharacterHpToCharacterNode::new(character_hp_node)))
        } else {
            Err("Expected CharacterHpToCharacter token".to_string())
        }
    }
}