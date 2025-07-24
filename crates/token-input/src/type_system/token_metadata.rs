//! トークンメタデータシステム
//! 
//! 各トークンの型情報と検証ルールを宣言的に定義

use super::{Type, TypeSignature, TypedAst, CompileError};
use std::collections::HashMap;

/// トークンの引数メタデータ
#[derive(Debug, Clone)]
pub struct ArgumentMetadata {
    /// 引数名
    pub name: String,
    /// 期待される型
    pub expected_type: Type,
    /// 必須かどうか
    pub required: bool,
    /// デフォルト値を生成する関数（オプション）
    pub default_value: Option<fn() -> crate::structured_token::StructuredTokenInput>,
}

/// トークンのメタデータ
#[derive(Clone)]
pub struct TokenMetadata {
    /// トークンタイプ
    pub token_type: String,
    /// 引数のメタデータ
    pub arguments: Vec<ArgumentMetadata>,
    /// 出力型
    pub output_type: Type,
    /// カスタム検証ロジック（オプション）
    pub custom_validator: Option<fn(&crate::structured_token::StructuredTokenInput) -> Result<(), String>>,
    /// 出力型の推論ロジック（オプション）
    pub output_type_inference: Option<fn(&HashMap<String, Type>) -> Type>,
    /// 引数のコンテキストを準備するロジック（オプション）
    pub argument_context_provider: Option<fn(&str, &HashMap<String, super::TypedAst>) -> Result<Option<Type>, super::CompileError>>,
}

impl TokenMetadata {
    /// 型シグネチャを生成
    pub fn to_signature(&self) -> TypeSignature {
        let inputs = self.arguments.iter()
            .filter(|arg| arg.required)
            .map(|arg| (arg.name.clone(), arg.expected_type.clone()))
            .collect();
        
        TypeSignature::new(inputs, self.output_type.clone())
    }
    
    /// 引数の型マップから出力型を推論
    pub fn infer_output_type(&self, arg_types: &HashMap<String, Type>) -> Type {
        if let Some(inference_fn) = self.output_type_inference {
            inference_fn(arg_types)
        } else {
            self.output_type.clone()
        }
    }
}

/// トークンメタデータのレジストリ
pub struct TokenMetadataRegistry {
    metadata: HashMap<String, TokenMetadata>,
}

impl TokenMetadataRegistry {
    pub fn new() -> Self {
        Self {
            metadata: HashMap::new(),
        }
    }
    
    /// メタデータを登録したレジストリを作成
    pub fn with_builtin_tokens() -> Self {
        let mut registry = Self::new();
        registry.register_builtin_tokens();
        registry
    }
    
    /// 組み込みトークンのメタデータを登録
    fn register_builtin_tokens(&mut self) {
        // 全てのトークンタイプを自動的に検出して登録
        let token_types = vec![
            "Strike", "Heal", "Check", "TrueOrFalseRandom",
            "GreaterThan", "LessThan", "Eq",
            "ActingCharacter", "AllCharacters", "CharacterToHp", "CharacterHpToCharacter",
            "RandomPick", "FilterList", "Map", "Element",
            "CharacterTeam", "TeamMembers", "AllTeamSides",
            "Enemy", "Hero", "Number",
            "Max", "Min", "NumericMax", "NumericMin",
        ];
        
        for token_type in token_types {
            self.try_generate_metadata(token_type);
        }
    }
    
    /// メタデータを登録
    pub fn register(&mut self, metadata: TokenMetadata) {
        self.metadata.insert(metadata.token_type.clone(), metadata);
    }
    
    /// メタデータを取得
    pub fn get(&self, token_type: &str) -> Option<&TokenMetadata> {
        self.metadata.get(token_type)
    }
    
    /// メタデータを取得（登録されていない場合は動的に生成）
    pub fn get_or_generate(&mut self, token_type: &str) -> Option<&TokenMetadata> {
        if self.metadata.contains_key(token_type) {
            return self.metadata.get(token_type);
        }
        
        // 登録されていない場合は動的に生成を試みる
        self.try_generate_metadata(token_type);
        self.metadata.get(token_type)
    }
    
