//! Hindley-Milner型推論システム
//! 
//! let多相性と主要型推論をサポートする本格的な型推論エンジン

use std::collections::{HashMap, HashSet};
use super::Type;

/// 型変数ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TypeVarId(usize);

/// 型スキーム（多相型）
#[derive(Debug, Clone, PartialEq)]
pub struct TypeScheme {
    /// 量化された型変数
    quantified_vars: HashSet<TypeVarId>,
    /// 型本体
    ty: PolyType,
}

impl TypeScheme {
    /// 単相型から型スキームを作成
    pub fn monomorphic(ty: PolyType) -> Self {
        Self {
            quantified_vars: HashSet::new(),
            ty,
        }
    }
    
    /// 多相型スキームを作成
    pub fn polymorphic(vars: HashSet<TypeVarId>, ty: PolyType) -> Self {
        Self {
            quantified_vars: vars,
            ty,
        }
    }
    
    /// 型スキームをインスタンス化
    pub fn instantiate(&self, engine: &mut HindleyMilner) -> PolyType {
        let mut subst = HashMap::new();
        
        // 量化された変数に新しい型変数を割り当て
        for &var in &self.quantified_vars {
            subst.insert(var, PolyType::Var(engine.fresh_type_var()));
        }
        
        self.ty.apply_substitution(&subst)
    }
}

/// 多相型（型変数を含む型）
#[derive(Debug, Clone, PartialEq)]
pub enum PolyType {
    /// 具体型
    Concrete(Type),
    /// 型変数
    Var(TypeVarId),
    /// 関数型
    Function(Box<PolyType>, Box<PolyType>),
    /// ジェネリック型
    Generic(String, Vec<PolyType>), // e.g., Generic("List", vec![Var(0)])
}

impl PolyType {
    /// 型に代入を適用
    fn apply_substitution(&self, subst: &HashMap<TypeVarId, PolyType>) -> PolyType {
        match self {
            PolyType::Concrete(_) => self.clone(),
            PolyType::Var(id) => {
                subst.get(id).cloned().unwrap_or_else(|| self.clone())
            }
            PolyType::Function(arg, ret) => {
                PolyType::Function(
                    Box::new(arg.apply_substitution(subst)),
                    Box::new(ret.apply_substitution(subst)),
                )
            }
            PolyType::Generic(name, args) => {
                PolyType::Generic(
                    name.clone(),
                    args.iter().map(|arg| arg.apply_substitution(subst)).collect(),
                )
            }
        }
    }
    
    /// 自由型変数を収集
    fn free_vars(&self) -> HashSet<TypeVarId> {
        match self {
            PolyType::Concrete(_) => HashSet::new(),
            PolyType::Var(id) => {
                let mut set = HashSet::new();
                set.insert(*id);
                set
            }
            PolyType::Function(arg, ret) => {
                let mut vars = arg.free_vars();
                vars.extend(ret.free_vars());
                vars
            }
            PolyType::Generic(_, args) => {
                args.iter()
                    .flat_map(|arg| arg.free_vars())
                    .collect()
            }
        }
    }
    
    /// 具体型に変換（型変数が残っていたらエラー）
    pub fn to_concrete(&self) -> Result<Type, String> {
        match self {
            PolyType::Concrete(ty) => Ok(ty.clone()),
            PolyType::Var(id) => Err(format!("Unresolved type variable: {:?}", id)),
            PolyType::Function(_, _) => Err("Function types not supported in concrete types".to_string()),
            PolyType::Generic(name, args) => {
                // ジェネリック型を具体型に変換
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
        }
    }
}

/// 型環境
#[derive(Debug, Clone)]
pub struct TypeEnv {
    /// 変数名から型スキームへのマッピング
    bindings: HashMap<String, TypeScheme>,
}

impl TypeEnv {
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
        }
    }
    
    /// 変数を束縛
    pub fn bind(&mut self, name: String, scheme: TypeScheme) {
        self.bindings.insert(name, scheme);
    }
    
    /// 変数の型スキームを取得
    pub fn lookup(&self, name: &str) -> Option<&TypeScheme> {
        self.bindings.get(name)
    }
    
    /// 環境の自由型変数を収集
    fn free_vars(&self) -> HashSet<TypeVarId> {
        self.bindings.values()
            .flat_map(|scheme| {
                // スキームの型の自由変数から量化された変数を除く
                let mut free = scheme.ty.free_vars();
                for &var in &scheme.quantified_vars {
                    free.remove(&var);
                }
                free
            })
            .collect()
    }
}

