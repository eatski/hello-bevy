//! 型チェッカーの包括的なテスト

#[cfg(test)]
mod tests {
    use crate::compiler::{Compiler, ErrorReporter};
    use crate::structured_token::StructuredTokenInput;
    use crate::type_system::{TokenMetadataRegistry, TokenMetadata, ArgumentMetadata, Type, TypeChecker};
    
    // 基本的な型チェッカーテスト
    
    #[test]
    fn test_compiler_basic() {
        let compiler = Compiler::new();
        
        // 基本的なトークンのコンパイル
        let token = StructuredTokenInput::Strike {
            target: Box::new(StructuredTokenInput::ActingCharacter),
        };
        
        let result = compiler.compile(&token);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_metadata_driven_validation() {
        let compiler = Compiler::new();
        
        // メタデータ駆動の検証
        let invalid_token = StructuredTokenInput::Strike {
            target: Box::new(StructuredTokenInput::Number { value: 42 }),
        };
        
        let result = compiler.compile(&invalid_token);
        assert!(result.is_err());
        
        if let Err(e) = result {
            let report = ErrorReporter::format_error(&e);
            println!("Error report: {}", report);
            assert!(report.contains("Type mismatch") || report.contains("TypeMismatch"));
            assert!(report.contains("expected Character") || report.contains("Character"));
        }
    }
    
    #[test]
    fn test_complex_type_inference() {
        let type_checker = TypeChecker::new();
        
        // FilterListでのElement型推論
        let token = StructuredTokenInput::FilterList {
            array: Box::new(StructuredTokenInput::AllCharacters),
            condition: Box::new(StructuredTokenInput::GreaterThan {
                left: Box::new(StructuredTokenInput::CharacterToHp {
                    character: Box::new(StructuredTokenInput::Element),
                }),
                right: Box::new(StructuredTokenInput::Number { value: 50 }),
            }),
        };
        
        // 型チェックのみ
        let result = type_checker.check(&token);
        if let Err(e) = &result {
            let report = ErrorReporter::format_error(&e);
            println!("Error in test_complex_type_inference: {}", report);
        }
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_array_operation_type_inference() {
        let type_checker = TypeChecker::new();
        
        // Map操作での型推論
        let token = StructuredTokenInput::Map {
            array: Box::new(StructuredTokenInput::AllCharacters),
            transform: Box::new(StructuredTokenInput::CharacterToHp {
                character: Box::new(StructuredTokenInput::Element),
            }),
        };
        
        // このトークンはVec<CharacterHP>を生成するはず（型チェックのみ）
        let result = type_checker.check(&token);
        assert!(result.is_ok());
        if let Ok(typed_ast) = result {
            assert!(matches!(typed_ast.ty, crate::type_system::Type::Vec(_)));
        }
    }
    
    #[test]
    fn test_numeric_type_compatibility() {
        let type_checker = TypeChecker::new();
        
        // Numeric型の互換性テスト
        let token1 = StructuredTokenInput::GreaterThan {
            left: Box::new(StructuredTokenInput::CharacterToHp {
                character: Box::new(StructuredTokenInput::ActingCharacter),
            }),
            right: Box::new(StructuredTokenInput::Number { value: 100 }),
        };
        
        let result1 = type_checker.check(&token1);
        assert!(result1.is_ok());
        
        // 逆も可能
        let token2 = StructuredTokenInput::GreaterThan {
            left: Box::new(StructuredTokenInput::Number { value: 50 }),
            right: Box::new(StructuredTokenInput::CharacterToHp {
                character: Box::new(StructuredTokenInput::ActingCharacter),
            }),
        };
        
        let result2 = type_checker.check(&token2);
        assert!(result2.is_ok());
    }
    
    #[test]
    fn test_extensibility_with_custom_token() {
        // カスタムトークンの追加をシミュレート
        // 実際には、StructuredTokenInputにカスタムバリアントを追加する必要があるが、
        // ここではメタデータシステムの拡張性をテスト
        
        let mut registry = TokenMetadataRegistry::new();
        
        // 新しいトークンタイプを登録
        registry.register(TokenMetadata {
            token_type: "CustomMultiply".to_string(),
            arguments: vec![
                ArgumentMetadata {
                    name: "left".to_string(),
                    expected_type: Type::Numeric,
                    required: true,
                    default_value: None,
                },
                ArgumentMetadata {
                    name: "right".to_string(),
                    expected_type: Type::Numeric,
                    required: true,
                    default_value: None,
                },
            ],
            output_type: Type::Numeric,
            custom_validator: None,
            output_type_inference: None,
        });
        
        // レジストリに新しいトークンが登録されていることを確認
        assert!(registry.get("CustomMultiply").is_some());
    }
    
    #[test]
    fn test_compilation() {
        // コンパイラのテスト
        let compiler = Compiler::new();
        
        let token = StructuredTokenInput::Check {
            condition: Box::new(StructuredTokenInput::GreaterThan {
                left: Box::new(StructuredTokenInput::CharacterToHp {
                    character: Box::new(StructuredTokenInput::ActingCharacter),
                }),
                right: Box::new(StructuredTokenInput::Number { value: 50 }),
            }),
            then_action: Box::new(StructuredTokenInput::Strike {
                target: Box::new(StructuredTokenInput::RandomPick {
                    array: Box::new(StructuredTokenInput::AllCharacters),
                }),
            }),
        };
        
        let result = compiler.compile(&token);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_performance_on_deeply_nested_tokens() {
        let type_checker = TypeChecker::new();
        
        // 深くネストしたトークン構造
        let mut token = StructuredTokenInput::ActingCharacter;
        
        // 10レベルのネストを作成
        for i in 0..10 {
            token = StructuredTokenInput::CharacterToHp {
                character: Box::new(StructuredTokenInput::RandomPick {
                    array: Box::new(StructuredTokenInput::FilterList {
                        array: Box::new(StructuredTokenInput::AllCharacters),
                        condition: Box::new(StructuredTokenInput::GreaterThan {
                            left: Box::new(StructuredTokenInput::Number { value: i }),
                            right: Box::new(StructuredTokenInput::Number { value: 0 }),
                        }),
                    }),
                }),
            };
        }
        
        // 深いネストでも問題なく型チェックできることを確認
        let result = type_checker.check(&token);
        assert!(result.is_ok());
    }
}