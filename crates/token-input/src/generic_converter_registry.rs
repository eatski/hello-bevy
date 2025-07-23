//! ジェネリックコンバーターレジストリ
//!
//! 型の組み合わせごとにコンバーターを登録する必要をなくし、
//! 動的にコンバーターを生成するシステム

use std::any::TypeId;
use std::collections::HashMap;
use action_system::NodeRegistry;
use node_core::Node as ActionSystemNode;
use crate::type_system::{Type, TypedAst};
use crate::typed_node_converter::TypedConverterRegistry;

/// コンバーターファクトリトレイト
pub trait ConverterFactory: Send + Sync {
    /// このファクトリがサポートするトークンタイプ
    fn supported_tokens(&self) -> Vec<&'static str>;
    
    /// 指定された型の組み合わせでコンバーターを作成できるか
    fn can_create(&self, token_type: &str, output_type: &Type) -> bool;
    
    /// コンバーターを作成
    fn create_converter(
        &self,
        token_type: &str,
        output_type: &Type,
        typed_ast: &TypedAst,
        registry: &dyn TypedConverterRegistry,
    ) -> Result<Box<dyn std::any::Any + Send + Sync>, String>;
}

/// ジェネリックコンバーターレジストリ
pub struct GenericConverterRegistry {
    /// コンバーターファクトリのリスト
    factories: Vec<Box<dyn ConverterFactory>>,
    
    /// キャッシュされたコンバーター（パフォーマンス最適化用）
    converter_cache: HashMap<(String, TypeId), Box<dyn std::any::Any + Send + Sync>>,
}

impl GenericConverterRegistry {
    pub fn new() -> Self {
        Self {
            factories: Vec::new(),
            converter_cache: HashMap::new(),
        }
    }
    
    /// ファクトリを登録
    pub fn register_factory(&mut self, factory: Box<dyn ConverterFactory>) {
        self.factories.push(factory);
    }
    
    /// 型情報に基づいてコンバーターを取得または生成
    pub fn get_or_create_converter<T: 'static>(
        &mut self,
        token_type: &str,
        output_type: &Type,
        typed_ast: &TypedAst,
        registry: &dyn TypedConverterRegistry,
    ) -> Result<Box<dyn ActionSystemNode<T, EvaluationContext>>, String> {
        let type_id = TypeId::of::<T>();
        let cache_key = (token_type.to_string(), type_id);
        
        // キャッシュをチェック
        if let Some(converter) = self.converter_cache.get(&cache_key) {
            if let Some(node) = converter.downcast_ref::<Box<dyn ActionSystemNode<T, EvaluationContext>>>() {
                return Ok(node.clone());
            }
        }
        
        // 適切なファクトリを探す
        for factory in &self.factories {
            if factory.can_create(token_type, output_type) {
                let converter = factory.create_converter(token_type, output_type, typed_ast, registry)?;
                
                // 型変換を試みる
                if let Ok(node) = converter.downcast::<Box<dyn ActionSystemNode<T, EvaluationContext>>>() {
                    // キャッシュに保存
                    self.converter_cache.insert(cache_key, Box::new(node.clone()));
                    return Ok(*node);
                }
            }
        }
        
        Err(format!("No converter factory found for token {} with output type {:?}", token_type, output_type))
    }
}

/// 汎用的な配列操作コンバーターファクトリ
pub struct ArrayOperationFactory;

impl ConverterFactory for ArrayOperationFactory {
    fn supported_tokens(&self) -> Vec<&'static str> {
        vec!["RandomPick", "Map", "FilterList", "Max", "Min"]
    }
    
    fn can_create(&self, token_type: &str, _output_type: &Type) -> bool {
        self.supported_tokens().contains(&token_type)
    }
    
    fn create_converter(
        &self,
        token_type: &str,
        output_type: &Type,
        typed_ast: &TypedAst,
        registry: &dyn TypedConverterRegistry,
    ) -> Result<Box<dyn std::any::Any + Send + Sync>, String> {
        match token_type {
            "RandomPick" => {
                // 出力型に基づいて適切なRandomPickNodeを作成
                match output_type {
                    Type::Character => {
                        use crate::typed_node_converter::convert_child;
                        use action_system::{Character, nodes::character::RandomCharacterPickNode};
                        
                        let array_ast = typed_ast.children.get("array")
                            .ok_or("Missing array argument")?;
                        let array_node = convert_child::<Vec<Character>>(registry, array_ast, "array")?;
                        
                        let node: Box<dyn ActionSystemNode<Character, EvaluationContext>> = 
                            Box::new(CharacterRandomPickNode::new(array_node));
                        Ok(Box::new(node))
                    }
                    Type::I32 => {
                        use crate::typed_node_converter::convert_child;
                        use action_system::nodes::array::RandomPickNode;
                        
                        let array_ast = typed_ast.children.get("array")
                            .ok_or("Missing array argument")?;
                        let array_node = convert_child::<Vec<i32>>(registry, array_ast, "array")?;
                        
                        let node: Box<dyn ActionSystemNode<i32, EvaluationContext>> = 
                            Box::new(RandomPickNode::new(array_node));
                        Ok(Box::new(node))
                    }
                    _ => Err(format!("RandomPick not supported for type {:?}", output_type))
                }
            }
            // 他のトークンタイプも同様に実装
            _ => Err(format!("Unsupported token type: {}", token_type))
        }
    }
}

/// 基本的な値コンバーターファクトリ
pub struct ValueConverterFactory;

impl ConverterFactory for ValueConverterFactory {
    fn supported_tokens(&self) -> Vec<&'static str> {
        vec!["Number", "ActingCharacter", "Element"]
    }
    
    fn can_create(&self, token_type: &str, _output_type: &Type) -> bool {
        self.supported_tokens().contains(&token_type)
    }
    
    fn create_converter(
        &self,
        token_type: &str,
        _output_type: &Type,
        typed_ast: &TypedAst,
        _registry: &dyn TypedConverterRegistry,
    ) -> Result<Box<dyn std::any::Any + Send + Sync>, String> {
        match token_type {
            "Number" => {
                use action_system::nodes::value::NumberNode;
                
                // StructuredTokenInputから値を取得
                if let crate::structured_token::StructuredTokenInput::Number { value } = &typed_ast.token {
                    let node: Box<dyn ActionSystemNode<i32, EvaluationContext>> = Box::new(NumberNode::new(*value));
                    Ok(Box::new(node))
                } else {
                    Err("Invalid token type for Number".to_string())
                }
            }
            "ActingCharacter" => {
                use action_system::{Character, nodes::character::ActingCharacterNode};
                
                let node: Box<dyn ActionSystemNode<Character, EvaluationContext>> = Box::new(ActingCharacterNode);
                Ok(Box::new(node))
            }
            _ => Err(format!("Unsupported token type: {}", token_type))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_generic_converter_registry() {
        let mut registry = GenericConverterRegistry::new();
        registry.register_factory(Box::new(ArrayOperationFactory));
        registry.register_factory(Box::new(ValueConverterFactory));
        
        // ファクトリが登録されていることを確認
        assert_eq!(registry.factories.len(), 2);
    }
}