/// Hindley-Milner型推論エンジン
pub struct HindleyMilner {
    /// 次の型変数ID
    next_var_id: usize,
    /// 型代入
    substitution: HashMap<TypeVarId, PolyType>,
    /// 型制約
    constraints: Vec<(PolyType, PolyType)>,
}

impl HindleyMilner {
    pub fn new() -> Self {
        Self {
            next_var_id: 0,
            substitution: HashMap::new(),
            constraints: Vec::new(),
        }
    }
    
    /// 新しい型変数を生成
    pub fn fresh_type_var(&mut self) -> TypeVarId {
        let id = TypeVarId(self.next_var_id);
        self.next_var_id += 1;
        id
    }
    
    /// 型を一般化（generalize）
    pub fn generalize(&self, env: &TypeEnv, ty: &PolyType) -> TypeScheme {
        // 型の自由変数から環境の自由変数を除いたものが量化可能
        let ty_vars = ty.free_vars();
        let env_vars = env.free_vars();
        let quantified: HashSet<_> = ty_vars.difference(&env_vars).copied().collect();
        
        if quantified.is_empty() {
            TypeScheme::monomorphic(ty.clone())
        } else {
            TypeScheme::polymorphic(quantified, ty.clone())
        }
    }
    
    /// 型を統一（unify）
    pub fn unify(&mut self, t1: &PolyType, t2: &PolyType) -> Result<(), String> {
        // 現在の代入を適用
        let t1 = self.apply_current_substitution(t1);
        let t2 = self.apply_current_substitution(t2);
        
        match (&t1, &t2) {
            // 同じ具体型
            (PolyType::Concrete(a), PolyType::Concrete(b)) if a == b => Ok(()),
            
            // Numeric型の特殊処理
            (PolyType::Concrete(Type::Numeric), PolyType::Concrete(Type::I32)) |
            (PolyType::Concrete(Type::I32), PolyType::Concrete(Type::Numeric)) |
            (PolyType::Concrete(Type::Numeric), PolyType::Concrete(Type::CharacterHP)) |
            (PolyType::Concrete(Type::CharacterHP), PolyType::Concrete(Type::Numeric)) => Ok(()),
            
            // 型変数
            (PolyType::Var(id), other) | (other, PolyType::Var(id)) => {
                if let PolyType::Var(other_id) = other {
                    if id == other_id {
                        return Ok(());
                    }
                }
                // occurs check
                if other.free_vars().contains(id) {
                    return Err(format!("Infinite type: {:?} occurs in {:?}", id, other));
                }
                self.substitution.insert(*id, other.clone());
                Ok(())
            }
            
            // 関数型
            (PolyType::Function(a1, r1), PolyType::Function(a2, r2)) => {
                self.unify(a1, a2)?;
                self.unify(r1, r2)
            }
            
            // ジェネリック型
            (PolyType::Generic(n1, args1), PolyType::Generic(n2, args2)) => {
                if n1 != n2 || args1.len() != args2.len() {
                    return Err(format!("Cannot unify {} and {}", n1, n2));
                }
                for (a1, a2) in args1.iter().zip(args2.iter()) {
                    self.unify(a1, a2)?;
                }
                Ok(())
            }
            
            // Vec型の特殊処理
            (PolyType::Concrete(Type::Vec(elem1)), PolyType::Generic(name, args)) |
            (PolyType::Generic(name, args), PolyType::Concrete(Type::Vec(elem1))) => {
                if name == "Vec" && args.len() == 1 {
                    let elem_poly = PolyType::Concrete(elem1.as_ref().clone());
                    self.unify(&args[0], &elem_poly)
                } else {
                    Err(format!("Cannot unify Vec with {}", name))
                }
            }
            
            // Vec型同士の統一
            (PolyType::Concrete(Type::Vec(elem1)), PolyType::Concrete(Type::Vec(elem2))) => {
                // 要素型を統一
                let elem1_poly = PolyType::Concrete(elem1.as_ref().clone());
                let elem2_poly = PolyType::Concrete(elem2.as_ref().clone());
                self.unify(&elem1_poly, &elem2_poly)?;
                Ok(())
            }
            
            // 統一できない
            _ => Err(format!("Cannot unify {:?} and {:?}", t1, t2)),
        }
    }
    
