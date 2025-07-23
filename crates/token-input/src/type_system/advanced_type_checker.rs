//! Phase 2対応の高度な型チェッカー
//! 
//! Hindley-Milner型推論、traitシステム、高度なジェネリクスを統合

use std::collections::HashMap;
use crate::structured_token::StructuredTokenInput;
use super::{
    Type, TokenMetadataRegistry, TypeError, CompileError, CompileResult, TypedAst, TypeContext,
    hindley_milner::{HindleyMilner, PolyType, TypeEnv},
};

/// Phase 2対応の型チェッカー
pub struct AdvancedTypeChecker {
    /// トークンメタデータ
    metadata_registry: TokenMetadataRegistry,
    /// Hindley-Milner型推論エンジン
    hm_engine: HindleyMilner,
    /// 型環境
    type_env: TypeEnv,
}

impl AdvancedTypeChecker {
    pub fn new() -> Self {
        Self {
            metadata_registry: TokenMetadataRegistry::new(),
            hm_engine: HindleyMilner::new(),
            type_env: TypeEnv::new(),
        }
    }
    
    /// トークンの型チェックを実行（Phase 2機能付き）
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
        // トークンタイプを取得
        let token_type = self.get_token_type(token);
        
        // メタデータを取得してクローン（借用を回避）
        let metadata = self.metadata_registry.get(&token_type)
            .ok_or_else(|| CompileError::new(TypeError::UndefinedToken {
                token_type: token_type.clone(),
            }))?
            .clone();
        
        // カスタム検証
        if let Some(validator) = metadata.custom_validator {
            validator(token).map_err(|e| CompileError::new(TypeError::UnresolvedType {
                context: e,
            }))?;
        }
        
        // 引数の型推論
        let token_args = self.extract_arguments(token);
        let mut typed_children = HashMap::new();
        let mut arg_poly_types = HashMap::new();
        
