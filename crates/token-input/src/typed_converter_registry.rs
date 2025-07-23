// 型情報を伝播させるコンバーターレジストリの実装

use std::any::{Any, TypeId};
use std::collections::HashMap;
use crate::typed_node_converter::{TypedNodeConverter, TypedConverterRegistry, ErasedNode};
use crate::type_system::{TypedAst, Type};
use crate::StructuredTokenInput;
use action_system::*;

// Type alias for Node trait with action-system's EvaluationContext
type ActionSystemNode<T> = dyn for<'a> Node<T, EvaluationContext<'a>> + Send + Sync;

/// 型情報を伝播させるコンバーターレジストリの実装
pub struct TypedConverterRegistryImpl {
    /// 型IDごとのコンバーターリスト
    converters: HashMap<TypeId, Vec<Box<dyn Any + Send + Sync>>>,
}

impl TypedConverterRegistryImpl {
    pub fn new() -> Self {
        let mut registry = Self {
            converters: HashMap::new(),
        };
        
        // デフォルトコンバーターを登録
        registry.register_default_converters();
        registry
    }
    
    /// コンバーターを登録
    pub fn register<T: Any + 'static>(&mut self, converter: Box<dyn TypedNodeConverter<T>>) {
        let type_id = TypeId::of::<T>();
        self.converters.entry(type_id)
            .or_insert_with(Vec::new)
            .push(Box::new(converter));
    }
    
    /// デフォルトコンバーターを登録
    fn register_default_converters(&mut self) {
        use crate::node_converters::{
            TypedStrikeActionConverter, TypedHealActionConverter, TypedCheckActionConverter,
            TypedGreaterThanConverter, TypedEqConverter, TypedTrueOrFalseRandomConverter,
            TypedMapConverter, TypedRandomPickConverter, TypedFilterListCharacterConverter,
            TypedGenericFilterListConverter, TypedMaxConverter, TypedMinConverter, TypedMaxCharacterConverter,
            TypedNumberConverter, TypedActingCharacterConverter, TypedElementConverter,
            TypedCharacterToHpConverter, TypedCharacterHpToCharacterConverter,
            TypedAllCharactersConverter, TypedTeamMembersConverter, TypedAllTeamSidesConverter,
            TypedEnemyConverter, TypedHeroConverter, TypedCharacterTeamConverter
        };
        
        // アクションコンバーター
        self.register(Box::new(TypedStrikeActionConverter));
        self.register(Box::new(TypedHealActionConverter));
        self.register(Box::new(TypedCheckActionConverter));
        
        // 条件コンバーター
        self.register(Box::new(TypedGreaterThanConverter));
        self.register(Box::new(TypedEqConverter));
        self.register(Box::new(TypedTrueOrFalseRandomConverter));
        
        // 配列コンバーター（各型の組み合わせ）
        // Map converters
        self.register(Box::new(TypedMapConverter::<Character, CharacterHP>::new()));
        self.register(Box::new(TypedMapConverter::<Character, i32>::new()));
        self.register(Box::new(TypedMapConverter::<CharacterHP, Character>::new()));
        self.register(Box::new(TypedMapConverter::<CharacterHP, i32>::new()));
        
        // RandomPick converters
        self.register(Box::new(TypedRandomPickConverter::<Character>::new()));
        self.register(Box::new(TypedRandomPickConverter::<i32>::new()));
        self.register(Box::new(TypedRandomPickConverter::<CharacterHP>::new()));
        self.register(Box::new(TypedRandomPickConverter::<TeamSide>::new()));
        
        // FilterList converters
        self.register(Box::new(TypedFilterListCharacterConverter)); // Character専用（Element context必要）
        self.register(Box::new(TypedGenericFilterListConverter::<i32>::new()));
        self.register(Box::new(TypedGenericFilterListConverter::<CharacterHP>::new()));
        self.register(Box::new(TypedGenericFilterListConverter::<TeamSide>::new()));
        
        // Max/Min converters
        self.register(Box::new(TypedMaxConverter::<i32>::new()));
        self.register(Box::new(TypedMaxConverter::<CharacterHP>::new()));
        self.register(Box::new(TypedMinConverter::<i32>::new()));
        self.register(Box::new(TypedMinConverter::<CharacterHP>::new()));
        self.register(Box::new(TypedMaxCharacterConverter)); // Max for Character type
        // Note: Character doesn't implement Numeric trait
        
        // 値コンバーター
        self.register(Box::new(TypedNumberConverter));
        
        // キャラクターコンバーター
        self.register(Box::new(TypedActingCharacterConverter));
        self.register(Box::new(TypedElementConverter));
        self.register(Box::new(TypedCharacterToHpConverter));
        self.register(Box::new(TypedCharacterHpToCharacterConverter));
        
        // 基本配列コンバーター
        self.register(Box::new(TypedAllCharactersConverter));
        self.register(Box::new(TypedTeamMembersConverter));
        self.register(Box::new(TypedAllTeamSidesConverter));
        
        // TeamSideコンバーター
        self.register(Box::new(TypedEnemyConverter));
        self.register(Box::new(TypedHeroConverter));
        self.register(Box::new(TypedCharacterTeamConverter));
    }
}