    /// 現在の代入を型に適用
    fn apply_current_substitution(&self, ty: &PolyType) -> PolyType {
        ty.apply_substitution(&self.substitution)
    }
    
    /// 制約を解決
    pub fn solve_constraints(&mut self) -> Result<(), String> {
        let constraints = std::mem::take(&mut self.constraints);
        
        for (t1, t2) in constraints {
            self.unify(&t1, &t2)?;
        }
        
        Ok(())
    }
    
    /// 制約を追加
    pub fn add_constraint(&mut self, t1: PolyType, t2: PolyType) {
        self.constraints.push((t1, t2));
    }
}

impl Default for HindleyMilner {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_type_generalization() {
        let mut engine = HindleyMilner::new();
        let env = TypeEnv::new();
        
        // id = λx. x の型を推論
        let var_x = engine.fresh_type_var();
        let id_type = PolyType::Function(
            Box::new(PolyType::Var(var_x)),
            Box::new(PolyType::Var(var_x)),
        );
        
        // 一般化すると ∀a. a -> a になる
        let scheme = engine.generalize(&env, &id_type);
        assert_eq!(scheme.quantified_vars.len(), 1);
        assert!(scheme.quantified_vars.contains(&var_x));
    }
    
    #[test]
    fn test_type_instantiation() {
        let mut engine = HindleyMilner::new();
        
        // ∀a. a -> a
        let var_a = TypeVarId(0);
        let scheme = TypeScheme::polymorphic(
            vec![var_a].into_iter().collect(),
            PolyType::Function(
                Box::new(PolyType::Var(var_a)),
                Box::new(PolyType::Var(var_a)),
            ),
        );
        
        // インスタンス化すると新しい型変数になる
        let instance1 = scheme.instantiate(&mut engine);
        let instance2 = scheme.instantiate(&mut engine);
        
        // 2つのインスタンスは異なる型変数を持つ
        assert_ne!(instance1, instance2);
    }
    
    #[test]
    fn test_unification() {
        let mut engine = HindleyMilner::new();
        
        // Int と Numeric の統一
        let t1 = PolyType::Concrete(Type::I32);
        let t2 = PolyType::Concrete(Type::Numeric);
        assert!(engine.unify(&t1, &t2).is_ok());
        
        // Vec<a> と Vec<Int> の統一
        let var_a = engine.fresh_type_var();
        let vec_a = PolyType::Generic("Vec".to_string(), vec![PolyType::Var(var_a)]);
        let vec_int = PolyType::Concrete(Type::Vec(Box::new(Type::I32)));
        assert!(engine.unify(&vec_a, &vec_int).is_ok());
        
        // 代入を確認
        let subst_a = engine.apply_current_substitution(&PolyType::Var(var_a));
        assert_eq!(subst_a, PolyType::Concrete(Type::I32));
    }
}