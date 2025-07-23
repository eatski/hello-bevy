//! コンパイルパイプライン
//! 
//! StructuredTokenInput → 型検査 → コード生成の一連の流れ

use action_system::RuleNode;
use crate::structured_token::StructuredTokenInput;
use crate::type_system::{TypeChecker, CompileResult};
use super::typed_code_generator::TypedCodeGenerator;

/// コンパイラオプション
#[derive(Debug, Clone)]
pub struct CompilerOptions {
    /// デバッグ情報を出力するか
    pub debug: bool,
}

impl Default for CompilerOptions {
    fn default() -> Self {
        Self {
            debug: false,
        }
    }
}

/// コンパイラ
pub struct Compiler {
    type_checker: TypeChecker,
    typed_code_generator: TypedCodeGenerator,
    options: CompilerOptions,
}

impl Compiler {
    /// デフォルト設定でコンパイラを作成
    pub fn new() -> Self {
        Self::with_options(CompilerOptions::default())
    }
    
    
    /// オプション指定でコンパイラを作成
    pub fn with_options(options: CompilerOptions) -> Self {
        let type_checker = TypeChecker::new();
        let typed_code_generator = TypedCodeGenerator::new();
        
        Self {
            type_checker,
            typed_code_generator,
            options,
        }
    }
    
    /// StructuredTokenInputをコンパイル
    pub fn compile(&self, token: &StructuredTokenInput) -> CompileResult<RuleNode> {
        if self.options.debug {
            eprintln!("Compiling token: {:?}", token);
        }
        
        // Phase 1: 型検査
        let typed_ast = self.type_checker.check(token)?;
        
        if self.options.debug {
            eprintln!("Type checked AST: {:?}", typed_ast);
        }
        
        // Phase 2: コード生成
        if self.options.debug {
            eprintln!("Using typed code generator (type-propagating system)");
        }
        let node = self.typed_code_generator.generate(&typed_ast)?;
        
        if self.options.debug {
            eprintln!("Generated node successfully");
        }
        
        Ok(node)
    }
    
    /// 複数のトークンをコンパイル
    pub fn compile_many(&self, tokens: &[StructuredTokenInput]) -> CompileResult<Vec<RuleNode>> {
        tokens.iter()
            .map(|token| self.compile(token))
            .collect()
    }
}

impl Default for Compiler {
    fn default() -> Self {
        Self::new()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    
    fn create_simple_token() -> StructuredTokenInput {
        StructuredTokenInput::ActingCharacter
    }
    
    fn create_strike_token() -> StructuredTokenInput {
        StructuredTokenInput::Strike {
            target: Box::new(StructuredTokenInput::ActingCharacter),
        }
    }
    
    #[test]
    fn test_compile_simple_token() {
        let compiler = Compiler::new();
        let token = create_simple_token();
        
        // 型チェックありでコンパイル - ActingCharacterはAction型ではないのでエラーになるはず
        let result = compiler.compile(&token);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_compile_strike_token() {
        let compiler = Compiler::new();
        let token = create_strike_token();
        
        // Strikeトークンは正常にコンパイルできるはず
        let result = compiler.compile(&token);
        assert!(result.is_ok());
    }
    
    
    #[test]
    fn test_compile_complex_token() {
        let compiler = Compiler::new();
        
        let token = StructuredTokenInput::Check {
            condition: Box::new(StructuredTokenInput::TrueOrFalseRandom),
            then_action: Box::new(StructuredTokenInput::Strike {
                target: Box::new(StructuredTokenInput::ActingCharacter),
            }),
        };
        
        let result = compiler.compile(&token);
        assert!(result.is_ok());
    }
}