# 型システムの構築

## Context
hello-bevy プロジェクトでは、ゲームロジックをトークンベースの宣言的な記述で表現できるようにすることでユーザーに自由な戦闘の戦略を決定することを目指したい。
現在の `crates/token-input/src/converter.rs` の実装は、場当たり的な変換ロジックの羅列となっており、今後の拡張に対してスケールしない。より本格的な型システムとコンパイラアーキテクチャが必要である。

### 目指す姿

- 今後、トークンの種類が増える際のnodeへの変換ロジックが
  - 最小になっていること
  - 修正漏れが発生しずらいこと
- 型システムがゲーム固有の型定義を持たず、完全にジェネリックであること

### 考えられる拡張

- より複雑な型を持つトークンの追加
- 様々な種類のトークンの追加
- 基本型の追加

## Decision
言語コアシステムとして、以下の3層アーキテクチャを採用する：

### 1. フロントエンド層（Frontend Layer）
- **役割**: 各種入力形式（UI、JSON）を受け付ける
- **特徴**:
  - 入力形式ごとに専用のデータ構造を定義
  - 演算子の概念は存在せず、すべてトークン（関数）として表現
  - 例: `=` 演算子ではなく `Eq(a, b)` トークン
- **実装例**:
  - `rules/enemy_rules.json`

### 2. コア層（Core Layer）
- **役割**: 型定義と型解決のルールを管理
- **機能**:
  - **静的型検査**: 基本的な型チェック機能を提供
  - **型推論**: フロントエンドからの型指定なしに型を自動推論
  - **ジェネリクス**: 型パラメータによる汎用的な型定義
  - **抽象型**: `Numeric` のような共通インターフェースの定義
- **設計方針**:
  - 具体的な型に依存せず、構造的な型解決ルールのみを定義
  - エラーフィードバックをユーザーに適切に伝達

### 3. 実行層（Execution Layer）
- **役割**: コンパイル結果を実行可能な形式に変換
- **出力**: 木構造の `Node` グラフ
- **特徴**:
  - 各 `Node` が実行時ロジックを直接保持
  - `action-system` クレートで実装

## アーキテクチャ詳細

### ジェネリック型システム設計

#### 設計原則
- 型システムは具体的な型を一切知らない
- ジェネリクスを使用して任意のドメインで再利用可能
- 型の「形」と「関係性」のみを扱う

#### TypeSystemトレイト
```rust
trait TypeSystem {
    type TypeId: Clone + Eq + Hash;
    
    // 型の関係性を定義
    fn is_subtype(&self, sub: &Self::TypeId, super_: &Self::TypeId) -> bool;
    fn unify(&self, a: &Self::TypeId, b: &Self::TypeId) -> Option<Self::TypeId>;
}
```

#### 型推論エンジン
```rust
struct TypeInferenceEngine<T: TypeSystem> {
    system: T,
}

impl<T: TypeSystem> TypeInferenceEngine<T> {
    pub fn infer<AST>(&self, ast: &AST) -> Result<TypedAST<T::TypeId>, TypeError>
    where
        AST: TypeInferable<T::TypeId>
}
```

#### 型付きAST
```rust
struct TypedAST<TypeId> {
    node_type: TypeId,
    token_name: String,
    arguments: Vec<TypedAST<TypeId>>,
}
```

#### メタデータ駆動
```rust
struct TokenMetadata<TypeId> {
    name: String,
    argument_types: Vec<TypeId>,
    return_type: TypeId,
    validator: Option<Box<dyn Fn(&[TypeId]) -> bool>>,
}
```

#### 擬似関数の実現
フロントエンドに関数概念は存在しないが、特定のトークンで擬似的に実現

例: 全てのキャラクターからHero側のキャラクターを取得する
```json
{
  "type": "FilterList",
  "array": {
    "type": "AllCharacters"
  },
  "condition": {
    "type": "Eq",
    "left": {
      "type": "CharacterTeam",
      "character": {
        "type": "Element"
      }
    },
    "right": {
      "type": "Hero"
    }
  }
}
```

### コンパイルパイプライン
```
入力（JSON/UI）
    ↓
構造化トークン（StructuredTokenInput）
    ↓
型検査・型推論（ジェネリック）
    ↓
中間表現（TypedAST<TypeId>）
    ↓
コード生成（ドメイン固有）
    ↓
実行可能Node
```

### 実装例

#### ゲーム側での型定義
```rust
// ゲーム固有の型（action-systemクレート）
enum GameType {
    Character,
    CharacterHP,
    I32,
    Bool,
    Action,
    Array(Box<GameType>),
    Numeric, // 抽象型
}

struct GameTypeSystem;

impl TypeSystem for GameTypeSystem {
    type TypeId = GameType;
    
    fn is_subtype(&self, sub: &GameType, super_: &GameType) -> bool {
        match (sub, super_) {
            (GameType::I32, GameType::Numeric) => true,
            (GameType::CharacterHP, GameType::Numeric) => true,
            _ => false,
        }
    }
}
```

#### コンパイラの使用
```rust
// 型システムの初期化
let type_system = GameTypeSystem;
let mut registry = TokenRegistry::new();

// トークンメタデータの登録
registry.register(TokenMetadata {
    name: "Strike".to_string(),
    argument_types: vec![GameType::Character],
    return_type: GameType::Action,
    validator: None,
});

// コンパイラの作成
let compiler = Compiler {
    type_system,
    inference_engine: TypeInferenceEngine::new(type_system),
    code_generator: GameCodeGenerator,
};
```

## 利点

1. **完全な独立性**: 型システムはドメイン固有の型を知らない
2. **再利用性**: 異なるドメインで同じ型システムを使用可能
3. **拡張性**: 新しい型は`TypeSystem`実装を追加するだけ
4. **型安全性**: ジェネリクスによるコンパイル時チェック
5. **保守性**: 型システムとドメインロジックの完全な分離
