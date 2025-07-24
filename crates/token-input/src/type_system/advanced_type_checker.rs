//! 高度な型チェッカー
//! 
//! Hindley-Milner型推論、traitシステム、高度なジェネリクスを統合

use std::collections::HashMap;
use crate::structured_token::StructuredTokenInput;
use super::{
    Type, TypeError, CompileError, CompileResult, TypedAst, TypeContext,
    hindley_milner::{HindleyMilner, PolyType, TypeEnv},
};

/// 高度な型チェッカー
pub struct AdvancedTypeChecker {
    /// Hindley-Milner型推論エンジン
    hm_engine: HindleyMilner,
    /// 型環境
    type_env: TypeEnv,
}

impl AdvancedTypeChecker {
    pub fn new() -> Self {
        Self {
            hm_engine: HindleyMilner::new(),
            type_env: TypeEnv::new(),
        }
    }
    
    /// トークンの型チェックを実行
    pub fn check(&mut self, token: &StructuredTokenInput) -> CompileResult<TypedAst> {
        let mut context = TypeContext::new();
        self.check_with_inference(token, &mut context)
    }
    
    /// 型推論付きチェック
    fn check_with_inference(
        &mut self,
        token: &StructuredTokenInput,
        context: &mut TypeContext,
    ) -> CompileResult<TypedAst> {
        // トークンから直接期待される引数の型を取得
        let expected_args = token.expected_argument_types();
        
        // 引数の型推論
        let token_args: HashMap<String, &StructuredTokenInput> = token.arguments()
            .into_iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect();
        let mut typed_children = HashMap::new();
        let mut arg_poly_types = HashMap::new();
        
        // FilterList/Map特殊処理
        let mut element_context = None;
        if matches!(token, StructuredTokenInput::FilterList { .. } | StructuredTokenInput::Map { .. }) {
            if let Some(array_arg) = token_args.get("array") {
                let typed_array = self.check_with_inference(array_arg, context)?;
                
                if let Type::Vec(elem_type) = &typed_array.ty {
                    element_context = Some((**elem_type).clone());
                }
                
                arg_poly_types.insert("array".to_string(), PolyType::Concrete(typed_array.ty.clone()));
                typed_children.insert("array".to_string(), typed_array);
            }
        }
        
        // 各引数の型推論
        for (arg_name, expected_type) in expected_args {
            if typed_children.contains_key(arg_name) {
                continue;
            }
            
            if let Some(arg_token) = token_args.get(arg_name) {
                let typed_arg = if (matches!(token, StructuredTokenInput::FilterList { .. }) && arg_name == "condition") ||
                                   (matches!(token, StructuredTokenInput::Map { .. }) && arg_name == "transform") {
                    if let Some(elem_ctx) = &element_context {
                        let mut new_context = context.clone();
                        new_context.set_current_context(Some(elem_ctx.clone()));
                        self.check_with_inference(arg_token, &mut new_context)?
                    } else {
                        self.check_with_inference(arg_token, context)?
                    }
                } else {
                    self.check_with_inference(arg_token, context)?
                };
                
                // 型の互換性をチェック
                if !typed_arg.ty.is_compatible_with(&expected_type) {
                    return Err(CompileError::new(TypeError::TypeMismatch {
                        expected: expected_type.clone(),
                        actual: typed_arg.ty.clone(),
                        context: format!("{:?}.{}", token, arg_name),
                    }));
                }
                
                // 型推論制約を追加（Numeric型とAny型、Vec(Any)は特殊処理）
                let should_add_constraint = match (&expected_type, &typed_arg.ty) {
                    (Type::Numeric, _) => false,
                    (Type::Any, _) => false,
                    (_, Type::Any) => false,
                    (Type::Vec(expected_elem), Type::Vec(_actual_elem)) => {
                        // Vec(Any)は任意のVec型を受け入れる
                        !matches!(expected_elem.as_ref(), Type::Any)
                    }
                    _ => true,
                };
                
                if should_add_constraint {
                    let expected_poly = PolyType::Concrete(expected_type.clone());
                    let actual_poly = PolyType::Concrete(typed_arg.ty.clone());
                    self.hm_engine.add_constraint(expected_poly, actual_poly);
                }
                
                // Trait境界チェック
                if expected_type == Type::Numeric {
                    // Numeric traitを要求 - I32とCharacterHPは互換性がある
                    if !matches!(typed_arg.ty, Type::I32 | Type::CharacterHP | Type::Numeric) {
                        return Err(CompileError::new(TypeError::TypeMismatch {
                            expected: expected_type.clone(),
                            actual: typed_arg.ty.clone(),
                            context: format!("Type {} does not implement Numeric trait", typed_arg.ty),
                        }));
                    }
                }
                
                arg_poly_types.insert(arg_name.to_string(), PolyType::Concrete(typed_arg.ty.clone()));
                typed_children.insert(arg_name.to_string(), typed_arg);
            } else {
                return Err(CompileError::new(TypeError::MissingField {
                    token_type: format!("{:?}", token),
                    field_name: arg_name.to_string(),
                }));
            }
        }
        
        // 制約を解決
        self.hm_engine.solve_constraints()
            .map_err(|e| CompileError::new(TypeError::UnresolvedType { context: e }))?;
        
        // 出力型を推論
        let output_type = self.infer_output_type(token, &typed_children, context)?;
        
        // 多相型の一般化（let多相性）
        if self.is_value_binding(token) {
            let poly_type = PolyType::Concrete(output_type.clone());
            let type_scheme = self.hm_engine.generalize(&self.type_env, &poly_type);
            
            // 型環境に登録（将来の参照用）
            if let Some(binding_name) = self.get_binding_name(token) {
                self.type_env.bind(binding_name, type_scheme);
            }
        }
        
        // TypedASTを構築
        let mut typed_ast = TypedAst::new(token.clone(), output_type);
        typed_ast.children = typed_children;
        
        Ok(typed_ast)
    }
    