impl TypedConverterRegistry for TypedConverterRegistryImpl {
    fn convert_typed_erased(&self, typed_ast: &TypedAst, target_type_id: TypeId) -> Result<ErasedNode, String> {
        // Special handling for Numeric types
        if typed_ast.ty == Type::Numeric {
            // If the TypedAst has Numeric type, we need to infer the actual type from context
            // For NumericMax/NumericMin, check the array element type
            match &typed_ast.token {
                StructuredTokenInput::NumericMax { .. } | StructuredTokenInput::NumericMin { .. } => {
                    if let Some(array_ast) = typed_ast.children.get("array") {
                        if let Type::Vec(elem_type) = &array_ast.ty {
                            match elem_type.as_ref() {
                                Type::CharacterHP => {
                                    if target_type_id == TypeId::of::<CharacterHP>() {
                                        return self.convert_typed_internal::<CharacterHP>(typed_ast)
                                            .map(|node| Box::new(node) as ErasedNode);
                                    }
                                }
                                Type::I32 => {
                                    if target_type_id == TypeId::of::<i32>() {
                                        return self.convert_typed_internal::<i32>(typed_ast)
                                            .map(|node| Box::new(node) as ErasedNode);
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        
        // i32
        if target_type_id == TypeId::of::<i32>() {
            return self.convert_typed_internal::<i32>(typed_ast)
                .map(|node| Box::new(node) as ErasedNode);
        }
        // bool
        if target_type_id == TypeId::of::<bool>() {
            return self.convert_typed_internal::<bool>(typed_ast)
                .map(|node| Box::new(node) as ErasedNode);
        }
        // Character
        if target_type_id == TypeId::of::<Character>() {
            return self.convert_typed_internal::<Character>(typed_ast)
                .map(|node| Box::new(node) as ErasedNode);
        }
        // CharacterHP
        if target_type_id == TypeId::of::<CharacterHP>() {
            return self.convert_typed_internal::<CharacterHP>(typed_ast)
                .map(|node| Box::new(node) as ErasedNode);
        }
        // TeamSide
        if target_type_id == TypeId::of::<TeamSide>() {
            return self.convert_typed_internal::<TeamSide>(typed_ast)
                .map(|node| Box::new(node) as ErasedNode);
        }
        // Box<dyn Action>
        if target_type_id == TypeId::of::<Box<dyn Action>>() {
            return self.convert_typed_internal::<Box<dyn Action>>(typed_ast)
                .map(|node| Box::new(node) as ErasedNode);
        }
        // Vec<Character>
        if target_type_id == TypeId::of::<Vec<Character>>() {
            return self.convert_typed_internal::<Vec<Character>>(typed_ast)
                .map(|node| Box::new(node) as ErasedNode);
        }
        // Vec<CharacterHP>
        if target_type_id == TypeId::of::<Vec<CharacterHP>>() {
            return self.convert_typed_internal::<Vec<CharacterHP>>(typed_ast)
                .map(|node| Box::new(node) as ErasedNode);
        }
        // Vec<i32>
        if target_type_id == TypeId::of::<Vec<i32>>() {
            return self.convert_typed_internal::<Vec<i32>>(typed_ast)
                .map(|node| Box::new(node) as ErasedNode);
        }
        // Vec<TeamSide>
        if target_type_id == TypeId::of::<Vec<TeamSide>>() {
            return self.convert_typed_internal::<Vec<TeamSide>>(typed_ast)
                .map(|node| Box::new(node) as ErasedNode);
        }
        
        Err(format!("No converter for type_id {:?}", target_type_id))
    }
    
    fn convert_child_erased(&self, 
                           typed_ast: &TypedAst, 
                           child_name: &str,
                           target_type_id: TypeId) -> Result<ErasedNode, String> {
        if let Some(child) = typed_ast.children.get(child_name) {
            self.convert_typed_erased(child, target_type_id)
        } else {
            Err(format!("Child '{}' not found in TypedAst", child_name))
        }
    }
}

impl TypedConverterRegistryImpl {
    fn convert_typed_internal<T: Any + 'static>(&self, typed_ast: &TypedAst) -> Result<Box<ActionSystemNode<T>>, String> {
        let type_id = TypeId::of::<T>();
        
        if let Some(converters) = self.converters.get(&type_id) {
            for converter_any in converters {
                if let Some(converter) = converter_any.downcast_ref::<Box<dyn TypedNodeConverter<T>>>() {
                    if converter.can_convert(&typed_ast.token, &typed_ast.ty) {
                        return converter.convert(typed_ast, self);
                    }
                }
            }
        }
        
        Err(format!("No typed converter found for token {:?} with type {:?} to {}", 
                    typed_ast.token, typed_ast.ty, std::any::type_name::<T>()))
    }
}

/// 型情報に基づいて動的にコンバーターを生成するファクトリー
pub struct DynamicTypedConverterFactory;

impl DynamicTypedConverterFactory {
    /// 型情報に基づいてコンバーターを生成
    pub fn create_converter(token: &StructuredTokenInput, expected_type: &Type) -> Option<Box<dyn Any>> {
        match token {
            StructuredTokenInput::RandomPick { .. } => {
                Self::create_random_pick_converter(expected_type)
            }
            StructuredTokenInput::FilterList { .. } => {
                Self::create_filter_list_converter(expected_type)
            }
            StructuredTokenInput::Map { .. } => {
                Self::create_map_converter(expected_type)
            }
            StructuredTokenInput::Max { .. } | StructuredTokenInput::NumericMax { .. } => {
                Self::create_max_converter(expected_type)
            }
            StructuredTokenInput::Min { .. } | StructuredTokenInput::NumericMin { .. } => {
                Self::create_min_converter(expected_type)
            }
            _ => None,
        }
    }
    
    fn create_random_pick_converter(_output_type: &Type) -> Option<Box<dyn Any>> {
        // TODO: 実装
        None
    }
    
    fn create_filter_list_converter(_output_type: &Type) -> Option<Box<dyn Any>> {
        // TODO: 実装
        None
    }
    
    fn create_map_converter(_output_type: &Type) -> Option<Box<dyn Any>> {
        // TODO: 実装
        None
    }
    
    fn create_max_converter(_output_type: &Type) -> Option<Box<dyn Any>> {
        // TODO: 実装
        None
    }
    
    fn create_min_converter(_output_type: &Type) -> Option<Box<dyn Any>> {
        // TODO: 実装
        None
    }
}