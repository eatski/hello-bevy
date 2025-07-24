//! トークンメタデータシステム
//! 
//! 各トークンの型情報と検証ルールを宣言的に定義

use super::{Type, TypeSignature};
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
        let mut registry = Self {
            metadata: HashMap::new(),
        };
        registry.register_builtin_tokens();
        registry
    }
    
    /// 組み込みトークンのメタデータを登録
    fn register_builtin_tokens(&mut self) {
        use Type::*;
        
        // アクション
        self.register(TokenMetadata {
            token_type: "Strike".to_string(),
            arguments: vec![
                ArgumentMetadata {
                    name: "target".to_string(),
                    expected_type: Character,
                    required: true,
                    default_value: None,
                },
            ],
            output_type: Action,
            custom_validator: None,
            output_type_inference: None,
            argument_context_provider: None,
        });
        
        self.register(TokenMetadata {
            token_type: "Heal".to_string(),
            arguments: vec![
                ArgumentMetadata {
                    name: "target".to_string(),
                    expected_type: Character,
                    required: true,
                    default_value: None,
                },
            ],
            output_type: Action,
            custom_validator: None,
            output_type_inference: None,
            argument_context_provider: None,
        });
        
        // 条件
        self.register(TokenMetadata {
            token_type: "GreaterThan".to_string(),
            arguments: vec![
                ArgumentMetadata {
                    name: "left".to_string(),
                    expected_type: Numeric,
                    required: true,
                    default_value: None,
                },
                ArgumentMetadata {
                    name: "right".to_string(),
                    expected_type: Numeric,
                    required: true,
                    default_value: None,
                },
            ],
            output_type: Bool,
            custom_validator: None,
            output_type_inference: None,
            argument_context_provider: None,
        });
        
        self.register(TokenMetadata {
            token_type: "LessThan".to_string(),
            arguments: vec![
                ArgumentMetadata {
                    name: "left".to_string(),
                    expected_type: Numeric,
                    required: true,
                    default_value: None,
                },
                ArgumentMetadata {
                    name: "right".to_string(),
                    expected_type: Numeric,
                    required: true,
                    default_value: None,
                },
            ],
            output_type: Bool,
            custom_validator: None,
            output_type_inference: None,
            argument_context_provider: None,
        });
        
        // 値取得
        self.register(TokenMetadata {
            token_type: "ActingCharacter".to_string(),
            arguments: vec![],
            output_type: Character,
            custom_validator: None,
            output_type_inference: None,
            argument_context_provider: None,
        });
        
        // 配列操作
        self.register(TokenMetadata {
            token_type: "FilterList".to_string(),
            arguments: vec![
                ArgumentMetadata {
                    name: "array".to_string(),
                    expected_type: Vec(Box::new(Any)),
                    required: true,
                    default_value: None,
                },
                ArgumentMetadata {
                    name: "condition".to_string(),
                    expected_type: Bool,
                    required: true,
                    default_value: None,
                },
            ],
            output_type: Vec(Box::new(Any)),
            custom_validator: None,
            output_type_inference: Some(|arg_types| {
                // 配列の要素型を維持
                arg_types.get("array").cloned().unwrap_or(Vec(Box::new(Any)))
            }),
            argument_context_provider: Some(|arg_name, args| {
                if arg_name == "condition" {
                    // condition引数では、配列の要素型をElementのコンテキストとして提供
                    if let Some(array_ast) = args.get("array") {
                        if let Type::Vec(elem_type) = &array_ast.ty {
                            return Ok(Some((**elem_type).clone()));
                        }
                    }
                }
                Ok(None)
            }),
        });
        
        self.register(TokenMetadata {
            token_type: "Map".to_string(),
            arguments: vec![
                ArgumentMetadata {
                    name: "array".to_string(),
                    expected_type: Vec(Box::new(Any)),
                    required: true,
                    default_value: None,
                },
                ArgumentMetadata {
                    name: "transform".to_string(),
                    expected_type: Any,
                    required: true,
                    default_value: None,
                },
            ],
            output_type: Vec(Box::new(Any)),
            custom_validator: None,
            output_type_inference: Some(|arg_types| {
                // transformの出力型のVecを返す
                if let Some(transform_type) = arg_types.get("transform") {
                    Vec(Box::new(transform_type.clone()))
                } else {
                    Vec(Box::new(Any))
                }
            }),
            argument_context_provider: Some(|arg_name, args| {
                if arg_name == "transform" {
                    // transform引数では、配列の要素型をElementのコンテキストとして提供
                    if let Some(array_ast) = args.get("array") {
                        if let Type::Vec(elem_type) = &array_ast.ty {
                            return Ok(Some((**elem_type).clone()));
                        }
                    }
                }
                Ok(None)
            }),
        });
        
        // 特殊なトークン
        self.register(TokenMetadata {
            token_type: "Element".to_string(),
            arguments: vec![],
            output_type: Any, // コンテキストから推論される
            custom_validator: None,
            output_type_inference: Some(|_| {
                // Element の型は TypeChecker で特殊処理される
                Any
            }),
            argument_context_provider: None,
        });
        
        self.register(TokenMetadata {
            token_type: "Number".to_string(),
            arguments: vec![], // Numberはvalueをフィールドとして持つが、引数ではない
            output_type: I32,
            custom_validator: None,
            output_type_inference: None,
            argument_context_provider: None,
        });
        
        // 他のトークンも同様に登録...
        self.register_remaining_tokens();
    }
    
    /// 残りのトークンを登録
    fn register_remaining_tokens(&mut self) {
        use Type::*;
        
        // Check
        self.register(TokenMetadata {
            token_type: "Check".to_string(),
            arguments: vec![
                ArgumentMetadata {
                    name: "condition".to_string(),
                    expected_type: Bool,
                    required: true,
                    default_value: None,
                },
                ArgumentMetadata {
                    name: "then_action".to_string(),
                    expected_type: Action,
                    required: true,
                    default_value: None,
                },
            ],
            output_type: Action,
            custom_validator: None,
            output_type_inference: None,
            argument_context_provider: None,
        });
        
        // TrueOrFalseRandom
        self.register(TokenMetadata {
            token_type: "TrueOrFalseRandom".to_string(),
            arguments: vec![],
            output_type: Bool,
            custom_validator: None,
            output_type_inference: None,
            argument_context_provider: None,
        });
        
        // 比較演算
        self.register(TokenMetadata {
            token_type: "Eq".to_string(),
            arguments: vec![
                ArgumentMetadata {
                    name: "left".to_string(),
                    expected_type: Any,
                    required: true,
                    default_value: None,
                },
                ArgumentMetadata {
                    name: "right".to_string(),
                    expected_type: Any,
                    required: true,
                    default_value: None,
                },
            ],
            output_type: Bool,
            custom_validator: None,
            output_type_inference: None,
            argument_context_provider: None,
        });
        
        // TrueOrFalseRandom
        self.register(TokenMetadata {
            token_type: "TrueOrFalseRandom".to_string(),
            arguments: vec![],
            output_type: Bool,
            custom_validator: None,
            output_type_inference: None,
            argument_context_provider: None,
        });
        
        // 配列操作
        self.register(TokenMetadata {
            token_type: "AllCharacters".to_string(),
            arguments: vec![],
            output_type: Vec(Box::new(Character)),
            custom_validator: None,
            output_type_inference: None,
            argument_context_provider: None,
        });
        
        self.register(TokenMetadata {
            token_type: "AllTeamSides".to_string(),
            arguments: vec![],
            output_type: Vec(Box::new(TeamSide)),
            custom_validator: None,
            output_type_inference: None,
            argument_context_provider: None,
        });
        
        self.register(TokenMetadata {
            token_type: "TeamMembers".to_string(),
            arguments: vec![
                ArgumentMetadata {
                    name: "team_side".to_string(),
                    expected_type: TeamSide,
                    required: true,
                    default_value: None,
                },
            ],
            output_type: Vec(Box::new(Character)),
            custom_validator: None,
            output_type_inference: None,
            argument_context_provider: None,
        });
        
        self.register(TokenMetadata {
            token_type: "RandomPick".to_string(),
            arguments: vec![
                ArgumentMetadata {
                    name: "array".to_string(),
                    expected_type: Vec(Box::new(Any)),
                    required: true,
                    default_value: None,
                },
            ],
            output_type: Any,
            custom_validator: None,
            output_type_inference: Some(|arg_types| {
                // 配列の要素型を返す
                if let Some(Type::Vec(elem_type)) = arg_types.get("array") {
                    (**elem_type).clone()
                } else {
                    Any
                }
            }),
            argument_context_provider: None,
        });
        
        // 数値演算
        for op in &["Max", "Min", "NumericMax", "NumericMin"] {
            let is_numeric = op.starts_with("Numeric");
            let elem_type = if is_numeric { Numeric } else { Character };
            let output_type = if is_numeric { Numeric } else { Character };
            
            self.register(TokenMetadata {
                token_type: op.to_string(),
                arguments: vec![
                    ArgumentMetadata {
                        name: "array".to_string(),
                        expected_type: Vec(Box::new(elem_type)),
                        required: true,
                        default_value: None,
                    },
                ],
                output_type,
                custom_validator: None,
                output_type_inference: None,
                argument_context_provider: None,
            });
        }
        
        // キャラクター関連
        self.register(TokenMetadata {
            token_type: "CharacterToHp".to_string(),
            arguments: vec![
                ArgumentMetadata {
                    name: "character".to_string(),
                    expected_type: Character,
                    required: true,
                    default_value: None,
                },
            ],
            output_type: CharacterHP,
            custom_validator: None,
            output_type_inference: None,
            argument_context_provider: None,
        });
        
        self.register(TokenMetadata {
            token_type: "CharacterHpToCharacter".to_string(),
            arguments: vec![
                ArgumentMetadata {
                    name: "character_hp".to_string(),
                    expected_type: CharacterHP,
                    required: true,
                    default_value: None,
                },
            ],
            output_type: Character,
            custom_validator: None,
            output_type_inference: None,
            argument_context_provider: None,
        });
        
        self.register(TokenMetadata {
            token_type: "CharacterTeam".to_string(),
            arguments: vec![
                ArgumentMetadata {
                    name: "character".to_string(),
                    expected_type: Character,
                    required: true,
                    default_value: None,
                },
            ],
            output_type: TeamSide,
            custom_validator: None,
            output_type_inference: None,
            argument_context_provider: None,
        });
        
        // チーム関連
        self.register(TokenMetadata {
            token_type: "TeamMembers".to_string(),
            arguments: vec![
                ArgumentMetadata {
                    name: "team_side".to_string(),
                    expected_type: TeamSide,
                    required: true,
                    default_value: None,
                },
            ],
            output_type: Vec(Box::new(Character)),
            custom_validator: None,
            output_type_inference: None,
            argument_context_provider: None,
        });
        
        self.register(TokenMetadata {
            token_type: "AllTeamSides".to_string(),
            arguments: vec![],
            output_type: Vec(Box::new(TeamSide)),
            custom_validator: None,
            output_type_inference: None,
            argument_context_provider: None,
        });
        
        // CharacterHpToCharacter
        self.register(TokenMetadata {
            token_type: "CharacterHpToCharacter".to_string(),
            arguments: vec![
                ArgumentMetadata {
                    name: "character_hp".to_string(),
                    expected_type: CharacterHP,
                    required: true,
                    default_value: None,
                },
            ],
            output_type: Character,
            custom_validator: None,
            output_type_inference: None,
            argument_context_provider: None,
        });
        
        // 定数
        self.register(TokenMetadata {
            token_type: "Hero".to_string(),
            arguments: vec![],
            output_type: TeamSide,
            custom_validator: None,
            output_type_inference: None,
            argument_context_provider: None,
        });
        
        self.register(TokenMetadata {
            token_type: "Enemy".to_string(),
            arguments: vec![],
            output_type: TeamSide,
            custom_validator: None,
            output_type_inference: None,
            argument_context_provider: None,
        });
    }
    
    /// メタデータを登録
    pub fn register(&mut self, metadata: TokenMetadata) {
        self.metadata.insert(metadata.token_type.clone(), metadata);
    }
    
    /// メタデータを取得
    pub fn get(&self, token_type: &str) -> Option<&TokenMetadata> {
        self.metadata.get(token_type)
    }
}

impl Default for TokenMetadataRegistry {
    fn default() -> Self {
        Self::new()
    }
}