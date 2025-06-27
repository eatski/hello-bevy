# hello-bevy 設計サマリ

## 📝 ドキュメント管理ルール
commit前に必ず以下を守れ
- `cargo check --workspace` (警告も全て修正すること)
- `cargo test --workspace` (全crateのテストを実行)
- README.mdの最新化
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
cargo test -p combat-engine
cargo test -p rule-parser
cargo test -p game-logic
cargo test -p bevy-frontend

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
- **UI検証**: トークン配置の有効性検証ロジックは削除済み - 実際の変換処理でのみ妥当性が確認される
- **変換アーキテクチャ**: UIの直接変換を廃止し、rule-parserを経由する統一パイプラインに変更（UI TokenType → rule-parser RuleSet → combat-engine RuleToken）

## 🏗️ アーキテクチャ設計

### 📁 ファイル構成（クレート分割後）
```
├── Cargo.toml          - ワークスペース設定
├── src/
│   └── main.rs         - Bevyエンジン統合・ゲーム統合バイナリ
├── crates/
│   ├── combat-engine/  - トークンベース行動計算システム
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs          - クレートエントリポイント
│   │       ├── character.rs    - Character型定義（循環依存回避）
│   │       ├── core.rs         - 基本トレイト・型定義
│   │       ├── actions.rs      - アクショントークン実装
│   │       ├── bool_tokens.rs  - 論理演算トークン実装
│   │       ├── number_tokens.rs- 数値トークン実装
│   │       └── system.rs       - 行動計算システム実装
│   ├── rule-parser/    - JSON ルール読み込み・変換システム
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs              - クレートエントリポイント
│   │       ├── rule_loader.rs      - JSON形式ルール読み込み・変換
│   │       └── rule_input_model.rs - JSON入力用データモデル定義
│   ├── game-logic/     - バトル管理・戦闘ロジック
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs  - クレートエントリポイント
│   │       └── battle.rs - バトル管理ロジック
│   └── bevy-frontend/  - Bevy UIコンポーネント・システム
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
- **UI層**: `bevy-frontend` クレート - Bevy UIコンポーネント・システム
- **ドメイン層**: `game-logic` クレート - バトル管理・戦闘ロジック
- **設定層**: `rule-parser` クレート - JSON ルール読み込み・変換システム
- **計算層**: `combat-engine` クレート - トークンベース行動計算システム
  - `character.rs` - Character型定義（循環依存回避）
  - `core.rs` - 基本トレイト・型定義
  - `actions.rs` - アクショントークン実装
  - `bool_tokens.rs` - 論理演算トークン実装
  - `number_tokens.rs` - 数値トークン実装
  - `system.rs` - 行動計算システム実装

## 🧩 システム設計

### 1. データ構造設計
```rust
// combat-engine/character.rs
struct Character {
    // ステータス管理
    hp/max_hp, mp/max_mp, attack
}

// game-logic/battle.rs  
struct Battle {
    // 戦闘状態管理
    player, enemy, current_turn, battle_over
}
```

### 2. トークンベース設計 (`combat-engine` クレート)
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
- **`bevy-frontend` クレート**: 汎用的なUI表示・入力処理・画面描画
- **`game-logic` クレート**: バトル管理・戦闘ロジック
- **`rule-parser` クレート**: JSON読み込み・変換システム
- **`combat-engine` クレート**: AI行動決定・トークン処理・Character型定義

### 4. 拡張性設計（クレート別）
- **新トークン追加**: `combat-engine` クレートの`Token`トレイト実装のみ
- **新アクション追加**: `combat-engine` クレートの`ActionType`enum拡張
- **UI変更**: `bevy-frontend` クレートのみ修正で対応
- **ゲーム設定変更**: `hello-bevy` (root)のみ修正で対応
- **カスタムルール**: `rule-parser` クレートでJSON外部ファイル読み込み（フォールバック機構付き）
- **戦闘ロジック変更**: `game-logic` クレートのみ修正で対応

### 5. JSON設定システム（`rule-parser` クレート）
```rust
// rule-parser/rule_input_model.rs
RuleSet { rules: [RuleChain{ tokens: [TokenConfig] }] }
TokenConfig: Strike | Heal | Check{args} | GreaterThan{args} | etc.

