# hello-bevy 設計サマリ

## 📝 ドキュメント管理ルール
commit前に必ず以下を守れ
- `cargo check --workspace` (警告も全て修正すること)
- `cargo test --workspace` (全crateのテストを実行)
**重要**: このCLAUDE.mdファイルは常に最新の状態に保つこと
- プロジェクトの変更時は必ずこのファイルを更新
- 設計変更、新機能追加、ファイル構成変更を即座に反映
- プロジェクトの現在の状態を正確に文書化
- ユーザーからの一般的なフィードバックもここに記録

### 🔧 テスト・ビルドコマンド
```bash
# 全ワークスペースの型チェック（推奨）
cargo check --workspace

# 全ワークスペースのテスト実行（推奨）
cargo test --workspace

# 個別crateのテスト
cargo test -p action-system
cargo test -p battle-system  
cargo test -p ui

# ドキュメンテーションテスト
cargo test --workspace --doc

# リリースビルド
cargo build --workspace --release
```


## 🗣️ ユーザーフィードバック履歴
### 開発時の注意事項
- **コンパイル確認**: 変更後は必ず`cargo check`を実行すること
- **ドキュメント更新**: 一般的なフィードバックはこのドキュメントを更新すること
- **UI分離**: 具体的なキャラクター設定はmain.rsに、汎用的なUIロジックはui.rsに分離すること
- **JSON設定**: キャラクターのruleはJSON外部ファイルから読み込み可能になった（フォールバック機構付き）

## 🏗️ アーキテクチャ設計

### 📁 ファイル構成（クレート分割後）
```
├── Cargo.toml          - ワークスペース設定
├── src/
│   └── main.rs         - Bevyエンジン統合・ゲーム統合バイナリ
├── crates/
│   ├── action-system/  - トークンベース行動計算システム
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs          - クレートエントリポイント
│   │       ├── character.rs    - Character型定義（循環依存回避）
│   │       ├── core.rs         - 基本トレイト・型定義
│   │       ├── actions.rs      - アクショントークン実装
│   │       ├── bool_tokens.rs  - 論理演算トークン実装
│   │       ├── number_tokens.rs- 数値トークン実装
│   │       └── system.rs       - 行動計算システム実装
│   ├── battle-system/  - バトル管理・ルール読み込みシステム
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs              - クレートエントリポイント
│   │       ├── battle.rs           - バトル管理ロジック
│   │       ├── rule_loader.rs      - JSON形式ルール読み込み・変換
│   │       └── rule_input_model.rs - JSON入力用データモデル定義
│   └── ui/             - Bevy UIコンポーネント・システム
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs  - クレートエントリポイント
│           └── ui.rs   - UI表示・入力処理・画面描画
└── rules/
    ├── player_rules.json - プレイヤーのデフォルトルール設定
    └── enemy_rules.json  - 敵のデフォルトルール設定
```

### 🎯 クレート分離設計
- **アプリ層**: `hello-bevy` (root) - Bevyエンジン統合・ゲーム統合バイナリ
- **UI層**: `ui` クレート - Bevy UIコンポーネント・システム
- **ドメイン層**: `battle-system` クレート - バトル管理・ルール読み込み
- **計算層**: `action-system` クレート - トークンベース行動計算システム
  - `character.rs` - Character型定義（循環依存回避）
  - `core.rs` - 基本トレイト・型定義
  - `actions.rs` - アクショントークン実装
  - `bool_tokens.rs` - 論理演算トークン実装
  - `number_tokens.rs` - 数値トークン実装
  - `system.rs` - 行動計算システム実装

## 🧩 システム設計

### 1. データ構造設計
```rust
// action-system/character.rs
struct Character {
    // ステータス管理
    hp/max_hp, mp/max_mp, attack
}

// battle-system/battle.rs  
struct Battle {
    // 戦闘状態管理
    player, enemy, current_turn, battle_over
}
```

### 2. トークンベース設計 (`action-system` クレート)
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

### 3. 責任分離設計（クレート別）
- **`hello-bevy` (root)**: Bevyシステム統合・ゲーム固有設定("勇者","スライム")
- **`ui` クレート**: 汎用的なUI表示・入力処理・画面描画
- **`battle-system` クレート**: 戦闘ルール・JSON読み込み・バトル管理
- **`action-system` クレート**: AI行動決定・トークン処理・Character型定義

