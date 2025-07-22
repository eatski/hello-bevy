//! ジェネリクス型システム
//! 
//! ユーザー定義ジェネリック型と高階型のサポート

use std::collections::HashMap;
use super::Type;

/// 型パラメータ
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeParam {
    /// パラメータ名
    pub name: String,
    /// 型の種類（kind）
    pub kind: Kind,
    /// trait境界
    pub bounds: Vec<String>,
}

/// 型の種類（Kind）
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Kind {
    /// 通常の型 (*)
    Type,
    /// 型コンストラクタ (* -> *)
    TypeConstructor(Box<Kind>, Box<Kind>),
    /// 高階型 ((* -> *) -> *)
    HigherKinded(Box<Kind>, Box<Kind>),
}

impl Kind {
    /// * -> * -> * のような種類を作成
    pub fn multi_param_constructor(arity: usize) -> Self {
        let mut kind = Kind::Type;
        for _ in 0..arity {
            kind = Kind::TypeConstructor(Box::new(Kind::Type), Box::new(kind));
        }
        kind
    }
}

/// ジェネリック型定義
#[derive(Debug, Clone)]
pub struct GenericTypeDef {
    /// 型名
    pub name: String,
    /// 型パラメータ
    pub type_params: Vec<TypeParam>,
    /// 型の本体
    pub body: GenericTypeBody,
}

/// ジェネリック型の本体
#[derive(Debug, Clone)]
pub enum GenericTypeBody {
    /// 構造体
    Struct {
        fields: Vec<(String, GenericType)>,
    },
    /// 列挙型
    Enum {
        variants: Vec<(String, Vec<GenericType>)>,
    },
    /// 型エイリアス
    Alias(GenericType),
    /// newtype
    Newtype(GenericType),
}

/// ジェネリック型（型パラメータを含む型）
#[derive(Debug, Clone, PartialEq)]
pub enum GenericType {
    /// 具体型
    Concrete(Type),
    /// 型パラメータ
    Param(String),
    /// 型適用
    App(Box<GenericType>, Box<GenericType>),
    /// ジェネリック型の参照
    Named(String, Vec<GenericType>),
    /// 関数型
    Function(Box<GenericType>, Box<GenericType>),
    /// タプル型
    Tuple(Vec<GenericType>),
}

impl GenericType {
    /// 型パラメータを具体型に置換
    pub fn substitute(&self, subst: &HashMap<String, GenericType>) -> GenericType {
        match self {
            GenericType::Concrete(_) => self.clone(),
            GenericType::Param(name) => {
                subst.get(name).cloned().unwrap_or_else(|| self.clone())
            }
            GenericType::App(f, arg) => {
                GenericType::App(
                    Box::new(f.substitute(subst)),
                    Box::new(arg.substitute(subst)),
                )
            }
            GenericType::Named(name, args) => {
                GenericType::Named(
                    name.clone(),
                    args.iter().map(|arg| arg.substitute(subst)).collect(),
                )
            }
            GenericType::Function(arg, ret) => {
                GenericType::Function(
                    Box::new(arg.substitute(subst)),
                    Box::new(ret.substitute(subst)),
                )
            }
            GenericType::Tuple(elems) => {
                GenericType::Tuple(
                    elems.iter().map(|elem| elem.substitute(subst)).collect()
                )
            }
        }
    }
    
    /// 具体型に変換（型パラメータが残っていたらエラー）
    pub fn to_concrete(&self) -> Result<Type, String> {
        match self {
            GenericType::Concrete(ty) => Ok(ty.clone()),
            GenericType::Param(name) => {
                Err(format!("Unresolved type parameter: {}", name))
            }
            GenericType::Named(name, args) => {
                // 組み込みジェネリック型の処理
                match name.as_str() {
                    "Vec" => {
                        if args.len() == 1 {
                            let elem_type = args[0].to_concrete()?;
                            Ok(Type::Vec(Box::new(elem_type)))
                        } else {
                            Err(format!("Vec expects 1 type argument, got {}", args.len()))
                        }
                    }
                    "Option" => {
                        if args.len() == 1 {
                            let inner_type = args[0].to_concrete()?;
                            Ok(Type::Option(Box::new(inner_type)))
                        } else {
                            Err(format!("Option expects 1 type argument, got {}", args.len()))
                        }
                    }
                    _ => Err(format!("Unknown generic type: {}", name)),
                }
            }
            _ => Err("Complex generic types cannot be converted to concrete types".to_string()),
        }
    }
}

/// ジェネリック型レジストリ
pub struct GenericTypeRegistry {
    /// 定義されたジェネリック型
    types: HashMap<String, GenericTypeDef>,
}

