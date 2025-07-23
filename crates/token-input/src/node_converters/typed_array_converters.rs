// 型情報を伝播させる配列コンバーター

use crate::{StructuredTokenInput, typed_node_converter::{TypedNodeConverter, TypedConverterRegistry, convert_child}};
use crate::type_system::{TypedAst, Type};
use action_system::*;
use action_system::nodes::array::{RandomPickNode, FilterListNode, GenericFilterListNode, MappingNode, CharacterRandomPickNode};
use std::marker::PhantomData;

// Type alias for Node trait with action-system's EvaluationContext
type ActionSystemNode<T> = dyn for<'a> Node<T, EvaluationContext<'a>> + Send + Sync;

/// 型情報を活用するMapコンバーター（ジェネリック）
pub struct TypedMapConverter<TInput, TOutput> 
where
    TInput: Clone + Send + Sync + 'static,
    TOutput: Clone + Send + Sync + 'static,
{
    _phantom: PhantomData<(TInput, TOutput)>,
}

impl<TInput, TOutput> TypedMapConverter<TInput, TOutput>
where
    TInput: Clone + Send + Sync + 'static,
    TOutput: Clone + Send + Sync + 'static,
{
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<TInput, TOutput> TypedNodeConverter<Vec<TOutput>> for TypedMapConverter<TInput, TOutput>
where
    TInput: Clone + Send + Sync + nodes::array::AsCurrentElement + 'static,
    TOutput: Clone + Send + Sync + 'static,
{
    fn can_convert(&self, token: &StructuredTokenInput, expected_type: &Type) -> bool {
        matches!(token, StructuredTokenInput::Map { .. }) && 
        matches!(expected_type, Type::Vec(_))
    }
    
    fn convert(&self, 
               typed_ast: &TypedAst, 
               registry: &dyn TypedConverterRegistry) -> Result<Box<ActionSystemNode<Vec<TOutput>>>, String> {
        if let StructuredTokenInput::Map { .. } = &typed_ast.token {
            // 子要素の型情報を確認
            if !typed_ast.children.contains_key("array") || !typed_ast.children.contains_key("transform") {
                return Err("Map requires array and transform arguments".to_string());
            }
            
            // 配列と変換関数を型情報に基づいて変換
            let array_node = convert_child::<Vec<TInput>>(registry, typed_ast, "array")?;
            let transform_node = convert_child::<TOutput>(registry, typed_ast, "transform")?;
            
            Ok(Box::new(MappingNode::new(array_node, transform_node)))
        } else {
            Err("Expected Map token".to_string())
        }
    }
}

/// 型情報を活用するRandomPickコンバーター（ジェネリック）
pub struct TypedRandomPickConverter<T> 
where
    T: Clone + Send + Sync + 'static,
{
    _phantom: PhantomData<T>,
}

impl<T> TypedRandomPickConverter<T>
where
    T: Clone + Send + Sync + 'static,
{
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<T> TypedNodeConverter<T> for TypedRandomPickConverter<T>
where
    T: Clone + Send + Sync + 'static,
{
    fn can_convert(&self, token: &StructuredTokenInput, _expected_type: &Type) -> bool {
        matches!(token, StructuredTokenInput::RandomPick { .. })
        // expected_typeは配列要素の型と一致するはず
    }
    
    fn convert(&self, 
               typed_ast: &TypedAst, 
               registry: &dyn TypedConverterRegistry) -> Result<Box<ActionSystemNode<T>>, String> {
        if let StructuredTokenInput::RandomPick { .. } = &typed_ast.token {
            // 子要素の型情報を確認
            if !typed_ast.children.contains_key("array") {
                return Err("RandomPick requires an array argument".to_string());
            }
            
            // 配列を型情報に基づいて変換
            let array_node = convert_child::<Vec<T>>(registry, typed_ast, "array")?;
            
            Ok(Box::new(RandomPickNode::new(array_node)))
        } else {
            Err("Expected RandomPick token".to_string())
        }
    }
}

/// 型情報を活用するFilterListコンバーター（Character専用）
pub struct TypedFilterListCharacterConverter;

impl TypedNodeConverter<Vec<Character>> for TypedFilterListCharacterConverter {
    fn can_convert(&self, token: &StructuredTokenInput, expected_type: &Type) -> bool {
        matches!(token, StructuredTokenInput::FilterList { .. }) && 
        matches!(expected_type, Type::Vec(elem) if matches!(elem.as_ref(), Type::Character))
    }
    
    fn convert(&self, 
               typed_ast: &TypedAst, 
               registry: &dyn TypedConverterRegistry) -> Result<Box<ActionSystemNode<Vec<Character>>>, String> {
        if let StructuredTokenInput::FilterList { .. } = &typed_ast.token {
            // 子要素の型情報を確認
            if !typed_ast.children.contains_key("array") || !typed_ast.children.contains_key("condition") {
                return Err("FilterList requires array and condition arguments".to_string());
            }
            
            // 配列と条件を型情報に基づいて変換
            let array_node = convert_child::<Vec<Character>>(registry, typed_ast, "array")?;
            let condition_node = convert_child::<bool>(registry, typed_ast, "condition")?;
            
            Ok(Box::new(FilterListNode::new(array_node, condition_node)))
        } else {
            Err("Expected FilterList token".to_string())
        }
    }
}

/// 型情報を活用するFilterListコンバーター（ジェネリック）
pub struct TypedGenericFilterListConverter<T> 
where
    T: Clone + Send + Sync + 'static,
{
    _phantom: PhantomData<T>,
}

impl<T> TypedGenericFilterListConverter<T>
where
    T: Clone + Send + Sync + 'static,
{
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<T> TypedNodeConverter<Vec<T>> for TypedGenericFilterListConverter<T>
where
    T: Clone + Send + Sync + 'static,
{
    fn can_convert(&self, token: &StructuredTokenInput, expected_type: &Type) -> bool {
        matches!(token, StructuredTokenInput::FilterList { .. }) && 
        matches!(expected_type, Type::Vec(_))
    }
    
    fn convert(&self, 
               typed_ast: &TypedAst, 
               registry: &dyn TypedConverterRegistry) -> Result<Box<ActionSystemNode<Vec<T>>>, String> {
        if let StructuredTokenInput::FilterList { .. } = &typed_ast.token {
            // 子要素の型情報を確認
            if !typed_ast.children.contains_key("array") || !typed_ast.children.contains_key("condition") {
                return Err("FilterList requires array and condition arguments".to_string());
            }
            
            // 配列と条件を型情報に基づいて変換
            let array_node = convert_child::<Vec<T>>(registry, typed_ast, "array")?;
            let condition_node = convert_child::<bool>(registry, typed_ast, "condition")?;
            
            Ok(Box::new(GenericFilterListNode::new(array_node, condition_node)))
        } else {
            Err("Expected FilterList token".to_string())
        }
    }
}

/// 型情報を活用するMaxコンバーター（Numeric trait実装型用）
pub struct TypedMaxConverter<T>
where
    T: Numeric + Clone + Send + Sync + 'static,
{
    _phantom: PhantomData<T>,
}

impl<T> TypedMaxConverter<T>
where
    T: Numeric + Clone + Send + Sync + 'static,
{
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<T> TypedNodeConverter<T> for TypedMaxConverter<T>
where
    T: Numeric + Clone + Send + Sync + 'static,
{
    fn can_convert(&self, token: &StructuredTokenInput, _expected_type: &Type) -> bool {
        matches!(token, StructuredTokenInput::Max { .. } | StructuredTokenInput::NumericMax { .. })
    }
    
    fn convert(&self, 
               typed_ast: &TypedAst, 
               registry: &dyn TypedConverterRegistry) -> Result<Box<ActionSystemNode<T>>, String> {
        if matches!(&typed_ast.token, StructuredTokenInput::Max { .. } | StructuredTokenInput::NumericMax { .. }) {
            // 子要素の型情報を確認
            if !typed_ast.children.contains_key("array") {
                return Err("Max requires an array argument".to_string());
            }
            
            // 配列を型情報に基づいて変換
            let array_node = convert_child::<Vec<T>>(registry, typed_ast, "array")?;
            
            Ok(Box::new(MaxNode::new(array_node)))
        } else {
            Err("Expected Max or NumericMax token".to_string())
        }
    }
}

/// 型情報を活用するMaxコンバーター（Character用）
pub struct TypedMaxCharacterConverter;

impl TypedNodeConverter<Character> for TypedMaxCharacterConverter {
    fn can_convert(&self, token: &StructuredTokenInput, expected_type: &Type) -> bool {
        matches!(token, StructuredTokenInput::Max { .. }) && 
        matches!(expected_type, Type::Character)
    }
    
    fn convert(&self, 
               typed_ast: &TypedAst, 
               registry: &dyn TypedConverterRegistry) -> Result<Box<ActionSystemNode<Character>>, String> {
        if let StructuredTokenInput::Max { .. } = &typed_ast.token {
            // 子要素の型情報を確認
            if !typed_ast.children.contains_key("array") {
                return Err("Max requires an array argument".to_string());
            }
            
            // 配列を型情報に基づいて変換
            let array_node = convert_child::<Vec<Character>>(registry, typed_ast, "array")?;
            
            Ok(Box::new(CharacterRandomPickNode::new(array_node)))
        } else {
            Err("Expected Max token".to_string())
        }
    }
}

/// 型情報を活用するMinコンバーター（Numeric trait実装型用）
pub struct TypedMinConverter<T>
where
    T: Numeric + Clone + Send + Sync + 'static,
{
    _phantom: PhantomData<T>,
}

impl<T> TypedMinConverter<T>
where
    T: Numeric + Clone + Send + Sync + 'static,
{
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<T> TypedNodeConverter<T> for TypedMinConverter<T>
where
    T: Numeric + Clone + Send + Sync + 'static,
{
    fn can_convert(&self, token: &StructuredTokenInput, _expected_type: &Type) -> bool {
        matches!(token, StructuredTokenInput::Min { .. } | StructuredTokenInput::NumericMin { .. })
    }
    
    fn convert(&self, 
               typed_ast: &TypedAst, 
               registry: &dyn TypedConverterRegistry) -> Result<Box<ActionSystemNode<T>>, String> {
        if matches!(&typed_ast.token, StructuredTokenInput::Min { .. } | StructuredTokenInput::NumericMin { .. }) {
            // 子要素の型情報を確認
            if !typed_ast.children.contains_key("array") {
                return Err("Min requires an array argument".to_string());
            }
            
            // 配列を型情報に基づいて変換
            let array_node = convert_child::<Vec<T>>(registry, typed_ast, "array")?;
            
            Ok(Box::new(MinNode::new(array_node)))
        } else {
            Err("Expected Min or NumericMin token".to_string())
        }
    }
}