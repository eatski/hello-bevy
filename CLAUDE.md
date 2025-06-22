# hello-bevy 設計サマリ

## 📝 ドキュメント管理ルール
**重要**: このCLAUDE.mdファイルは常に最新の状態に保つこと
- プロジェクトの変更時は必ずこのファイルを更新
- 設計変更、新機能追加、ファイル構成変更を即座に反映
- プロジェクトの現在の状態を正確に文書化
- ユーザーからの一般的なフィードバックもここに記録

## 🗣️ ユーザーフィードバック履歴
### 開発時の注意事項
- **コンパイル確認**: 変更後は必ず`cargo check`を実行すること
- **ドキュメント更新**: 一般的なフィードバックはこのドキュメントを更新すること
- **UI分離**: 具体的なキャラクター設定はmain.rsに、汎用的なUIロジックはui.rsに分離すること

## 🏗️ アーキテクチャ設計

### 📁 ファイル構成
```
src/
├── main.rs          - Bevyエンジン統合・ゲーム固有設定
├── ui.rs            - 汎用的なUI表示・入力処理
├── battle_system.rs - バトル管理・キャラクター定義
└── action_system.rs - トークンベース行動計算システム
```

### 🎯 モジュール分離設計
- **アプリ層**: `main.rs` - Bevyエンジン統合・ゲーム固有設定
- **UI層**: `ui.rs` - 汎用的なUI表示・入力処理
- **ドメイン層**: `battle_system.rs` - ゲームロジック
- **計算層**: `action_system.rs` - AI行動システム

## 🧩 システム設計

### 1. データ構造設計
```rust
// battle_system.rs
struct Character {
    // ステータス管理
    hp/max_hp, mp/max_mp, attack
}

struct Battle {
    // 戦闘状態管理
    player, enemy, current_turn, battle_over
}
```

### 2. トークンベース設計 (`action_system.rs`)
```rust
trait Token {
    fn evaluate() -> TokenResult
}

enum TokenResult {
    Continue(bool),  // 条件判定結果
    Action(ActionType), // 実行アクション
    Break,           // 行中断
}

// 外部からルール設定可能
ActionCalculationSystem::new(rules)
ActionCalculationSystem::with_seed(rules, seed)
```

### 3. 責任分離設計
- **`main.rs`**: Bevyシステム統合・ゲーム固有設定("勇者","スライム")
- **`ui.rs`**: 汎用的なUI表示・入力処理・画面描画
- **`battle_system.rs`**: 戦闘ルール・キャラクター管理
- **`action_system.rs`**: AI行動決定・トークン処理・外部ルール設定

### 4. 拡張性設計
- **新トークン追加**: `Token`トレイト実装のみ
- **新アクション追加**: `ActionType`enum拡張
- **UI変更**: `ui.rs`のみ修正で対応
- **ゲーム設定変更**: `main.rs`のみ修正で対応
- **カスタムルール**: 外部からルール設定可能

## 🔄 データフロー設計
```
入力 → ui.rs → battle_system.rs → action_system.rs → ui.rs → 結果表示
      ↑                                                         ↓
   main.rs (ゲーム設定)                                    画面描画
```

## 🧪 テスト設計
### 統合テスト (31テスト)
- **action_system.rs**: 単体テスト + 統合テスト
- **battle_system.rs**: 単体テスト + 様々なルールパターンテスト
- **ルールパターン例**:
  - 攻撃専用/回復専用ルール
  - 複雑なルールチェイン
  - 空ルール・決定論的実行テスト