    /// 出力型を推論
    fn infer_output_type(
        &self,
        token: &StructuredTokenInput,
        typed_args: &HashMap<String, TypedAst>,
        context: &TypeContext,
    ) -> Result<Type, CompileError> {
        // 特殊なケースの処理
        match token {
            StructuredTokenInput::Element => {
                context.current_context()
                    .and_then(|ty| match ty {
                        Type::Vec(elem_type) => Some(elem_type.as_ref().clone()),
                        _ => Some(ty.clone()),
                    })
                    .ok_or_else(|| CompileError::new(TypeError::UnresolvedType {
                        context: "Element used outside of list context".to_string(),
                    }))
            }
            StructuredTokenInput::FilterList { .. } => {
                // FilterListの出力型は入力配列の型と同じ
                Ok(typed_args.get("array")
                    .map(|ast| ast.ty.clone())
                    .unwrap_or(Type::Vec(Box::new(Type::Any))))
            }
            StructuredTokenInput::Map { .. } => {
                // Mapの出力型は変換後の要素型の配列
                Ok(typed_args.get("transform")
                    .map(|ast| Type::Vec(Box::new(ast.ty.clone())))
                    .unwrap_or(Type::Vec(Box::new(Type::Any))))
            }
            StructuredTokenInput::RandomPick { .. } => {
                // RandomPickの出力型は配列の要素型
                Ok(typed_args.get("array")
                    .and_then(|ast| match &ast.ty {
                        Type::Vec(elem_type) => Some(elem_type.as_ref().clone()),
                        _ => None,
                    })
                    .unwrap_or(Type::Any))
            }
            StructuredTokenInput::NumericMax { .. } | StructuredTokenInput::NumericMin { .. } => {
                // Numeric型操作の出力は具体的な型に依存
                Ok(typed_args.get("array")
                    .and_then(|ast| match &ast.ty {
                        Type::Vec(elem_type) => match elem_type.as_ref() {
                            Type::I32 => Some(Type::I32),
                            Type::CharacterHP => Some(Type::CharacterHP),
                            Type::Numeric => Some(Type::Numeric),
                            _ => None,
                        },
                        _ => None,
                    })
                    .unwrap_or(Type::Numeric))
            }
            _ => {
                // その他のトークンは事前定義された出力型を使用
                Ok(token.output_type())
            }
        }
    }
    
    /// 値束縛かどうか判定
    fn is_value_binding(&self, _token: &StructuredTokenInput) -> bool {
        // 将来的にletバインディングを追加した場合に使用
        false
    }
    
    /// 束縛名を取得
    fn get_binding_name(&self, _token: &StructuredTokenInput) -> Option<String> {
        // 将来的にletバインディングを追加した場合に使用
        None
    }
    
}

impl Default for AdvancedTypeChecker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_numeric_type_inference() {
        let mut checker = AdvancedTypeChecker::new();
        
        // Numeric trait を活用した型推論
        let token = StructuredTokenInput::GreaterThan {
            left: Box::new(StructuredTokenInput::CharacterToHp {
                character: Box::new(StructuredTokenInput::ActingCharacter),
            }),
            right: Box::new(StructuredTokenInput::Number { value: 50 }),
        };
        
        let result = checker.check(&token);
        assert!(result.is_ok());
        
        let typed_ast = result.unwrap();
        assert_eq!(typed_ast.ty, Type::Bool);
    }
    
    #[test]
    fn test_trait_bounds() {
        let mut checker = AdvancedTypeChecker::new();
        
        // NumericMaxはNumeric traitを要求
        let token = StructuredTokenInput::NumericMax {
            array: Box::new(StructuredTokenInput::Map {
                array: Box::new(StructuredTokenInput::AllCharacters),
                transform: Box::new(StructuredTokenInput::CharacterToHp {
                    character: Box::new(StructuredTokenInput::Element),
                }),
            }),
        };
        
        let result = checker.check(&token);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().ty, Type::CharacterHP);
    }
    
    #[test]
    fn test_polymorphic_function() {
        let mut checker = AdvancedTypeChecker::new();
        
        // FilterListは多相的（任意の配列型で動作）
        let token = StructuredTokenInput::FilterList {
            array: Box::new(StructuredTokenInput::AllCharacters),
            condition: Box::new(StructuredTokenInput::TrueOrFalseRandom),
        };
        
        let result = checker.check(&token);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().ty, Type::Vec(Box::new(Type::Character)));
    }
}