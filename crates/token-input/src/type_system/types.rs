//! 型定義
//! 
//! 基本型、ゲーム固有型、抽象型の定義

use std::fmt;
use std::collections::HashMap;
use crate::structured_token::StructuredTokenInput;

/// 型を表すenum
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    // プリミティブ型
    I32,
    Bool,
    String,
    
    // ゲーム固有型
    Character,
    Team,
    CharacterHP,
    TeamSide,
    
    // コレクション型
    Vec(Box<Type>),
    Option(Box<Type>),
    
    // トレイト/抽象型
    Numeric,     // i32またはCharacterHPを受け入れる
    Action,      // Actionトレイトを実装している型
    Condition,   // boolを返す条件
    
    // 特殊型
    Void,        // 何も返さない
    Any,         // 任意の型（型推論前の状態）
}

impl Type {
    /// 型が別の型と互換性があるかチェック
    pub fn is_compatible_with(&self, other: &Type) -> bool {
        match (self, other) {
            // 同じ型は常に互換
            (a, b) if a == b => true,
            
            // Numericは i32 と CharacterHP を受け入れる
            (Type::I32, Type::Numeric) | (Type::CharacterHP, Type::Numeric) => true,
            (Type::Numeric, Type::I32) | (Type::Numeric, Type::CharacterHP) => true,
            
            // Any は任意の型と互換（型推論前の状態）
            (Type::Any, _) | (_, Type::Any) => true,
            
            // Vecの要素型チェック
            (Type::Vec(a), Type::Vec(b)) => a.is_compatible_with(b),
            
            // Optionの要素型チェック
            (Type::Option(a), Type::Option(b)) => a.is_compatible_with(b),
            
            _ => false,
        }
    }
    
    /// 具体的な型へ解決（Numeric -> I32 または CharacterHP など）
    pub fn resolve_to_concrete(&self, hint: Option<&Type>) -> Type {
        match self {
            Type::Numeric => {
                // ヒントがあればそれを使用、なければデフォルトでi32
                match hint {
                    Some(Type::CharacterHP) => Type::CharacterHP,
                    _ => Type::I32,
                }
            }
            Type::Any => hint.cloned().unwrap_or(Type::Void),
            _ => self.clone(),
        }
    }
    
    /// 数値型かどうかをチェック
    pub fn is_numeric(&self) -> bool {
        matches!(self, Type::I32 | Type::CharacterHP | Type::Numeric)
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::I32 => write!(f, "i32"),
            Type::Bool => write!(f, "bool"),
            Type::String => write!(f, "String"),
            Type::Character => write!(f, "Character"),
            Type::Team => write!(f, "Team"),
            Type::CharacterHP => write!(f, "CharacterHP"),
            Type::TeamSide => write!(f, "TeamSide"),
            Type::Vec(t) => write!(f, "Vec<{}>", t),
            Type::Option(t) => write!(f, "Option<{}>", t),
            Type::Numeric => write!(f, "Numeric"),
            Type::Action => write!(f, "Action"),
            Type::Condition => write!(f, "Condition"),
            Type::Void => write!(f, "void"),
            Type::Any => write!(f, "any"),
        }
    }
}

/// トークンの型シグネチャ
#[derive(Debug, Clone)]
pub struct TypeSignature {
    /// 入力引数の型
    pub inputs: Vec<(String, Type)>,  // (引数名, 型)
    /// 出力の型
    pub output: Type,
}

impl TypeSignature {
    pub fn new(inputs: Vec<(String, Type)>, output: Type) -> Self {
        Self { inputs, output }
    }
    
    /// 引数なしのシグネチャ
    pub fn nullary(output: Type) -> Self {
        Self {
            inputs: vec![],
            output,
        }
    }
    
    /// 1引数のシグネチャ
    pub fn unary(input_name: &str, input_type: Type, output: Type) -> Self {
        Self {
            inputs: vec![(input_name.to_string(), input_type)],
            output,
        }
    }
    
    /// 2引数のシグネチャ
    pub fn binary(
        input1_name: &str, input1_type: Type,
        input2_name: &str, input2_type: Type,
        output: Type
    ) -> Self {
        Self {
            inputs: vec![
                (input1_name.to_string(), input1_type),
                (input2_name.to_string(), input2_type),
            ],
            output,
        }
    }
}

/// 型付きAST（型チェック済みのトークン）
#[derive(Debug, Clone)]
pub struct TypedAst {
    /// 元のトークン
    pub token: StructuredTokenInput,
    /// 推論された型
    pub ty: Type,
    /// 子ノードの型付きAST
    pub children: HashMap<String, TypedAst>,
}

impl TypedAst {
    /// 新しいTypedAstを作成
    pub fn new(token: StructuredTokenInput, ty: Type) -> Self {
        Self {
            token,
            ty,
            children: HashMap::new(),
        }
    }
}

/// 型チェックのコンテキスト
#[derive(Debug, Clone)]
pub struct TypeContext {
    /// 現在のコンテキスト（FilterList/Map内でのElement型など）
    current_context: Option<Type>,
}

impl TypeContext {
    /// 新しいコンテキストを作成
    pub fn new() -> Self {
        Self {
            current_context: None,
        }
    }
    
    /// 現在のコンテキストを取得
    pub fn current_context(&self) -> Option<&Type> {
        self.current_context.as_ref()
    }
    
    /// 現在のコンテキストを設定
    pub fn set_current_context(&mut self, ty: Option<Type>) {
        self.current_context = ty;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_type_compatibility() {
        assert!(Type::I32.is_compatible_with(&Type::I32));
        assert!(Type::I32.is_compatible_with(&Type::Numeric));
        assert!(Type::CharacterHP.is_compatible_with(&Type::Numeric));
        assert!(!Type::I32.is_compatible_with(&Type::Bool));
        
        let vec_i32 = Type::Vec(Box::new(Type::I32));
        let vec_numeric = Type::Vec(Box::new(Type::Numeric));
        assert!(vec_i32.is_compatible_with(&vec_numeric));
    }
    
    #[test]
    fn test_type_resolution() {
        assert_eq!(Type::Numeric.resolve_to_concrete(None), Type::I32);
        assert_eq!(
            Type::Numeric.resolve_to_concrete(Some(&Type::CharacterHP)),
            Type::CharacterHP
        );
    }
}