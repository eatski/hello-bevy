//! Trait（型クラス）システム
//! 
//! 抽象型とその実装を管理する型クラスシステム

use std::collections::{HashMap, HashSet};
use super::Type;

/// Trait定義
#[derive(Debug, Clone)]
pub struct TraitDef {
    /// Trait名
    pub name: String,
    /// 型パラメータ
    pub type_params: Vec<String>,
    /// 必要なメソッド
    pub methods: Vec<MethodSignature>,
    /// スーパートレイト
    pub supertraits: Vec<String>,
}

/// メソッドシグネチャ
#[derive(Debug, Clone)]
pub struct MethodSignature {
    /// メソッド名
    pub name: String,
    /// 引数の型
    pub arg_types: Vec<Type>,
    /// 戻り値の型
    pub return_type: Type,
}

/// Trait実装
#[derive(Debug, Clone)]
pub struct TraitImpl {
    /// 実装するTrait名
    pub trait_name: String,
    /// 実装対象の型
    pub for_type: Type,
    /// 型パラメータの具体化
    pub type_args: HashMap<String, Type>,
}

/// Traitシステム
pub struct TraitSystem {
    /// 定義されたTrait
    traits: HashMap<String, TraitDef>,
    /// Trait実装
    implementations: Vec<TraitImpl>,
    /// 型からTraitへのインデックス（高速検索用）
    type_to_traits: HashMap<Type, HashSet<String>>,
}

impl TraitSystem {
    pub fn new() -> Self {
        let mut system = Self {
            traits: HashMap::new(),
            implementations: Vec::new(),
            type_to_traits: HashMap::new(),
        };
        
        // 組み込みTraitを登録
        system.register_builtin_traits();
        system
    }
    
    /// 組み込みTraitを登録
    fn register_builtin_traits(&mut self) {
        // Numeric trait
        self.define_trait(TraitDef {
            name: "Numeric".to_string(),
            type_params: vec![],
            methods: vec![
                MethodSignature {
                    name: "to_i32".to_string(),
                    arg_types: vec![],
                    return_type: Type::I32,
                },
            ],
            supertraits: vec![],
        });
        
        // Numeric実装を登録
        self.implement_trait(TraitImpl {
            trait_name: "Numeric".to_string(),
            for_type: Type::I32,
            type_args: HashMap::new(),
        });
        
        self.implement_trait(TraitImpl {
            trait_name: "Numeric".to_string(),
            for_type: Type::CharacterHP,
            type_args: HashMap::new(),
        });
        
        // Eq trait (等価比較)
        self.define_trait(TraitDef {
            name: "Eq".to_string(),
            type_params: vec![],
            methods: vec![],
            supertraits: vec![],
        });
        
        // 基本型にEqを実装
        for ty in &[Type::I32, Type::Bool, Type::String, Type::Character, 
                    Type::Team, Type::CharacterHP, Type::TeamSide] {
            self.implement_trait(TraitImpl {
                trait_name: "Eq".to_string(),
                for_type: ty.clone(),
                type_args: HashMap::new(),
            });
        }
        
        // Ord trait (順序比較)
        self.define_trait(TraitDef {
            name: "Ord".to_string(),
            type_params: vec![],
            methods: vec![],
            supertraits: vec!["Eq".to_string()],
        });
        
        // 数値型にOrdを実装
        for ty in &[Type::I32, Type::CharacterHP] {
            self.implement_trait(TraitImpl {
                trait_name: "Ord".to_string(),
                for_type: ty.clone(),
                type_args: HashMap::new(),
            });
        }
        
        // Collection trait
        self.define_trait(TraitDef {
            name: "Collection".to_string(),
            type_params: vec!["T".to_string()],
            methods: vec![
                MethodSignature {
                    name: "len".to_string(),
                    arg_types: vec![],
                    return_type: Type::I32,
                },
            ],
            supertraits: vec![],
        });
        
        // Show trait (表示可能)
        self.define_trait(TraitDef {
            name: "Show".to_string(),
            type_params: vec![],
            methods: vec![
                MethodSignature {
                    name: "to_string".to_string(),
                    arg_types: vec![],
                    return_type: Type::String,
                },
            ],
            supertraits: vec![],
        });
    }
    
    /// Traitを定義
    pub fn define_trait(&mut self, trait_def: TraitDef) {
        self.traits.insert(trait_def.name.clone(), trait_def);
    }
    
    /// Traitを実装
    pub fn implement_trait(&mut self, trait_impl: TraitImpl) {
        // 型からTraitへのインデックスを更新
        self.type_to_traits
            .entry(trait_impl.for_type.clone())
            .or_insert_with(HashSet::new)
            .insert(trait_impl.trait_name.clone());
        
        self.implementations.push(trait_impl);
    }
    
