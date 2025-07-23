// 型情報を伝播させる条件コンバーター

use crate::{StructuredTokenInput, typed_node_converter::{TypedNodeConverter, TypedConverterRegistry, convert_child}};
use crate::type_system::{TypedAst, Type};
use action_system::*;
use action_system::nodes::condition::{EqConditionNode, CharacterHpVsValueGreaterThanNode};

// Type alias for Node trait with action-system's EvaluationContext
type ActionSystemNode<T> = dyn for<'a> Node<T, EvaluationContext<'a>> + Send + Sync;

/// 型情報を活用するGreaterThanコンバーター
pub struct TypedGreaterThanConverter;

impl TypedGreaterThanConverter {
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
                Type::I32 // default
            }
            _ => Type::I32 // default for other numeric types
        }
    }
}

impl TypedNodeConverter<bool> for TypedGreaterThanConverter {
    fn can_convert(&self, token: &StructuredTokenInput, expected_type: &Type) -> bool {
        matches!(token, StructuredTokenInput::GreaterThan { .. }) && 
        matches!(expected_type, Type::Bool)
    }
    
    fn convert(&self, 
               typed_ast: &TypedAst, 
               registry: &dyn TypedConverterRegistry) -> Result<Box<ActionSystemNode<bool>>, String> {
        if let StructuredTokenInput::GreaterThan { .. } = &typed_ast.token {
            // 子要素の型情報を確認
            let left_ast = typed_ast.children.get("left")
                .ok_or_else(|| "GreaterThan requires a left argument".to_string())?;
            let right_ast = typed_ast.children.get("right")
                .ok_or_else(|| "GreaterThan requires a right argument".to_string())?;
            
            let left_type = &left_ast.ty;
            let right_type = &right_ast.ty;
            
            // 型情報に基づいて適切なノードを生成
            match (left_type, right_type) {
                (Type::I32, Type::I32) => {
                    let left_node = convert_child::<i32>(registry, typed_ast, "left")?;
                    let right_node = convert_child::<i32>(registry, typed_ast, "right")?;
                    Ok(Box::new(GreaterThanNode::new(left_node, right_node)))
                }
                (Type::CharacterHP, Type::I32) => {
                    let left_node = convert_child::<CharacterHP>(registry, typed_ast, "left")?;
                    let right_node = convert_child::<i32>(registry, typed_ast, "right")?;
                    // Use the specialized CharacterHpVsValueGreaterThanNode for mixed types
                    Ok(Box::new(CharacterHpVsValueGreaterThanNode::new(left_node, right_node)))
                }
                (Type::I32, Type::CharacterHP) => {
                    let _left_node = convert_child::<i32>(registry, typed_ast, "left")?;
                    let _right_node = convert_child::<CharacterHP>(registry, typed_ast, "right")?;
                    // Swap the order and negate (a > b is !(b >= a))
                    return Err("GreaterThan with i32 on left and CharacterHP on right not supported".to_string());
                }
                (Type::CharacterHP, Type::CharacterHP) => {
                    let left_node = convert_child::<CharacterHP>(registry, typed_ast, "left")?;
                    let right_node = convert_child::<CharacterHP>(registry, typed_ast, "right")?;
                    Ok(Box::new(GreaterThanNode::new(left_node, right_node)))
                }
                // Handle Numeric types - they can be either i32 or CharacterHP
                (Type::Numeric, Type::I32) => {
                    // Check the actual type of the Numeric operand
                    let left_ast = typed_ast.children.get("left").unwrap();
                    if Self::infer_numeric_type(left_ast) == Type::CharacterHP {
                        // Convert both to CharacterHP
                        let left_node = convert_child::<CharacterHP>(registry, typed_ast, "left")?;
                        let right_node = convert_child::<i32>(registry, typed_ast, "right")?;
                        Ok(Box::new(CharacterHpVsValueGreaterThanNode::new(left_node, right_node)))
                    } else {
                        // Default to i32
                        let left_node = convert_child::<i32>(registry, typed_ast, "left")?;
                        let right_node = convert_child::<i32>(registry, typed_ast, "right")?;
                        Ok(Box::new(GreaterThanNode::new(left_node, right_node)))
                    }
                }
                (Type::I32, Type::Numeric) => {
                    // Check the actual type of the Numeric operand
                    let right_ast = typed_ast.children.get("right").unwrap();
                    if Self::infer_numeric_type(right_ast) == Type::CharacterHP {
                        let _left_node = convert_child::<i32>(registry, typed_ast, "left")?;
                        let _right_node = convert_child::<CharacterHP>(registry, typed_ast, "right")?;
                        return Err("GreaterThan with i32 on left and CharacterHP on right not supported".to_string());
                    } else {
                        // Default to i32
                        let left_node = convert_child::<i32>(registry, typed_ast, "left")?;
                        let right_node = convert_child::<i32>(registry, typed_ast, "right")?;
                        Ok(Box::new(GreaterThanNode::new(left_node, right_node)))
                    }
                }
                (Type::Numeric, Type::CharacterHP) => {
                    let left_node = convert_child::<CharacterHP>(registry, typed_ast, "left")?;
                    let right_node = convert_child::<CharacterHP>(registry, typed_ast, "right")?;
                    Ok(Box::new(GreaterThanNode::new(left_node, right_node)))
                }
                (Type::CharacterHP, Type::Numeric) => {
                    let left_node = convert_child::<CharacterHP>(registry, typed_ast, "left")?;
                    let right_node = convert_child::<CharacterHP>(registry, typed_ast, "right")?;
                    Ok(Box::new(GreaterThanNode::new(left_node, right_node)))
                }
                (Type::Numeric, Type::Numeric) => {
                    // When both are Numeric, default to i32
                    let left_node = convert_child::<i32>(registry, typed_ast, "left")?;
                    let right_node = convert_child::<i32>(registry, typed_ast, "right")?;
                    Ok(Box::new(GreaterThanNode::new(left_node, right_node)))
                }
                _ => Err(format!("GreaterThan not supported for types {:?} and {:?}", left_type, right_type))
            }
        } else {
            Err("Expected GreaterThan token".to_string())
        }
    }
}