impl GenericTypeRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            types: HashMap::new(),
        };
        registry.register_builtin_types();
        registry
    }
    
    /// 組み込みジェネリック型を登録
    fn register_builtin_types(&mut self) {
        // Result<T, E>
        self.register(GenericTypeDef {
            name: "Result".to_string(),
            type_params: vec![
                TypeParam {
                    name: "T".to_string(),
                    kind: Kind::Type,
                    bounds: vec![],
                },
                TypeParam {
                    name: "E".to_string(),
                    kind: Kind::Type,
                    bounds: vec![],
                },
            ],
            body: GenericTypeBody::Enum {
                variants: vec![
                    ("Ok".to_string(), vec![GenericType::Param("T".to_string())]),
                    ("Err".to_string(), vec![GenericType::Param("E".to_string())]),
                ],
            },
        });
        
        // List<T> (例として)
        self.register(GenericTypeDef {
            name: "List".to_string(),
            type_params: vec![
                TypeParam {
                    name: "T".to_string(),
                    kind: Kind::Type,
                    bounds: vec![],
                },
            ],
            body: GenericTypeBody::Enum {
                variants: vec![
                    ("Nil".to_string(), vec![]),
                    ("Cons".to_string(), vec![
                        GenericType::Param("T".to_string()),
                        GenericType::Named("List".to_string(), vec![GenericType::Param("T".to_string())]),
                    ]),
                ],
            },
        });
        
        // Functor trait向けの高階型の例
        // Functor<F> where F: * -> *
        self.register(GenericTypeDef {
            name: "Functor".to_string(),
            type_params: vec![
                TypeParam {
                    name: "F".to_string(),
                    kind: Kind::TypeConstructor(Box::new(Kind::Type), Box::new(Kind::Type)),
                    bounds: vec![],
                },
            ],
            body: GenericTypeBody::Alias(GenericType::Param("F".to_string())),
        });
    }
    
    /// ジェネリック型を登録
    pub fn register(&mut self, def: GenericTypeDef) {
        self.types.insert(def.name.clone(), def);
    }
    
    /// ジェネリック型定義を取得
    pub fn get(&self, name: &str) -> Option<&GenericTypeDef> {
        self.types.get(name)
    }
    
    /// 型をインスタンス化
    pub fn instantiate(
        &self,
        name: &str,
        type_args: Vec<GenericType>,
    ) -> Result<GenericType, String> {
        let def = self.get(name)
            .ok_or_else(|| format!("Unknown generic type: {}", name))?;
        
        if def.type_params.len() != type_args.len() {
            return Err(format!(
                "{} expects {} type arguments, got {}",
                name,
                def.type_params.len(),
                type_args.len()
            ));
        }
        
        // 型パラメータを置換
        let mut subst = HashMap::new();
        for (param, arg) in def.type_params.iter().zip(type_args.iter()) {
            subst.insert(param.name.clone(), arg.clone());
        }
        
        // 本体に適用
        match &def.body {
            GenericTypeBody::Alias(ty) => Ok(ty.substitute(&subst)),
            _ => Ok(GenericType::Named(name.to_string(), type_args)),
        }
    }
}

impl Default for GenericTypeRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// 型の種類を検証
pub fn check_kind(ty: &GenericType, expected: &Kind) -> Result<(), String> {
    let actual = infer_kind(ty)?;
    if actual != *expected {
        return Err(format!("Kind mismatch: expected {:?}, got {:?}", expected, actual));
    }
    Ok(())
}

/// 型の種類を推論
pub fn infer_kind(ty: &GenericType) -> Result<Kind, String> {
    match ty {
        GenericType::Concrete(_) => Ok(Kind::Type),
        GenericType::Param(_) => Ok(Kind::Type), // デフォルトでは*
        GenericType::App(f, _) => {
            match infer_kind(f)? {
                Kind::TypeConstructor(_, result) => Ok(*result),
                _ => Err("Type application to non-constructor".to_string()),
            }
        }
        GenericType::Named(name, _) => {
            // 組み込み型コンストラクタ
            match name.as_str() {
                "Vec" | "Option" | "List" => Ok(Kind::Type),
                _ => Ok(Kind::Type), // デフォルト
            }
        }
        GenericType::Function(_, _) => Ok(Kind::Type),
        GenericType::Tuple(_) => Ok(Kind::Type),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_generic_type_substitution() {
        let ty = GenericType::Named(
            "Result".to_string(),
            vec![
                GenericType::Param("T".to_string()),
                GenericType::Concrete(Type::String),
            ],
        );
        
        let mut subst = HashMap::new();
        subst.insert("T".to_string(), GenericType::Concrete(Type::I32));
        
        let result = ty.substitute(&subst);
        
        match result {
            GenericType::Named(name, args) => {
                assert_eq!(name, "Result");
                assert_eq!(args[0], GenericType::Concrete(Type::I32));
                assert_eq!(args[1], GenericType::Concrete(Type::String));
            }
            _ => panic!("Unexpected result"),
        }
    }
    
    #[test]
    fn test_kind_inference() {
        // Vec: * -> *
        let vec_constructor = GenericType::Named("Vec".to_string(), vec![]);
        
        // Vec<Int>: *
        let vec_int = GenericType::Named(
            "Vec".to_string(),
            vec![GenericType::Concrete(Type::I32)],
        );
        
        assert_eq!(infer_kind(&vec_int).unwrap(), Kind::Type);
    }
    
    #[test]
    fn test_generic_type_registry() {
        let registry = GenericTypeRegistry::new();
        
        // Result型が登録されている
        assert!(registry.get("Result").is_some());
        
        // インスタンス化
        let result_type = registry.instantiate(
            "Result",
            vec![
                GenericType::Concrete(Type::I32),
                GenericType::Concrete(Type::String),
            ],
        ).unwrap();
        
        match result_type {
            GenericType::Named(name, args) => {
                assert_eq!(name, "Result");
                assert_eq!(args.len(), 2);
            }
            _ => panic!("Unexpected result"),
        }
    }
}