    /// 型がTraitを実装しているか確認
    pub fn implements(&self, ty: &Type, trait_name: &str) -> bool {
        // 直接実装を確認
        if let Some(traits) = self.type_to_traits.get(ty) {
            if traits.contains(trait_name) {
                return true;
            }
        }
        
        // 特殊な型の処理
        match ty {
            Type::Numeric => {
                // Numeric型はNumeric traitを実装
                trait_name == "Numeric"
            }
            Type::Vec(_elem_type) => {
                // Vec<T>はCollection<T>を実装
                if trait_name == "Collection" {
                    return true;
                }
                // 要素型の制約を確認（例: Vec<T: Show>ならShowを要求）
                false
            }
            _ => false,
        }
    }
    
    /// Trait定義を取得
    pub fn get_trait(&self, name: &str) -> Option<&TraitDef> {
        self.traits.get(name)
    }
    
    /// 型に実装されているTraitのリストを取得
    pub fn traits_for_type(&self, ty: &Type) -> Vec<String> {
        let mut traits = Vec::new();
        
        // 直接実装
        if let Some(impls) = self.type_to_traits.get(ty) {
            traits.extend(impls.iter().cloned());
        }
        
        // 特殊な型
        match ty {
            Type::Numeric => traits.push("Numeric".to_string()),
            Type::Vec(_) => traits.push("Collection".to_string()),
            _ => {}
        }
        
        // スーパートレイトを追加
        let mut all_traits = HashSet::new();
        for trait_name in &traits {
            self.collect_with_supertraits(trait_name, &mut all_traits);
        }
        
        all_traits.into_iter().collect()
    }
    
    /// スーパートレイトを含めて収集
    fn collect_with_supertraits(&self, trait_name: &str, collected: &mut HashSet<String>) {
        if collected.contains(trait_name) {
            return;
        }
        
        collected.insert(trait_name.to_string());
        
        if let Some(trait_def) = self.traits.get(trait_name) {
            for supertrait in &trait_def.supertraits {
                self.collect_with_supertraits(supertrait, collected);
            }
        }
    }
    
    /// Trait境界を検証
    pub fn check_bounds(&self, ty: &Type, required_traits: &[String]) -> Result<(), String> {
        for trait_name in required_traits {
            if !self.implements(ty, trait_name) {
                return Err(format!(
                    "Type {} does not implement trait {}",
                    ty, trait_name
                ));
            }
        }
        Ok(())
    }
}

impl Default for TraitSystem {
    fn default() -> Self {
        Self::new()
    }
}

/// Trait境界
#[derive(Debug, Clone)]
pub struct TraitBound {
    /// 型パラメータ名
    pub type_param: String,
    /// 必要なTrait
    pub trait_name: String,
}

/// ジェネリック型定義
#[derive(Debug, Clone)]
pub struct GenericTypeDef {
    /// 型名
    pub name: String,
    /// 型パラメータ
    pub type_params: Vec<String>,
    /// Trait境界
    pub bounds: Vec<TraitBound>,
    /// 型の構造（例: struct, enum）
    pub structure: TypeStructure,
}

/// 型の構造
#[derive(Debug, Clone)]
pub enum TypeStructure {
    /// 構造体
    Struct {
        fields: Vec<(String, Type)>,
    },
    /// 列挙型
    Enum {
        variants: Vec<(String, Vec<Type>)>,
    },
    /// 型エイリアス
    Alias(Type),
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_trait_implementation() {
        let system = TraitSystem::new();
        
        // I32はNumericを実装
        assert!(system.implements(&Type::I32, "Numeric"));
        assert!(system.implements(&Type::CharacterHP, "Numeric"));
        
        // CharacterはNumericを実装していない
        assert!(!system.implements(&Type::Character, "Numeric"));
        
        // I32はEqとOrdを実装
        assert!(system.implements(&Type::I32, "Eq"));
        assert!(system.implements(&Type::I32, "Ord"));
    }
    
    #[test]
    fn test_supertrait() {
        let system = TraitSystem::new();
        
        // OrdはEqをスーパートレイトとして持つ
        let traits = system.traits_for_type(&Type::I32);
        assert!(traits.contains(&"Ord".to_string()));
        assert!(traits.contains(&"Eq".to_string()));
    }
    
    #[test]
    fn test_trait_bounds() {
        let system = TraitSystem::new();
        
        // I32はNumericとEqを満たす
        assert!(system.check_bounds(&Type::I32, &["Numeric".to_string(), "Eq".to_string()]).is_ok());
        
        // CharacterはNumericを満たさない
        assert!(system.check_bounds(&Type::Character, &["Numeric".to_string()]).is_err());
    }
}