/// 型情報を活用するEqualityコンバーター
pub struct TypedEqConverter;

impl TypedNodeConverter<bool> for TypedEqConverter {
    fn can_convert(&self, token: &StructuredTokenInput, expected_type: &Type) -> bool {
        matches!(token, StructuredTokenInput::Eq { .. }) && 
        matches!(expected_type, Type::Bool)
    }
    
    fn convert(&self, 
               typed_ast: &TypedAst, 
               registry: &dyn TypedConverterRegistry) -> Result<Box<ActionSystemNode<bool>>, String> {
        if let StructuredTokenInput::Eq { .. } = &typed_ast.token {
            // 子要素の型情報を確認
            let left_ast = typed_ast.children.get("left")
                .ok_or_else(|| "Eq requires a left argument".to_string())?;
            let right_ast = typed_ast.children.get("right")
                .ok_or_else(|| "Eq requires a right argument".to_string())?;
            
            let left_type = &left_ast.ty;
            let right_type = &right_ast.ty;
            
            // 型情報に基づいて適切なノードを生成
            match (left_type, right_type) {
                (Type::I32, Type::I32) => {
                    let left_node = convert_child::<i32>(registry, typed_ast, "left")?;
                    let right_node = convert_child::<i32>(registry, typed_ast, "right")?;
                    Ok(Box::new(EqConditionNode::new(left_node, right_node)))
                }
                (Type::Character, Type::Character) => {
                    let left_node = convert_child::<Character>(registry, typed_ast, "left")?;
                    let right_node = convert_child::<Character>(registry, typed_ast, "right")?;
                    Ok(Box::new(EqConditionNode::new(left_node, right_node)))
                }
                (Type::TeamSide, Type::TeamSide) => {
                    let left_node = convert_child::<TeamSide>(registry, typed_ast, "left")?;
                    let right_node = convert_child::<TeamSide>(registry, typed_ast, "right")?;
                    Ok(Box::new(EqConditionNode::new(left_node, right_node)))
                }
                _ => Err(format!("Eq not supported for types {:?} and {:?}", left_type, right_type))
            }
        } else {
            Err("Expected Eq token".to_string())
        }
    }
}

/// 型情報を活用するランダム条件コンバーター
pub struct TypedTrueOrFalseRandomConverter;

impl TypedNodeConverter<bool> for TypedTrueOrFalseRandomConverter {
    fn can_convert(&self, token: &StructuredTokenInput, expected_type: &Type) -> bool {
        matches!(token, StructuredTokenInput::TrueOrFalseRandom) && 
        matches!(expected_type, Type::Bool)
    }
    
    fn convert(&self, 
               _typed_ast: &TypedAst, 
               _registry: &dyn TypedConverterRegistry) -> Result<Box<ActionSystemNode<bool>>, String> {
        Ok(Box::new(RandomConditionNode))
    }
}