# 統一Node<T>トレイト設計案

## 現在の構造
```rust
trait CharacterNode: Send + Sync + Debug {
    fn evaluate(&self, eval_context: &EvaluationContext, rng: &mut dyn RngCore) -> NodeResult<i32>;
}

trait ValueNode: Send + Sync + Debug {
    fn evaluate(&self, eval_context: &EvaluationContext, rng: &mut dyn RngCore) -> NodeResult<i32>;
}

trait ConditionNode: Send + Sync + Debug {
    fn evaluate(&self, eval_context: &EvaluationContext, rng: &mut dyn RngCore) -> NodeResult<bool>;
}

trait ArrayNode<T>: Send + Sync + Debug {
    fn evaluate(&self, eval_context: &EvaluationContext, rng: &mut dyn RngCore) -> NodeResult<Vec<T>>;
}
```

## 提案: 統一Node<T>トレイト
```rust
/// 統一されたノードトレイト - あらゆる型の評価結果を返す
pub trait Node<T>: Send + Sync + std::fmt::Debug {
    fn evaluate(&self, eval_context: &EvaluationContext, rng: &mut dyn rand::RngCore) -> NodeResult<T>;
}

// 型エイリアスで後方互換性を保持
pub type CharacterNode = dyn Node<i32>;      // キャラクターID
pub type ValueNode = dyn Node<i32>;          // 数値 
pub type ConditionNode = dyn Node<bool>;     // 真偽値
pub type ArrayNode<T> = dyn Node<Vec<T>>;    // 配列

// 具体的な型エイリアス
pub type CharacterArrayNode = dyn Node<Vec<Character>>;
pub type ValueArrayNode = dyn Node<Vec<i32>>;
```

## メリット
1. **統一性**: 1つのトレイトですべてのノード型を扱える
2. **型安全性**: ジェネリクスによる完全な型チェック
3. **拡張性**: 新しい戻り値型（例: `String`, `f64`）を簡単に追加可能
4. **簡潔性**: トレイト定義が1つだけで済む
5. **後方互換性**: 型エイリアスで既存コードをそのまま使用可能

## デメリット
1. **型消去**: `Box<dyn Node<T>>`を使う際は`T`を明示する必要がある
2. **複雑性**: 初見の開発者には理解が困難かも
3. **コンパイル時間**: ジェネリクスによる若干の増加

## 実装例
```rust
// ActingCharacterNode
impl Node<i32> for ActingCharacterNode {
    fn evaluate(&self, eval_context: &EvaluationContext, _rng: &mut dyn RngCore) -> NodeResult<i32> {
        Ok(eval_context.get_battle_context().get_acting_character().id)
    }
}

// RandomConditionNode  
impl Node<bool> for RandomConditionNode {
    fn evaluate(&self, _eval_context: &EvaluationContext, rng: &mut dyn RngCore) -> NodeResult<bool> {
        Ok(rng.gen_bool(0.5))
    }
}

// AllCharactersNode
impl Node<Vec<Character>> for AllCharactersNode {
    fn evaluate(&self, eval_context: &EvaluationContext, _rng: &mut dyn RngCore) -> NodeResult<Vec<Character>> {
        let battle_context = eval_context.get_battle_context();
        let characters = battle_context.all_characters().into_iter().cloned().collect();
        Ok(characters)
    }
}
```

## 変更が必要な箇所
1. 全ノード実装の`impl`ブロック更新
2. 型エイリアスの追加
3. `Box<dyn Node<T>>`への変更
4. token-inputクレートでの型指定
5. ParsedResolverの更新

## 推奨実装手順
1. 新しい`Node<T>`トレイトを追加（既存トレイトと並行）
2. 型エイリアスで後方互換性を確保
3. 段階的に各ノードを移行
4. 既存トレイトを削除
5. 全テストで動作確認