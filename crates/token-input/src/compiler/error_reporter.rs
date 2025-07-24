//! エラーレポート機能
//! 
//! コンパイルエラーを分かりやすく表示

use crate::type_system::{CompileError, TypeError};
use crate::structured_token::StructuredTokenInput;
use std::fmt::Write as _;

/// エラーレポーター
pub struct ErrorReporter;

impl ErrorReporter {
    /// エラーを人間が読みやすい形式でフォーマット
    pub fn format_error(error: &CompileError) -> String {
        let mut output = String::new();
        
        // エラーのヘッダー
        writeln!(&mut output, "Compilation Error").unwrap();
        writeln!(&mut output, "=================").unwrap();
        writeln!(&mut output).unwrap();
        
        // エラーメッセージ
        writeln!(&mut output, "Error: {}", error.error).unwrap();
        
        // パス情報があれば表示
        if !error.path.is_empty() {
            writeln!(&mut output).unwrap();
            writeln!(&mut output, "Location: {}", error.path.join(" → ")).unwrap();
        }
        
        // トークン情報があれば表示
        if let Some(token) = &error.token {
            writeln!(&mut output).unwrap();
            writeln!(&mut output, "Token:").unwrap();
            writeln!(&mut output, "{}", Self::format_token(token, 2)).unwrap();
        }
        
        // エラーに応じた提案を表示
        writeln!(&mut output).unwrap();
        writeln!(&mut output, "Suggestion:").unwrap();
        writeln!(&mut output, "{}", Self::get_suggestion(&error.error)).unwrap();
        
        output
    }
    
    /// 複数のエラーをフォーマット
    pub fn format_errors(errors: &[CompileError]) -> String {
        let mut output = String::new();
        
        writeln!(&mut output, "Found {} compilation error(s):", errors.len()).unwrap();
        writeln!(&mut output).unwrap();
        
        for (i, error) in errors.iter().enumerate() {
            if i > 0 {
                writeln!(&mut output, "\n---\n").unwrap();
            }
            write!(&mut output, "{}", Self::format_error(error)).unwrap();
        }
        
        output
    }
    
    /// トークンを読みやすい形式でフォーマット
    fn format_token(token: &StructuredTokenInput, indent: usize) -> String {
        let mut output = String::new();
        let indent_str = " ".repeat(indent);
        
        let token_type = token.token_type();
        let args = token.arguments();
        
        // 引数がない場合はシンプルな表示
        if args.is_empty() {
            // 特殊なフィールドがあるトークン（Number）の処理
            if let Some(value) = token.get_number_value() {
                write!(&mut output, "{}{} {{ value: {} }}", indent_str, token_type, value).unwrap();
            } else {
                write!(&mut output, "{}{}", indent_str, token_type).unwrap();
            }
        } else {
            // 引数がある場合は構造化表示
            writeln!(&mut output, "{}{} {{", indent_str, token_type).unwrap();
            for (arg_name, arg_value) in args {
                writeln!(&mut output, "{}  {}: {}", indent_str, arg_name, Self::format_token(arg_value, indent + 4).trim()).unwrap();
            }
            write!(&mut output, "{}}}", indent_str).unwrap();
        }
        
        output
    }
    
    /// エラータイプに応じた提案を生成
    fn get_suggestion(error: &TypeError) -> String {
        match error {
            TypeError::TypeMismatch { expected, actual, context } => {
                format!(
                    "The {} expects a value of type '{}', but you provided type '{}'.\n  \
                     Please ensure the argument matches the expected type.",
                    context, expected, actual
                )
            }
            TypeError::UndefinedToken { token_type } => {
                format!(
                    "The token '{}' is not recognized. Available tokens include:\n  \
                     - Actions: Strike, Heal, Check\n  \
                     - Conditions: GreaterThan, Eq, TrueOrFalseRandom\n  \
                     - Values: ActingCharacter, Number, Hero, Enemy, Element\n  \
                     - Arrays: AllCharacters, FilterList, RandomPick, Map, TeamMembers\n  \
                     - Numeric: NumericMax, NumericMin, Max, Min",
                    token_type
                )
            }
            TypeError::ArgumentCountMismatch { token_type, expected, actual } => {
                format!(
                    "The token '{}' requires exactly {} argument(s), but {} were provided.\n  \
                     Please check the token documentation for the correct number of arguments.",
                    token_type, expected, actual
                )
            }
            TypeError::MissingField { token_type, field_name } => {
                format!(
                    "The token '{}' is missing the required field '{}'.\n  \
                     Please add this field with an appropriate value.",
                    token_type, field_name
                )
            }
            TypeError::UnresolvedType { context } => {
                format!(
                    "Type resolution failed: {}\n  \
                     This may indicate a bug in the type system or an unsupported operation.",
                    context
                )
            }
            TypeError::CyclicReference { token_type } => {
                format!(
                    "The token '{}' contains a cyclic reference.\n  \
                     Tokens cannot reference themselves directly or indirectly.",
                    token_type
                )
            }
            TypeError::InferenceError { kind, types, location } => {
                format!(
                    "Type inference failed: {:?}\n  \
                     Location: {}\n  \
                     Related types: {:?}",
                    kind,
                    if location.is_empty() { "root".to_string() } else { location.join(" -> ") },
                    types
                )
            }
            TypeError::TraitBoundError { ty, trait_name, available_traits: _ } => {
                format!(
                    "Type {} does not implement the required trait {}.\n  \
                     This type cannot be used in this context.",
                    ty, trait_name
                )
            }
        }
    }
    
    /// エラーを簡潔な1行形式でフォーマット（リスト表示用）
    pub fn format_error_oneline(error: &CompileError) -> String {
        let location = if error.path.is_empty() {
            String::new()
        } else {
            format!(" at {}", error.path.join(" → "))
        };
        
        format!("{}{}", error.error, location)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::type_system::Type;
    
    #[test]
    fn test_format_type_mismatch_error() {
        let error = CompileError::new(TypeError::TypeMismatch {
            expected: Type::Character,
            actual: Type::I32,
            context: "Strike.target".to_string(),
        });
        
        let formatted = ErrorReporter::format_error(&error);
        assert!(formatted.contains("expected Character, but got i32"));
        assert!(formatted.contains("Strike.target"));
    }
    
    #[test]
    fn test_format_with_path() {
        let error = CompileError::new(TypeError::UndefinedToken {
            token_type: "InvalidToken".to_string(),
        }).with_path(vec!["Rule1".to_string(), "condition".to_string()]);
        
        let formatted = ErrorReporter::format_error(&error);
        assert!(formatted.contains("Location: Rule1 → condition"));
    }
    
    #[test]
    fn test_format_with_token() {
        let token = StructuredTokenInput::Strike {
            target: Box::new(StructuredTokenInput::ActingCharacter),
        };
        
        let error = CompileError::new(TypeError::TypeMismatch {
            expected: Type::Character,
            actual: Type::Any,
            context: "Strike.target".to_string(),
        }).with_token(token);
        
        let formatted = ErrorReporter::format_error(&error);
        assert!(formatted.contains("Strike {"));
        assert!(formatted.contains("target: ActingCharacter"));
    }
    
    #[test]
    fn test_suggestions() {
        let error = CompileError::new(TypeError::UndefinedToken {
            token_type: "Atack".to_string(),
        });
        
        let formatted = ErrorReporter::format_error(&error);
        assert!(formatted.contains("Available tokens include:"));
        assert!(formatted.contains("Strike"));
    }
}