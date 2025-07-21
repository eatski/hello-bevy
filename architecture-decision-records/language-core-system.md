# Architecture Decision Record: Language Core System

## Status
Accepted

## Context
hello-bevy プロジェクトでは、ゲームロジックをトークンベースの宣言的な記述で表現している。現在、UIとJSONの2つの入力形式をサポートしているが、今後さらなる入力形式（ビジュアルプログラミング、自然言語など）の追加が想定される。

現在の `structured_to_node.rs` の実装は、場当たり的な変換ロジックの羅列となっており、今後の拡張に対してスケールしない。より本格的な型システムとコンパイラアーキテクチャが必要である。

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
  - `crates/token-input/src/structured_token.rs`

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

### 型システム設計

#### 基本型
- プリミティブ型: `i32`, `bool`, `String`
- ゲーム固有型: `Character`, `Team`, `CharacterHP`
- コレクション型: `Vec<T>`, `Option<T>`

#### 抽象型
```
trait Numeric {
    fn value(&self) -> i32;
}

// CharacterHP と i32 の両方が Numeric を実装
// これにより GreaterThan(CharacterHP, i32) のような異なる型の比較が可能
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
型検査・型推論
    ↓
中間表現（型付きAST）
    ↓
最適化
    ↓
実行可能Node
```

## Consequences

### Positive
- **型安全性**: コンパイル時の型チェックによりランタイムエラーを削減
- **拡張性**: 新しいトークンや型の追加が体系的に可能
- **保守性**: 明確な層分離により、各層の責任が明確
- **エラー診断**: 型エラーの詳細なフィードバック提供

## Implementation Roadmap

### Phase 1: 基本型システム
- 型定義の形式化
- 基本的な型チェック機能
- エラーレポート機能

### Phase 2: 型推論
- Hindley-Milner型推論の実装
- ジェネリクスサポート
- 抽象型（trait）の実装
