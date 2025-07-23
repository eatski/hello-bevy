// 型情報を伝播させるキャラクターコンバーター

use crate::{StructuredTokenInput, typed_node_converter::{TypedNodeConverter, TypedConverterRegistry, convert_child}};
use crate::type_system::{TypedAst, Type};
use action_system::*;

// Type alias for Node trait with action-system's EvaluationContext
type ActionSystemNode<T> = dyn for<'a> Node<T, EvaluationContext<'a>> + Send + Sync;

/// 型情報を活用するActingCharacterコンバーター
pub struct TypedActingCharacterConverter;

impl TypedNodeConverter<Character> for TypedActingCharacterConverter {
    fn can_convert(&self, token: &StructuredTokenInput, expected_type: &Type) -> bool {
        matches!(token, StructuredTokenInput::ActingCharacter) && 
        matches!(expected_type, Type::Character)
    }
    
    fn convert(&self, 
               _typed_ast: &TypedAst, 
               _registry: &dyn TypedConverterRegistry) -> Result<Box<ActionSystemNode<Character>>, String> {
        Ok(Box::new(ActingCharacterNode))
    }
}

/// 型情報を活用するElementコンバーター
pub struct TypedElementConverter;

impl TypedNodeConverter<Character> for TypedElementConverter {
    fn can_convert(&self, token: &StructuredTokenInput, expected_type: &Type) -> bool {
        matches!(token, StructuredTokenInput::Element) && 
        matches!(expected_type, Type::Character)
    }
    
    fn convert(&self, 
               _typed_ast: &TypedAst, 
               _registry: &dyn TypedConverterRegistry) -> Result<Box<ActionSystemNode<Character>>, String> {
        Ok(Box::new(ElementNode))
    }
}

/// 型情報を活用するCharacterToHpコンバーター
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
                .ok_or_else(|| "CharacterToHp requires a character argument".to_string())?;
            let character_type = &character_ast.ty;
            
            // 型チェック
            if !matches!(character_type, Type::Character) {
                return Err(format!("CharacterToHp requires Character, got {:?}", character_type));
            }
            
            // 型情報に基づいて変換
            let character_node = convert_child::<Character>(registry, typed_ast, "character")?;
            Ok(Box::new(CharacterToHpNode::new(character_node)))
        } else {
            Err("Expected CharacterToHp token".to_string())
        }
    }
}

/// 型情報を活用するCharacterHpToCharacterコンバーター
pub struct TypedCharacterHpToCharacterConverter;

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
            let hp_ast = typed_ast.children.get("character_hp")
                .ok_or_else(|| "HpCharacter requires a character_hp argument".to_string())?;
            let hp_type = &hp_ast.ty;
            
            // 型チェック
            if !matches!(hp_type, Type::CharacterHP | Type::Numeric) {
                return Err(format!("HpCharacter requires CharacterHP, got {:?}", hp_type));
            }
            
            // 型情報に基づいて変換
            let hp_node = convert_child::<CharacterHP>(registry, typed_ast, "character_hp")?;
            Ok(Box::new(CharacterHpToCharacterNode::new(hp_node)))
        } else {
            Err("Expected HpCharacter token".to_string())
        }
    }
}