        // FilterList/Map特殊処理
        let mut element_context = None;
        if matches!(token_type.as_str(), "FilterList" | "Map") {
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
        for arg_meta in &metadata.arguments {
            if typed_children.contains_key(&arg_meta.name) {
                continue;
            }
            
            if let Some(arg_token) = token_args.get(&arg_meta.name) {
                let typed_arg = if matches!((token_type.as_str(), arg_meta.name.as_str()), 
                                          ("FilterList", "condition") | 
                                          ("Map", "transform")) 
                                   && element_context.is_some() {
                    let mut new_context = context.clone();
                    new_context.set_current_context(element_context.clone());
                    self.check_with_inference(arg_token, &mut new_context)?
                } else {
                    self.check_with_inference(arg_token, context)?
                };
                
                // 型推論制約を追加（Numeric型とAny型、Vec(Any)は特殊処理）
                let should_add_constraint = match (&arg_meta.expected_type, &typed_arg.ty) {
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
                    let expected_poly = PolyType::Concrete(arg_meta.expected_type.clone());
                    let actual_poly = PolyType::Concrete(typed_arg.ty.clone());
                    self.hm_engine.add_constraint(expected_poly, actual_poly);
                }
                
                // Trait境界チェック
                if arg_meta.expected_type == Type::Numeric {
                    // Numeric traitを要求 - I32とCharacterHPは互換性がある
                    if !matches!(typed_arg.ty, Type::I32 | Type::CharacterHP | Type::Numeric) {
                        return Err(CompileError::new(TypeError::TypeMismatch {
                            expected: arg_meta.expected_type.clone(),
                            actual: typed_arg.ty.clone(),
                            context: format!("Type {} does not implement Numeric trait", typed_arg.ty),
                        }));
                    }
                }
                
                arg_poly_types.insert(arg_meta.name.clone(), PolyType::Concrete(typed_arg.ty.clone()));
                typed_children.insert(arg_meta.name.clone(), typed_arg);
            } else if arg_meta.required {
                return Err(CompileError::new(TypeError::MissingField {
                    token_type: token_type.clone(),
                    field_name: arg_meta.name.clone(),
                }));
            }
        }
        
        // 制約を解決
        self.hm_engine.solve_constraints()
            .map_err(|e| CompileError::new(TypeError::UnresolvedType { context: e }))?;
        
        // 出力型を推論
        let output_type = self.infer_output_type(&token_type, &typed_children, context)?;
        
        // 多相型の一般化（let多相性）
        if self.is_value_binding(&token_type) {
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
    
    /// 出力型を推論（Phase 2対応）
    fn infer_output_type(
        &self,
        token_type: &str,
        typed_args: &HashMap<String, TypedAst>,
        context: &TypeContext,
    ) -> Result<Type, CompileError> {
        // 特殊なケースの処理
        match token_type {
            "Element" => {
                context.current_context()
                    .and_then(|ty| match ty {
                        Type::Vec(elem_type) => Some(elem_type.as_ref().clone()),
                        _ => Some(ty.clone()),
                    })
                    .ok_or_else(|| CompileError::new(TypeError::UnresolvedType {
                        context: "Element used outside of list context".to_string(),
                    }))
            }
            _ => {
                // メタデータから推論
                let metadata = self.metadata_registry.get(token_type).unwrap();
                let arg_types: HashMap<String, Type> = typed_args.iter()
                    .map(|(k, v)| (k.clone(), v.ty.clone()))
                    .collect();
                Ok(metadata.infer_output_type(&arg_types))
            }
        }
    }
    
    /// 値束縛かどうか判定
    fn is_value_binding(&self, _token_type: &str) -> bool {
        // 将来的にletバインディングを追加した場合に使用
        false
    }
    
    /// 束縛名を取得
    fn get_binding_name(&self, _token: &StructuredTokenInput) -> Option<String> {
        // 将来的にletバインディングを追加した場合に使用
        None
    }
    
    /// トークンタイプを取得（既存の実装を流用）
    fn get_token_type(&self, token: &StructuredTokenInput) -> String {
        match token {
            StructuredTokenInput::Strike { .. } => "Strike",
            StructuredTokenInput::Heal { .. } => "Heal",
            StructuredTokenInput::Check { .. } => "Check",
            StructuredTokenInput::TrueOrFalseRandom => "TrueOrFalseRandom",
            StructuredTokenInput::GreaterThan { .. } => "GreaterThan",
            StructuredTokenInput::Eq { .. } => "Eq",
            StructuredTokenInput::ActingCharacter => "ActingCharacter",
            StructuredTokenInput::AllCharacters => "AllCharacters",
            StructuredTokenInput::CharacterToHp { .. } => "CharacterToHp",
            StructuredTokenInput::CharacterHpToCharacter { .. } => "CharacterHpToCharacter",
            StructuredTokenInput::RandomPick { .. } => "RandomPick",
            StructuredTokenInput::FilterList { .. } => "FilterList",
            StructuredTokenInput::Map { .. } => "Map",
            StructuredTokenInput::Element => "Element",
            StructuredTokenInput::CharacterTeam { .. } => "CharacterTeam",
            StructuredTokenInput::TeamMembers { .. } => "TeamMembers",
            StructuredTokenInput::AllTeamSides => "AllTeamSides",
            StructuredTokenInput::Enemy => "Enemy",
            StructuredTokenInput::Hero => "Hero",
            StructuredTokenInput::Number { .. } => "Number",
            StructuredTokenInput::Max { .. } => "Max",
            StructuredTokenInput::Min { .. } => "Min",
            StructuredTokenInput::NumericMax { .. } => "NumericMax",
            StructuredTokenInput::NumericMin { .. } => "NumericMin",
        }.to_string()
    }
    
    /// 引数を抽出（既存の実装を流用）
    fn extract_arguments<'a>(&self, token: &'a StructuredTokenInput) -> HashMap<String, &'a StructuredTokenInput> {
        let mut args = HashMap::new();
        
        match token {
            StructuredTokenInput::Strike { target } |
            StructuredTokenInput::Heal { target } => {
                args.insert("target".to_string(), target.as_ref());
            }
            StructuredTokenInput::Check { condition, then_action } => {
                args.insert("condition".to_string(), condition.as_ref());
                args.insert("then_action".to_string(), then_action.as_ref());
            }
            StructuredTokenInput::GreaterThan { left, right } |
            StructuredTokenInput::Eq { left, right } => {
                args.insert("left".to_string(), left.as_ref());
                args.insert("right".to_string(), right.as_ref());
            }
            StructuredTokenInput::CharacterToHp { character } |
            StructuredTokenInput::CharacterTeam { character } => {
                args.insert("character".to_string(), character.as_ref());
            }
            StructuredTokenInput::CharacterHpToCharacter { character_hp } => {
                args.insert("character_hp".to_string(), character_hp.as_ref());
            }
            StructuredTokenInput::TeamMembers { team_side } => {
                args.insert("team_side".to_string(), team_side.as_ref());
            }
            StructuredTokenInput::RandomPick { array } |
            StructuredTokenInput::Max { array } |
            StructuredTokenInput::Min { array } |
            StructuredTokenInput::NumericMax { array } |
            StructuredTokenInput::NumericMin { array } => {
                args.insert("array".to_string(), array.as_ref());
            }
            StructuredTokenInput::FilterList { array, condition } => {
                args.insert("array".to_string(), array.as_ref());
                args.insert("condition".to_string(), condition.as_ref());
            }
            StructuredTokenInput::Map { array, transform } => {
                args.insert("array".to_string(), array.as_ref());
                args.insert("transform".to_string(), transform.as_ref());
            }
            _ => {}
        }
        
        args
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
    fn test_phase2_type_inference() {
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
        assert_eq!(result.unwrap().ty, Type::Numeric);
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