//! 型推論エンジン
//! 
//! ジェネリック型の解決とコンテキスト依存の型推論

use super::{Type, TypeContext};
use std::collections::HashMap;

/// 型変数（型推論で使用）
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeVariable(String);

impl TypeVariable {
    pub fn new(name: &str) -> Self {
        Self(name.to_string())
    }
}

/// 型制約
#[derive(Debug, Clone)]
pub enum TypeConstraint {
    /// 型が等しい
    Equal(Type, Type),
    /// 型が互換性がある
    Compatible(Type, Type),
    /// 型がコレクションの要素型
    ElementOf(Type, Type), // ElementOf(elem_type, collection_type)
}

/// 型推論エンジン
pub struct TypeInferenceEngine {
    /// 型変数の代入
    substitutions: HashMap<TypeVariable, Type>,
    /// 型制約のリスト
    constraints: Vec<TypeConstraint>,
}

impl TypeInferenceEngine {
    pub fn new() -> Self {
        Self {
            substitutions: HashMap::new(),
            constraints: Vec::new(),
        }
    }
    
    /// 制約を追加
    pub fn add_constraint(&mut self, constraint: TypeConstraint) {
        self.constraints.push(constraint);
    }
    
    /// 型を統一（unify）
    pub fn unify(&mut self, t1: &Type, t2: &Type) -> Result<Type, String> {
        match (t1, t2) {
            // 同じ型
            (a, b) if a == b => Ok(a.clone()),
            
            // Any型は任意の型と統一可能
            (Type::Any, other) | (other, Type::Any) => Ok(other.clone()),
            
            // Numericは i32 と CharacterHP と統一可能
            (Type::Numeric, Type::I32) | (Type::I32, Type::Numeric) => Ok(Type::I32),
            (Type::Numeric, Type::CharacterHP) | (Type::CharacterHP, Type::Numeric) => Ok(Type::CharacterHP),
            
            // Vec型の統一
            (Type::Vec(a), Type::Vec(b)) => {
                let elem_type = self.unify(a, b)?;
                Ok(Type::Vec(Box::new(elem_type)))
            }
            
            // Option型の統一
            (Type::Option(a), Type::Option(b)) => {
                let inner_type = self.unify(a, b)?;
                Ok(Type::Option(Box::new(inner_type)))
            }
            
            // 統一できない
            _ => Err(format!("Cannot unify types {} and {}", t1, t2)),
        }
    }
    
    /// 制約を解決
    pub fn solve(&mut self) -> Result<(), String> {
        let constraints = std::mem::take(&mut self.constraints);
        
        for constraint in constraints {
            match constraint {
                TypeConstraint::Equal(t1, t2) => {
                    self.unify(&t1, &t2)?;
                }
                TypeConstraint::Compatible(t1, t2) => {
                    if !t1.is_compatible_with(&t2) {
                        return Err(format!("Type {} is not compatible with {}", t1, t2));
                    }
                }
                TypeConstraint::ElementOf(elem, collection) => {
                    match collection {
                        Type::Vec(expected_elem) => {
                            self.unify(&elem, &expected_elem)?;
                        }
                        _ => {
                            return Err(format!("Type {} is not a collection", collection));
                        }
                    }
                }
            }
        }
        
        Ok(())
    }
    
    /// コンテキストから型を推論
    pub fn infer_from_context(context: &TypeContext, token_type: &str) -> Option<Type> {
        match token_type {
            "Element" => {
                // FilterList/Map内でのElement型
                context.current_context().cloned()
            }
            _ => None,
        }
    }
    
    /// 配列操作の出力型を推論
    pub fn infer_array_operation_type(
        operation: &str,
        input_type: &Type,
        transform_type: Option<&Type>,
    ) -> Type {
        match operation {
            "FilterList" => {
                // フィルタリングは入力型を維持
                input_type.clone()
            }
            "Map" => {
                // マッピングは変換関数の出力型のVecを返す
                if let Some(transform) = transform_type {
                    Type::Vec(Box::new(transform.clone()))
                } else {
                    input_type.clone()
                }
            }
            "RandomPick" => {
                // ランダム選択は要素型を返す
                if let Type::Vec(elem_type) = input_type {
                    (**elem_type).clone()
                } else {
                    Type::Any
                }
            }
            _ => input_type.clone(),
        }
    }
    
    /// 数値演算の出力型を推論
    pub fn infer_numeric_operation_type(
        operation: &str,
        left_type: &Type,
        _right_type: Option<&Type>,
    ) -> Type {
        match operation {
            "Max" | "Min" => {
                // 入力型に基づいて出力型を決定
                match left_type {
                    Type::Vec(elem) => match elem.as_ref() {
                        Type::I32 => Type::I32,
                        Type::CharacterHP => Type::CharacterHP,
                        Type::Character => Type::Character,
                        Type::Numeric => Type::Numeric,
                        _ => Type::Any,
                    }
                    _ => Type::Any,
                }
            }
            "NumericMax" | "NumericMin" => {
                // 常にNumeric型を返す
                Type::Numeric
            }
            _ => Type::Any,
        }
    }
}

impl Default for TypeInferenceEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// 型環境（変数と型のマッピング）
#[derive(Debug, Clone)]
pub struct TypeEnvironment {
    bindings: HashMap<String, Type>,
}

impl TypeEnvironment {
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
        }
    }
    
    /// 変数を束縛
    pub fn bind(&mut self, name: String, ty: Type) {
        self.bindings.insert(name, ty);
    }
    
    /// 変数の型を取得
    pub fn lookup(&self, name: &str) -> Option<&Type> {
        self.bindings.get(name)
    }
    
    /// 新しいスコープを作成
    pub fn with_scope<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut TypeEnvironment) -> R,
    {
        let mut new_env = self.clone();
        f(&mut new_env)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_type_unification() {
        let mut engine = TypeInferenceEngine::new();
        
        // 同じ型の統一
        assert_eq!(
            engine.unify(&Type::I32, &Type::I32).unwrap(),
            Type::I32
        );
        
        // Any型との統一
        assert_eq!(
            engine.unify(&Type::Any, &Type::Character).unwrap(),
            Type::Character
        );
        
        // Numeric型との統一
        assert_eq!(
            engine.unify(&Type::Numeric, &Type::I32).unwrap(),
            Type::I32
        );
        
        // Vec型の統一
        let vec_i32 = Type::Vec(Box::new(Type::I32));
        let vec_any = Type::Vec(Box::new(Type::Any));
        assert_eq!(
            engine.unify(&vec_i32, &vec_any).unwrap(),
            vec_i32
        );
    }
    
    #[test]
    fn test_array_operation_inference() {
        let vec_char = Type::Vec(Box::new(Type::Character));
        
        // FilterListは型を維持
        assert_eq!(
            TypeInferenceEngine::infer_array_operation_type("FilterList", &vec_char, None),
            vec_char
        );
        
        // Mapは変換型を適用
        let hp_type = Type::CharacterHP;
        assert_eq!(
            TypeInferenceEngine::infer_array_operation_type("Map", &vec_char, Some(&hp_type)),
            Type::Vec(Box::new(Type::CharacterHP))
        );
        
        // RandomPickは要素型を返す
        assert_eq!(
            TypeInferenceEngine::infer_array_operation_type("RandomPick", &vec_char, None),
            Type::Character
        );
    }
}