### 4. 拡張性設計（クレート別）
- **新トークン追加**: `action-system` クレートの`Token`トレイト実装のみ
- **新アクション追加**: `action-system` クレートの`ActionType`enum拡張
- **UI変更**: `ui` クレートのみ修正で対応
- **ゲーム設定変更**: `hello-bevy` (root)のみ修正で対応
- **カスタムルール**: `battle-system` クレートでJSON外部ファイル読み込み（フォールバック機構付き）

### 5. JSON設定システム（`battle-system` クレート）
```rust
// battle-system/rule_input_model.rs
RuleSet { rules: [RuleChain{ tokens: [TokenConfig] }] }
TokenConfig: Strike | Heal | Check{args} | GreaterThan{args} | etc.

// battle-system/rule_loader.rs
load_rules_from_file(path) -> parse_rules_from_json(content) -> convert_to_token_rules(rule_set)
```
- **入力モデル**: `rule_input_model.rs` - JSON入力専用データ構造定義
- **ファイル読み込み**: `load_rules_from_file(path)`
- **JSON解析**: `parse_rules_from_json(content)`
- **変換処理**: `convert_to_token_rules(rule_set)` → `action-system` トークンに変換
- **フォールバック**: JSON読み込み失敗時はハードコードルールを使用

## 🔄 データフロー設計（クレート間）
```
入力 → ui クレート → battle-system クレート → action-system クレート → ui クレート → 結果表示
      ↑                                                                           ↓
   hello-bevy (root) - ゲーム設定                                              画面描画
```

## 📦 クレート依存関係ルール

### 依存関係の階層構造（ワークスペース）
```
hello-bevy (root バイナリ)
├── ui クレート
│   └── battle-system クレート
│       └── action-system クレート
└── 直接依存: action-system, battle-system, ui
```

### クレート間依存関係の制約ルール

1. **階層依存のみ許可（循環依存回避）**
   - `hello-bevy` (root) → `ui`, `battle-system`, `action-system` 依存可能
   - `ui` → `battle-system` のみ依存
   - `battle-system` → `action-system` のみ依存
   - `action-system` → 外部クレートのみ依存（完全独立）
   - **逆方向依存は禁止** (下位クレートが上位クレートに依存してはいけない)

2. **同一層内の相互依存は禁止**
   - `ui` ↔ `action-system` の直接依存は禁止（`battle-system`経由で利用）

3. **Character型の配置戦略**
   - `action-system` クレートに`Character`型を配置（循環依存回避）
   - `battle-system` が `action-system::Character` を再エクスポート

4. **許可される依存パターン**
   ```rust
   // ✅ 許可
   hello-bevy → ui, battle-system, action-system
   ui → battle-system
   battle-system → action-system
   
   // ❌ 禁止
   action-system → battle-system (逆方向)
   ui → action-system (同一層)
   action-system → ui (逆方向)
   ```

5. **新クレート追加時のルール**
   - 依存関係を明確に定義してから実装開始
   - 循環依存が発生しないことを確認
   - より下位の層に配置できないか検討
   - ワークスペースのCargo.tomlに追加

## 🧪 テスト設計（クレート別）
### 統合テスト (49テスト)
- **`action-system` クレート**: 11テスト - 単体テスト + トークンシステムテスト
  - ActionResolver, Token, 各種トークンの動作テスト
  - ActionCalculationSystemの統合テスト
- **`battle-system` クレート**: 38テスト - バトルシステム + ルール管理テスト
  - Battle, Character, RuleLoaderの単体テスト
  - 様々なルールパターンテスト（攻撃専用/回復専用/複雑なチェイン）
  - JSON読み込み・変換テスト
- **`ui` クレート**: 0テスト - UI関連（Bevyテストは別途）
- **`hello-bevy` (root)**: 0テスト - 統合バイナリ

### テスト実行方法
```bash
# 全クレートのテスト実行（推奨）
cargo test --workspace

# 個別クレートのテスト
cargo test -p action-system    # 11テスト
cargo test -p battle-system    # 38テスト
cargo test -p ui               # 0テスト

# 特定テストパターン
cargo test -p battle-system -- integration_tests
cargo test -p action-system -- token
```
