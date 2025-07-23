use crate::{StructuredTokenInput, typed_node_converter::{TypedNodeConverter, TypedConverterRegistry, convert_child}};
use crate::type_system::{TypedAst, Type};
use action_system::*;
use action_system::nodes::array::{RandomPickNode, CharacterRandomPickNode};
use std::marker::PhantomData;

// Type alias for Node trait with action-system's EvaluationContext
type ActionSystemNode<T> = dyn for<'a> Node<T, EvaluationContext<'a>> + Send + Sync;

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
    }
    
    fn convert(&self, 
               typed_ast: &TypedAst, 
               registry: &dyn TypedConverterRegistry) -> Result<Box<ActionSystemNode<T>>, String> {
        if let StructuredTokenInput::RandomPick { .. } = &typed_ast.token {
            let array_ast = typed_ast.children.get("array")
                .ok_or_else(|| "RandomPick requires an array".to_string())?;
            
            // 配列の要素型を確認
            match &array_ast.ty {
                Type::Vec(elem_type) => {
                    // Character型の場合は特殊処理
                    if **elem_type == Type::Character {
                        let array_node = convert_child::<Vec<Character>>(registry, typed_ast, "array")?;
                        // CharacterRandomPickNodeをBox<dyn Node<Character, _>>として返す
                        let node = Box::new(CharacterRandomPickNode::new(array_node)) as Box<ActionSystemNode<Character>>;
                        // 型安全性のため、unsafe transmute を使用
                        unsafe {
                            Ok(std::mem::transmute::<Box<ActionSystemNode<Character>>, Box<ActionSystemNode<T>>>(node))
                        }
                    } else {
                        // その他の型はRandomPickNode<T>を使用
                        let array_node = convert_child::<Vec<T>>(registry, typed_ast, "array")?;
                        Ok(Box::new(RandomPickNode::new(array_node)))
                    }
                }
                _ => Err(format!("RandomPick array must be Vec type, got {:?}", array_ast.ty))
            }
        } else {
            Err("Expected RandomPick token".to_string())
        }
    }
}