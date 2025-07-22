//! 高度な型システムのテスト (Hindley-Milner型推論、Traitシステムなど)

#[cfg(test)]
mod tests {
    use crate::compiler::ErrorReporter;
    use crate::structured_token::StructuredTokenInput;
    use crate::type_system::{Type, AdvancedTypeChecker};
    
    #[test]
    fn test_hindley_milner_type_inference() {
        let mut checker = AdvancedTypeChecker::new();
        
        // 型推論のテスト: FilterList の出力型が正しく推論される
        let token = StructuredTokenInput::FilterList {
            array: Box::new(StructuredTokenInput::Map {
                array: Box::new(StructuredTokenInput::AllCharacters),
                transform: Box::new(StructuredTokenInput::CharacterToHp {
                    character: Box::new(StructuredTokenInput::Element),
                }),
            }),
            condition: Box::new(StructuredTokenInput::GreaterThan {
                left: Box::new(StructuredTokenInput::Element),
                right: Box::new(StructuredTokenInput::Number { value: 50 }),
            }),
        };
        
        let result = checker.check(&token);
        if let Err(e) = &result {
            eprintln!("Error in test_hindley_milner_type_inference: {:?}", e);
        }
        assert!(result.is_ok());
        
        let typed_ast = result.unwrap();
        // FilterList(Map(Vec<Character> -> Vec<CharacterHP>)) -> Vec<CharacterHP>
        assert_eq!(typed_ast.ty, Type::Vec(Box::new(Type::CharacterHP)));
    }
    
    #[test]
    fn test_trait_constraint_checking() {
        let mut checker = AdvancedTypeChecker::new();
        
        // Numeric trait制約のテスト
        // NumericMaxはNumeric型の配列を要求
        let valid_token = StructuredTokenInput::NumericMax {
            array: Box::new(StructuredTokenInput::Map {
                array: Box::new(StructuredTokenInput::AllCharacters),
                transform: Box::new(StructuredTokenInput::CharacterToHp {
                    character: Box::new(StructuredTokenInput::Element),
                }),
            }),
        };
        
        let result = checker.check(&valid_token);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().ty, Type::Numeric);
        
        // 非Numeric型でNumericMaxを使うとエラー
        let invalid_token = StructuredTokenInput::NumericMax {
            array: Box::new(StructuredTokenInput::AllCharacters), // Vec<Character>はNumericではない
        };
        
        let result = checker.check(&invalid_token);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_polymorphic_type_instantiation() {
        let mut checker = AdvancedTypeChecker::new();
        
        // 同じ多相関数（RandomPick）を異なる型で使用
        
        // RandomPick<Character>
        let pick_character = StructuredTokenInput::RandomPick {
            array: Box::new(StructuredTokenInput::AllCharacters),
        };
        
        let result1 = checker.check(&pick_character);
        if let Err(e) = &result1 {
            eprintln!("Error in test_polymorphic_type_instantiation (1): {:?}", e);
        }
        assert!(result1.is_ok());
        assert_eq!(result1.unwrap().ty, Type::Character);
        
        // RandomPick<TeamSide>
        let pick_team = StructuredTokenInput::RandomPick {
            array: Box::new(StructuredTokenInput::AllTeamSides),
        };
        
        let result2 = checker.check(&pick_team);
        assert!(result2.is_ok());
        assert_eq!(result2.unwrap().ty, Type::TeamSide);
    }
    
    #[test]
    fn test_complex_type_inference_with_traits() {
        let mut checker = AdvancedTypeChecker::new();
        
        // 複雑な型推論: 
        // 1. AllCharactersから開始 (Vec<Character>)
        // 2. Mapでhpに変換 (Vec<CharacterHP>)
        // 3. FilterListで50以上のみ (Vec<CharacterHP>, CharacterHPはNumeric)
        // 4. NumericMaxで最大値 (Numeric)
        let complex_token = StructuredTokenInput::NumericMax {
            array: Box::new(StructuredTokenInput::FilterList {
                array: Box::new(StructuredTokenInput::Map {
                    array: Box::new(StructuredTokenInput::AllCharacters),
                    transform: Box::new(StructuredTokenInput::CharacterToHp {
                        character: Box::new(StructuredTokenInput::Element),
                    }),
                }),
                condition: Box::new(StructuredTokenInput::GreaterThan {
                    left: Box::new(StructuredTokenInput::Element),
                    right: Box::new(StructuredTokenInput::Number { value: 50 }),
                }),
            }),
        };
        
        let result = checker.check(&complex_token);
        assert!(result.is_ok());
        
        let typed_ast = result.unwrap();
        // 最終的な型はNumeric（CharacterHPの抽象化）
        assert_eq!(typed_ast.ty, Type::Numeric);
    }
    
    #[test]
    fn test_element_type_context_propagation() {
        let mut checker = AdvancedTypeChecker::new();
        
        // ネストしたMap/FilterListでのElement型の伝播
        let nested = StructuredTokenInput::Map {
            array: Box::new(StructuredTokenInput::FilterList {
                array: Box::new(StructuredTokenInput::AllCharacters),
                condition: Box::new(StructuredTokenInput::GreaterThan {
                    left: Box::new(StructuredTokenInput::CharacterToHp {
                        character: Box::new(StructuredTokenInput::Element), // Character型として解決
                    }),
                    right: Box::new(StructuredTokenInput::Number { value: 30 }),
                }),
            }),
            transform: Box::new(StructuredTokenInput::CharacterTeam {
                character: Box::new(StructuredTokenInput::Element), // Character型として解決
            }),
        };
        
        let result = checker.check(&nested);
        assert!(result.is_ok());
        
        // Map(FilterList(Vec<Character>)) -> Vec<TeamSide>
        assert_eq!(result.unwrap().ty, Type::Vec(Box::new(Type::TeamSide)));
    }
    
    #[test]
    fn test_generic_type_error_messages() {
        let mut checker = AdvancedTypeChecker::new();
        
        // 型エラーの詳細なメッセージ
        let error_token = StructuredTokenInput::Strike {
            target: Box::new(StructuredTokenInput::Number { value: 42 }), // Characterが必要
        };
        
        let result = checker.check(&error_token);
        assert!(result.is_err());
        
        if let Err(e) = result {
            let report = ErrorReporter::format_error(&e);
            println!("Error report: {}", report);
            
            // エラーメッセージに型情報が含まれることを確認
            assert!(report.contains("Character") || report.contains("expected"));
            // I32として表示される
            assert!(report.contains("I32") || report.contains("i32") || report.contains("Number"));
        }
    }
    
    #[test]
    fn test_higher_order_type_inference() {
        let mut checker = AdvancedTypeChecker::new();
        
        // Map の中で Map を使う（高階型の推論）
        let higher_order = StructuredTokenInput::Map {
            array: Box::new(StructuredTokenInput::AllTeamSides),
            transform: Box::new(StructuredTokenInput::TeamMembers {
                team_side: Box::new(StructuredTokenInput::Element), // TeamSide型として解決
            }),
        };
        
        let result = checker.check(&higher_order);
        assert!(result.is_ok());
        
        // Map(Vec<TeamSide>, TeamSide -> Vec<Character>) -> Vec<Vec<Character>>
        assert_eq!(
            result.unwrap().ty, 
            Type::Vec(Box::new(Type::Vec(Box::new(Type::Character))))
        );
    }
}