// rule-parser/rule_loader.rs
load_rules_from_file(path) -> parse_rules_from_json(content) -> convert_to_token_rules(rule_set)
```
- **入力モデル**: `rule_input_model.rs` - JSON入力専用データ構造定義
- **ファイル読み込み**: `load_rules_from_file(path)`
- **JSON解析**: `parse_rules_from_json(content)`
- **変換処理**: `convert_to_token_rules(rule_set)` → `combat-engine` トークンに変換
- **フォールバック**: JSON読み込み失敗時はハードコードルールを使用

## 🔄 データフロー設計（クレート間）
```
入力 → bevy-frontend クレート → game-logic クレート → combat-engine クレート → bevy-frontend クレート → 結果表示
      ↑                          ↑                                                                      ↓
   hello-bevy (root)           rule-parser クレート (JSON読み込み)                                  画面描画
```

## 📦 クレート依存関係ルール

### 依存関係の階層構造（ワークスペース）
```
hello-bevy (root バイナリ)
├── bevy-frontend クレート
│   └── game-logic クレート
│       ├── rule-parser クレート
│       │   └── combat-engine クレート
│       └── combat-engine クレート
└── 直接依存: combat-engine, rule-parser, game-logic, bevy-frontend
```

### クレート間依存関係の制約ルール

1. **階層依存のみ許可（循環依存回避）**
   - `hello-bevy` (root) → `bevy-frontend`, `game-logic`, `rule-parser`, `combat-engine` 依存可能
   - `bevy-frontend` → `game-logic` のみ依存
   - `game-logic` → `rule-parser`, `combat-engine` 依存
   - `rule-parser` → `combat-engine` のみ依存
   - `combat-engine` → 外部クレートのみ依存（完全独立）
   - **逆方向依存は禁止** (下位クレートが上位クレートに依存してはいけない)

2. **同一層内の相互依存は禁止**
   - `bevy-frontend` ↔ `rule-parser` の直接依存は禁止（`game-logic`経由で利用）

3. **Character型の配置戦略**
   - `combat-engine` クレートに`Character`型を配置（循環依存回避）
   - `game-logic` が `combat-engine::Character` を再エクスポート

4. **許可される依存パターン**
   ```rust
   // ✅ 許可
   hello-bevy → bevy-frontend, game-logic, rule-parser, combat-engine
   bevy-frontend → game-logic
   game-logic → rule-parser, combat-engine
   rule-parser → combat-engine
   
   // ❌ 禁止
   combat-engine → rule-parser (逆方向)
   bevy-frontend → rule-parser (同一層)
   combat-engine → bevy-frontend (逆方向)
   ```

5. **新クレート追加時のルール**
   - 依存関係を明確に定義してから実装開始
   - 循環依存が発生しないことを確認
   - より下位の層に配置できないか検討
   - ワークスペースのCargo.tomlに追加

## 🧪 テスト設計（クレート別）
### 統合テスト (49テスト)
- **`combat-engine` クレート**: 11テスト - 単体テスト + トークンシステムテスト
  - ActionResolver, Token, 各種トークンの動作テスト
  - ActionCalculationSystemの統合テスト
- **`rule-parser` クレート**: 12テスト - ルール読み込み・変換テスト
  - JSON読み込み・解析テスト
  - TokenConfig → ActionResolver変換テスト
  - エラーハンドリングテスト
- **`game-logic` クレート**: 26テスト - バトルシステムテスト
  - Battle, Character の単体テスト
  - 様々なルールパターンテスト（攻撃専用/回復専用/複雑なチェイン）
  - 戦闘ロジック統合テスト
- **`bevy-frontend` クレート**: 0テスト - UI関連（Bevyテストは別途）
- **`hello-bevy` (root)**: 0テスト - 統合バイナリ

### テスト実行方法
```bash
# 全クレートのテスト実行（推奨）
cargo test --workspace

# 個別クレートのテスト
cargo test -p combat-engine
cargo test -p rule-parser
cargo test -p game-logic
cargo test -p bevy-frontend

# 特定テストパターン
cargo test -p game-logic -- integration_tests
cargo test -p combat-engine -- token
cargo test -p rule-parser -- loader
```
