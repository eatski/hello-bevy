//! コンパイラシステムの統合テスト

#[cfg(test)]
mod tests {
    use crate::compiler::{Compiler, CompilerOptions, ErrorReporter};
    use crate::structured_token::StructuredTokenInput;
    use crate::type_system::{CompileError, TypeError, Type};
    
    #[test]
    fn test_compile_with_type_checking() {
        let mut compiler = Compiler::new();
        
        // 正しい型のトークン
        let valid_token = StructuredTokenInput::Strike {
            target: Box::new(StructuredTokenInput::ActingCharacter),
        };
        
        let result = compiler.compile(&valid_token);
        assert!(result.is_ok());
        
        // 型が合わないトークン（Strikeは数値を受け取れない）
        let invalid_token = StructuredTokenInput::Strike {
            target: Box::new(StructuredTokenInput::Number { value: 42 }),
        };
        
        let result = compiler.compile(&invalid_token);
        assert!(result.is_err());
        
        if let Err(error) = result {
            match error.error {
                TypeError::TypeMismatch { expected, actual, .. } => {
                    assert_eq!(expected, Type::Character);
                    assert_eq!(actual, Type::I32);
                }
                _ => panic!("Expected TypeMismatch error"),
            }
        }
    }
    
    
    #[test]
    fn test_error_reporting() {
        let mut compiler = Compiler::new();
        
        // 未定義のトークンを作成（テスト用に無効なトークンを作成）
        let invalid_token = StructuredTokenInput::Check {
            condition: Box::new(StructuredTokenInput::ActingCharacter), // ActingCharacterはboolを返さない
            then_action: Box::new(StructuredTokenInput::Strike {
                target: Box::new(StructuredTokenInput::ActingCharacter),
            }),
        };
        
        let result = compiler.compile(&invalid_token);
        assert!(result.is_err());
        
        if let Err(error) = result {
            let formatted = ErrorReporter::format_error(&error);
            println!("Error report:\n{}", formatted);
            
            // エラーレポートに必要な情報が含まれているか確認
            assert!(formatted.contains("Type mismatch"));
            assert!(formatted.contains("expected bool"));
            assert!(formatted.contains("Character"));
        }
    }
    
    #[test]
    fn test_complex_token_compilation() {
        let mut compiler = Compiler::new();
        
        // 複雑なトークンツリー
        let complex_token = StructuredTokenInput::Check {
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
        
        let result = compiler.compile(&complex_token);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_numeric_type_compatibility() {
        let mut compiler = Compiler::with_options(CompilerOptions { debug: true });
        
        // NumericMax/Minトークンのテスト（配列形式）
        // このテストでは実際にはCharacterHPを返すノードを作成しているが、
        // 現在のCodeGeneratorはAction型しか処理できないため、
        // Actionの中でNumericMaxを使うテストに変更
        let action_with_numeric_max = StructuredTokenInput::Check {
            condition: Box::new(StructuredTokenInput::GreaterThan {
                left: Box::new(StructuredTokenInput::NumericMax {
                    array: Box::new(StructuredTokenInput::Map {
                        array: Box::new(StructuredTokenInput::AllCharacters),
                        transform: Box::new(StructuredTokenInput::CharacterToHp {
                            character: Box::new(StructuredTokenInput::Element),
                        }),
                    }),
                }),
                right: Box::new(StructuredTokenInput::Number { value: 50 }),
            }),
            then_action: Box::new(StructuredTokenInput::Strike {
                target: Box::new(StructuredTokenInput::ActingCharacter),
            }),
        };
        
        let result = compiler.compile(&action_with_numeric_max);
        if let Err(ref e) = result {
            eprintln!("NumericMax compilation failed: {:?}", e);
        }
        assert!(result.is_ok());
        
        // 通常のMaxとの互換性テスト（Actionの中で使用）
        let action_with_regular_max = StructuredTokenInput::Strike {
            target: Box::new(StructuredTokenInput::Max {
                array: Box::new(StructuredTokenInput::AllCharacters),
            }),
        };
        
        let result = compiler.compile(&action_with_regular_max);
        if let Err(ref e) = result {
            eprintln!("Regular Max compilation failed: {:?}", e);
        }
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_multiple_errors() {
        let mut compiler = Compiler::new();
        
        // 複数のエラーを含むトークン
        let tokens = vec![
            // 型エラー
            StructuredTokenInput::Strike {
                target: Box::new(StructuredTokenInput::Number { value: 42 }),
            },
            // 別の型エラー
            StructuredTokenInput::Heal {
                target: Box::new(StructuredTokenInput::AllCharacters), // Healは単一のCharacterを期待
            },
        ];
        
        let errors: Vec<CompileError> = tokens.iter()
            .filter_map(|token| compiler.compile(token).err())
            .collect();
        
        assert_eq!(errors.len(), 2);
        
        let formatted = ErrorReporter::format_errors(&errors);
        println!("Multiple errors:\n{}", formatted);
        
        assert!(formatted.contains("Found 2 compilation error(s)"));
    }
    
    #[test]
    fn test_debug_mode() {
        let mut compiler = Compiler::with_options(CompilerOptions {
            debug: true,
        });
        
        // ActingCharacterはCharacter型を返すので、Actionの中で使用する
        let token = StructuredTokenInput::Strike {
            target: Box::new(StructuredTokenInput::ActingCharacter),
        };
        
        // デバッグモードではstderrに出力される
        let result = compiler.compile(&token);
        assert!(result.is_ok());
    }
}