    /// トークンタイプから動的にメタデータを生成
    fn try_generate_metadata(&mut self, token_type: &str) {
        use crate::structured_token::StructuredTokenInput;
        
        // サンプルトークンを作成してメタデータを取得
        let sample_token = match token_type {
            "Strike" => Some(StructuredTokenInput::Strike { 
                target: Box::new(StructuredTokenInput::ActingCharacter) 
            }),
            "Heal" => Some(StructuredTokenInput::Heal { 
                target: Box::new(StructuredTokenInput::ActingCharacter) 
            }),
            "Check" => Some(StructuredTokenInput::Check {
                condition: Box::new(StructuredTokenInput::TrueOrFalseRandom),
                then_action: Box::new(StructuredTokenInput::Strike { 
                    target: Box::new(StructuredTokenInput::ActingCharacter) 
                }),
            }),
            "TrueOrFalseRandom" => Some(StructuredTokenInput::TrueOrFalseRandom),
            "GreaterThan" => Some(StructuredTokenInput::GreaterThan {
                left: Box::new(StructuredTokenInput::Number { value: 0 }),
                right: Box::new(StructuredTokenInput::Number { value: 0 }),
            }),
            "LessThan" => Some(StructuredTokenInput::LessThan {
                left: Box::new(StructuredTokenInput::Number { value: 0 }),
                right: Box::new(StructuredTokenInput::Number { value: 0 }),
            }),
            "Eq" => Some(StructuredTokenInput::Eq {
                left: Box::new(StructuredTokenInput::Number { value: 0 }),
                right: Box::new(StructuredTokenInput::Number { value: 0 }),
            }),
            "ActingCharacter" => Some(StructuredTokenInput::ActingCharacter),
            "AllCharacters" => Some(StructuredTokenInput::AllCharacters),
            "CharacterToHp" => Some(StructuredTokenInput::CharacterToHp {
                character: Box::new(StructuredTokenInput::ActingCharacter),
            }),
            "CharacterHpToCharacter" => Some(StructuredTokenInput::CharacterHpToCharacter {
                character_hp: Box::new(StructuredTokenInput::CharacterToHp {
                    character: Box::new(StructuredTokenInput::ActingCharacter),
                }),
            }),
            "RandomPick" => Some(StructuredTokenInput::RandomPick {
                array: Box::new(StructuredTokenInput::AllCharacters),
            }),
            "FilterList" => Some(StructuredTokenInput::FilterList {
                array: Box::new(StructuredTokenInput::AllCharacters),
                condition: Box::new(StructuredTokenInput::TrueOrFalseRandom),
            }),
            "Map" => Some(StructuredTokenInput::Map {
                array: Box::new(StructuredTokenInput::AllCharacters),
                transform: Box::new(StructuredTokenInput::CharacterToHp {
                    character: Box::new(StructuredTokenInput::Element),
                }),
            }),
            "Element" => Some(StructuredTokenInput::Element),
            "CharacterTeam" => Some(StructuredTokenInput::CharacterTeam {
                character: Box::new(StructuredTokenInput::ActingCharacter),
            }),
            "TeamMembers" => Some(StructuredTokenInput::TeamMembers {
                team_side: Box::new(StructuredTokenInput::Hero),
            }),
            "AllTeamSides" => Some(StructuredTokenInput::AllTeamSides),
            "Enemy" => Some(StructuredTokenInput::Enemy),
            "Hero" => Some(StructuredTokenInput::Hero),
            "Number" => Some(StructuredTokenInput::Number { value: 0 }),
            "Max" => Some(StructuredTokenInput::Max {
                array: Box::new(StructuredTokenInput::AllCharacters),
            }),
            "Min" => Some(StructuredTokenInput::Min {
                array: Box::new(StructuredTokenInput::AllCharacters),
            }),
            "NumericMax" => Some(StructuredTokenInput::NumericMax {
                array: Box::new(StructuredTokenInput::Map {
                    array: Box::new(StructuredTokenInput::AllCharacters),
                    transform: Box::new(StructuredTokenInput::CharacterToHp {
                        character: Box::new(StructuredTokenInput::Element),
                    }),
                }),
            }),
            "NumericMin" => Some(StructuredTokenInput::NumericMin {
                array: Box::new(StructuredTokenInput::Map {
                    array: Box::new(StructuredTokenInput::AllCharacters),
                    transform: Box::new(StructuredTokenInput::CharacterToHp {
                        character: Box::new(StructuredTokenInput::Element),
                    }),
                }),
            }),
            _ => None,
        };
        
        if let Some(token) = sample_token {
            let arguments = token.expected_argument_types()
                .into_iter()
                .map(|(name, expected_type)| ArgumentMetadata {
                    name: name.to_string(),
                    expected_type,
                    required: true,
                    default_value: None,
                })
                .collect();
            
            let output_type = token.output_type();
            
            // 特殊な型推論やコンテキストプロバイダーを設定
            let output_type_inference = match token_type {
                "FilterList" => Some(filter_list_type_inference as fn(&HashMap<std::string::String, Type>) -> Type),
                "Map" => Some(map_type_inference as fn(&HashMap<std::string::String, Type>) -> Type),
                "RandomPick" => Some(random_pick_type_inference as fn(&HashMap<std::string::String, Type>) -> Type),
                "Element" => Some(element_type_inference as fn(&HashMap<std::string::String, Type>) -> Type),
                _ => None,
            };
            
            let argument_context_provider = match token_type {
                "FilterList" => Some(filter_list_context_provider as fn(&str, &HashMap<std::string::String, TypedAst>) -> Result<std::option::Option<Type>, CompileError>),
                "Map" => Some(map_context_provider as fn(&str, &HashMap<std::string::String, TypedAst>) -> Result<std::option::Option<Type>, CompileError>),
                _ => None,
            };
            
            self.register(TokenMetadata {
                token_type: token_type.to_string(),
                arguments,
                output_type,
                custom_validator: None,
                output_type_inference,
                argument_context_provider,
            });
        }
    }
}

