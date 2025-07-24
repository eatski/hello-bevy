//! 抽象的な型チェッカー
//! 
//! トークン固有のロジックを持たない、メタデータ駆動の型チェッカー

use std::collections::HashMap;
use crate::structured_token::StructuredTokenInput;
use super::{Type, TokenMetadataRegistry, TypeError, CompileError, CompileResult, TypedAst, TypeContext};

/// 抽象的な型チェッカー
pub struct TypeChecker {
    metadata_registry: TokenMetadataRegistry,
}

impl TypeChecker {
    pub fn new() -> Self {
        Self {
            metadata_registry: TokenMetadataRegistry::new(),
        }
    }
    
    /// カスタムメタデータレジストリを使用
    pub fn with_registry(metadata_registry: TokenMetadataRegistry) -> Self {
        Self { metadata_registry }
    }
    
    /// トークンの型チェックを実行
    pub fn check(&self, token: &StructuredTokenInput) -> CompileResult<TypedAst> {
        let mut context = TypeContext::new();
        self.check_with_context(token, &mut context)
    }
    
    /// コンテキスト付きで型チェック
    fn check_with_context(
        &self,
        token: &StructuredTokenInput,
        context: &mut TypeContext,
    ) -> CompileResult<TypedAst> {
        // トークンタイプを取得
        let token_type = self.get_token_type(token);
        
        // メタデータを取得
        let metadata = self.metadata_registry.get(&token_type)
            .ok_or_else(|| CompileError::new(TypeError::UndefinedToken {
                token_type: token_type.clone(),
            }))?;
        
        // カスタム検証があれば実行
        if let Some(validator) = metadata.custom_validator {
            validator(token).map_err(|e| CompileError::new(TypeError::UnresolvedType {
                context: e,
            }))?;
        }
        
        // 引数を抽出して型チェック
        let token_args = self.extract_arguments(token);
        let mut typed_children = HashMap::new();
        let mut arg_types = HashMap::new();
        
        // メタデータで定義された順序で引数を処理
        for arg_meta in &metadata.arguments {
            if let Some(arg_token) = token_args.get(&arg_meta.name) {
                // 引数のコンテキストを準備
                let arg_context = self.prepare_argument_context(
                    context,
                    &arg_meta.name,
                    &typed_children,
                    metadata,
                )?;
                
                // 型チェック実行
                let typed_arg = if let Some(ref ctx) = arg_context {
                    let mut new_context = context.clone();
                    new_context.set_current_context(Some(ctx.clone()));
                    self.check_with_context(arg_token, &mut new_context)?
                } else {
                    self.check_argument(
                        arg_token,
                        &arg_meta.expected_type,
                        context,
                        &token_type,
                        &arg_meta.name,
                    )?
                };
                
                arg_types.insert(arg_meta.name.clone(), typed_arg.ty.clone());
                typed_children.insert(arg_meta.name.clone(), typed_arg);
            } else if arg_meta.required {
                // 必須引数が不足
                return Err(CompileError::new(TypeError::MissingField {
                    token_type: token_type.clone(),
                    field_name: arg_meta.name.clone(),
                }));
            }
        }
        
        // 出力型を推論
        let output_type = if token_type == "Element" {
            // Elementは特殊: コンテキストから型を取得
            context.current_context()
                .and_then(|ty| match ty {
                    Type::Vec(elem_type) => Some(elem_type.as_ref().clone()),
                    _ => Some(ty.clone()),
                })
                .ok_or_else(|| CompileError::new(TypeError::UnresolvedType {
                    context: "Element used outside of list context".to_string(),
                }))?
        } else {
            metadata.infer_output_type(&arg_types)
        };
        
        // TypedASTを構築
        let mut typed_ast = TypedAst::new(token.clone(), output_type.clone());
        typed_ast.children = typed_children;
        
        Ok(typed_ast)
    }
    
    /// 引数の型チェック（特殊なケースの処理を含む）
    fn check_argument(
        &self,
        arg_token: &StructuredTokenInput,
        expected_type: &Type,
        context: &mut TypeContext,
        parent_token_type: &str,
        arg_name: &str,
    ) -> CompileResult<TypedAst> {
        // 通常の型チェック
        let typed_arg = self.check_with_context(arg_token, context)
            .map_err(|e| e.add_context(format!("{}.{}", parent_token_type, arg_name)))?;
        
        // 型の互換性チェック
        if !typed_arg.ty.is_compatible_with(expected_type) {
            return Err(CompileError::new(TypeError::TypeMismatch {
                expected: expected_type.clone(),
                actual: typed_arg.ty.clone(),
                context: format!("{}.{}", parent_token_type, arg_name),
            }));
        }
        
        Ok(typed_arg)
    }
    
    /// トークンからトークンタイプを取得
    fn get_token_type(&self, token: &StructuredTokenInput) -> String {
        match token {
            StructuredTokenInput::Strike { .. } => "Strike",
            StructuredTokenInput::Heal { .. } => "Heal",
            StructuredTokenInput::Check { .. } => "Check",
            StructuredTokenInput::TrueOrFalseRandom => "TrueOrFalseRandom",
            StructuredTokenInput::GreaterThan { .. } => "GreaterThan",
            StructuredTokenInput::LessThan { .. } => "LessThan",
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
    
    /// 引数のコンテキストを準備
    fn prepare_argument_context(
        &self,
        _context: &TypeContext,
        arg_name: &str,
        typed_children: &HashMap<String, TypedAst>,
        metadata: &super::TokenMetadata,
    ) -> Result<Option<Type>, CompileError> {
        // メタデータに定義されたコンテキスト準備ロジックを使用
        if let Some(context_provider) = metadata.argument_context_provider {
            context_provider(arg_name, typed_children)
        } else {
            Ok(None)
        }
    }
    
    /// トークンから引数を抽出
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
            StructuredTokenInput::LessThan { left, right } |
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
            StructuredTokenInput::Number { value: _ } => {
                // Numberは特殊: valueを仮想的なトークンとして扱う
                // この処理は後で改善が必要
            }
            _ => {} // 引数なしのトークン
        }
        
        args
    }
}

impl Default for TypeChecker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_metadata_driven_type_checking() {
        let checker = TypeChecker::new();
        
        // Strikeトークンのテスト
        let token = StructuredTokenInput::Strike {
            target: Box::new(StructuredTokenInput::ActingCharacter),
        };
        
        let result = checker.check(&token).unwrap();
        assert_eq!(result.ty, Type::Action);
    }
    
    #[test]
    fn test_type_mismatch_detection() {
        let checker = TypeChecker::new();
        
        // 型が合わないトークン
        let token = StructuredTokenInput::Strike {
            target: Box::new(StructuredTokenInput::Number { value: 42 }),
        };
        
        let result = checker.check(&token);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_custom_metadata_registry() {
        let mut registry = TokenMetadataRegistry::new();
        
        // カスタムトークンを追加
        use crate::type_system::TokenMetadata;
        registry.register(TokenMetadata {
            token_type: "CustomToken".to_string(),
            arguments: vec![],
            output_type: Type::String,
            custom_validator: None,
            output_type_inference: None,
            argument_context_provider: None,
        });
        
        let _checker = TypeChecker::with_registry(registry);
        // 実際のテストは、StructuredTokenInputにCustomTokenが追加されたら可能
    }
}