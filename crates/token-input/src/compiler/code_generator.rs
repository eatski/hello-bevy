//! コードジェネレータ
//! 
//! TypedASTから実行可能なNodeへの変換

use std::sync::Arc;
use action_system::{RuleNode, Action};
use crate::node_converter::{ConverterRegistry, TypedNode};
use crate::type_system::{TypedAst, Type, CompileError, CompileResult, TypeError};

/// コードジェネレータ
pub struct CodeGenerator {
    registry: Arc<ConverterRegistry>,
}

impl CodeGenerator {
    pub fn new() -> Self {
        Self {
            registry: Arc::new(ConverterRegistry::new()),
        }
    }
    
    /// TypedASTをNodeに変換
    pub fn generate(&self, ast: &TypedAst) -> CompileResult<RuleNode> {
        // TypedAstが保持しているtokenを直接使用
        // ConverterRegistryは既にenum型のStructuredTokenInputを想定しているので変換不要
        let typed_node = self.registry.convert_typed(&ast.token)
            .map_err(|e| CompileError::new(TypeError::UnresolvedType {
                context: format!("Failed to generate node: {}", e),
            }))?;
        
        // TypedNodeをRuleNodeに変換
        self.typed_node_to_rule_node(typed_node, &ast.ty)
    }
    
    /// TypedNodeをRuleNodeに変換
    fn typed_node_to_rule_node(&self, typed_node: TypedNode, expected_type: &Type) -> CompileResult<RuleNode> {
        // 型に応じて適切なダウンキャストを実行
        match expected_type {
            Type::Action => {
                let type_name = typed_node.type_name.clone();
                let action_node = typed_node.downcast::<Box<dyn Action>>()
                    .map_err(|e| CompileError::new(TypeError::TypeMismatch {
                        expected: Type::Action,
                        actual: self.type_name_to_type(&type_name),
                        context: e,
                    }))?;
                Ok(action_node)
            },
            _ => {
                // 他の型の場合、Action型への変換が必要
                // 現在のRuleNodeの定義では、Box<dyn Node<Box<dyn Action>, EvaluationContext>>が必要
                Err(CompileError::new(TypeError::UnresolvedType {
                    context: format!("Cannot convert {} to RuleNode. RuleNode requires Action type.", expected_type),
                }))
            }
        }
    }
    
    /// 型名をTypeに変換するヘルパーメソッド
    fn type_name_to_type(&self, type_name: &str) -> Type {
        match type_name {
            "Action" => Type::Action,
            "bool" => Type::Bool,
            "i32" => Type::I32,
            "Character" => Type::Character,
            "CharacterHP" => Type::CharacterHP,
            "Vec<Character>" => Type::Vec(Box::new(Type::Character)),
            "Vec<CharacterHP>" => Type::Vec(Box::new(Type::CharacterHP)),
            "Vec<i32>" => Type::Vec(Box::new(Type::I32)),
            "Vec<TeamSide>" => Type::Vec(Box::new(Type::TeamSide)),
            "TeamSide" => Type::TeamSide,
            _ => Type::Any,
        }
    }
}

impl Default for CodeGenerator {
    fn default() -> Self {
        Self::new()
    }
}