impl Default for TokenMetadataRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// 型推論関数
fn filter_list_type_inference(arg_types: &HashMap<std::string::String, Type>) -> Type {
    arg_types.get("array").cloned().unwrap_or(Type::Vec(Box::new(Type::Any)))
}

fn map_type_inference(arg_types: &HashMap<std::string::String, Type>) -> Type {
    if let Some(transform_type) = arg_types.get("transform") {
        Type::Vec(Box::new(transform_type.clone()))
    } else {
        Type::Vec(Box::new(Type::Any))
    }
}

fn random_pick_type_inference(arg_types: &HashMap<std::string::String, Type>) -> Type {
    if let Some(Type::Vec(elem_type)) = arg_types.get("array") {
        (**elem_type).clone()
    } else {
        Type::Any
    }
}

fn element_type_inference(_: &HashMap<std::string::String, Type>) -> Type {
    Type::Any
}

// コンテキストプロバイダー関数
fn filter_list_context_provider(arg_name: &str, args: &HashMap<std::string::String, TypedAst>) -> Result<std::option::Option<Type>, CompileError> {
    if arg_name == "condition" {
        if let Some(array_ast) = args.get("array") {
            if let Type::Vec(elem_type) = &array_ast.ty {
                return Ok(Some(elem_type.as_ref().clone()));
            }
        }
    }
    Ok(None)
}

fn map_context_provider(arg_name: &str, args: &HashMap<std::string::String, TypedAst>) -> Result<std::option::Option<Type>, CompileError> {
    if arg_name == "transform" {
        if let Some(array_ast) = args.get("array") {
            if let Type::Vec(elem_type) = &array_ast.ty {
                return Ok(Some(elem_type.as_ref().clone()));
            }
        }
    }
    Ok(None)
}