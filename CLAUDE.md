# hello-bevy 設計サマリ

## 📝　重要
タスク完了時に必ず以下を実施するように事前にタスク化すること
- `cargo check --workspace` (警告も全て修正すること)
- `cargo test --workspace` (全crateのテストを実行)
- README.mdの最新化
- このCLAUDE.mdファイルは常に最新の状態に保つこと
  - ユーザーからの一般的なフィードバックもここに記録
  - 設計変更、新機能追加、ファイル構成変更
- 想定されるコミットメッセージをユーザーに伝える（git操作はしないで）

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
- **UI関心分離**: UI層をBevy依存（bevy-ui）とBevy非依存（ui-core）に完全分離、文字列表示はBevy層に集約
- **トークン原子性**: UITokenType::HPを分割し、ActingCharacterとHPを個別トークンとして管理する原子的設計に変更
- **チーム戦闘移行**: 1vs1戦闘システムを完全に削除し、チーム戦闘システムに統一（TeamBattleクラス、Team構造体、チーム管理機能で置き換え）
- **UI最適化**: Rule設定セクションを小さくし、戦闘ログを削除してUIをコンパクト化
- **ランダムターゲット実装**: Strikeアクションで標的をランダムに決定するよう変更（以前は最初に見つかった生存キャラクターを攻撃）
- **1v1戦闘完全削除**: Battle構造体、impl Battle、1v1戦闘関連テスト（26個）を完全削除し、チーム戦闘のみのシステムに統一
- **main.rsリファクタリング**: 起動処理のみに集中、具体的なロジックをbevy-uiクレートに委譲（DI的なアーキテクチャ）
- **IDベースターゲティング実装**: CharacterにIDフィールドを追加し、ActionトレイトのtargetをIDで指定するように変更、BattleStateを用いた実際の戦闘処理を実装
- **設定可能ターゲット実装**: StrikeとHealアクションで標的をUI/JSONから設定可能に（ActingCharacter、RandomCharacterなど選択可、UI/JSON両層で必須指定、フォールバック廃止）

## 🏗️ アーキテクチャ設計

### 📁 ファイル構成（クレート分割後）
```
├── Cargo.toml          - ワークスペース設定
├── src/
│   └── main.rs         - アプリケーション起動（DI的な役割）
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
│   └── bevy-ui/        - Bevy UIコンポーネント・システム・プラグイン
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs     - クレートエントリポイント
│           ├── ui.rs      - UI表示・コンポーネント定義
│           ├── systems.rs - ゲームシステム実装
│           └── plugin.rs  - Bevyプラグイン統合
└── rules/
    ├── player_rules.json - プレイヤーのデフォルトルール設定
    └── enemy_rules.json  - 敵のデフォルトルール設定
```

### 🎯 クレート分離設計
- **アプリ層**: `hello-bevy` (root) - アプリケーション起動・DI的な役割
- **UI・システム層**: `bevy-ui` クレート - Bevy UIコンポーネント・システム・プラグイン統合
- **戦闘層**: `battle` クレート - チーム戦闘管理・戦闘ロジック
- **設定層**: `json-rule` クレート - JSON ルール読み込み・変換システム
- **計算層**: `action-system` クレート - トークンベース行動計算システム
- **UI Core層**: `ui-core` クレート - Bevy非依存のUIロジック
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

struct Team {
    // チーム管理
    name, members: Vec<Character>
}

enum TeamSide { Player, Enemy }

// battle/battle.rs  
struct TeamBattle {
    // チーム戦闘状態管理
    player_team, enemy_team, current_turn, current_character_index, current_team, battle_over, winner
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
- **`hello-bevy` (root)**: Bevyシステム統合・チーム設定("勇者パーティー","モンスター軍団")
- **`bevy-ui` クレート**: 汎用的なUI表示・入力処理・画面描画
- **`battle` クレート**: チーム戦闘管理・戦闘ロジック（TeamBattle）
- **`json-rule` クレート**: JSON読み込み・変換システム
- **`action-system` クレート**: AI行動決定・トークン処理・Character/Team型定義

### 4. 拡張性設計（クレート別）
- **新トークン追加**: `action-system` クレートの`ActionResolver`トレイト実装のみ
- **新アクション追加**: `action-system` クレートの`ActionType`enum拡張
- **UI変更**: `bevy-ui` クレートのみ修正で対応
- **チーム設定変更**: `hello-bevy` (root)のみ修正で対応
- **カスタムルール**: `json-rule` クレートでJSON外部ファイル読み込み（フォールバック機構付き）
- **戦闘ロジック変更**: `battle` クレートのみ修正で対応
- **チーム戦闘拡張**: TeamBattleクラスでマルチチーム対応済み

### 5. JSON設定システム（`json-rule` クレート）
```rust
// json-rule/rule_input_model.rs
RuleSet { rules: [RuleChain{ tokens: [TokenConfig] }] }
TokenConfig: Strike | Heal | Check{args} | GreaterThan{args} | etc.

// json-rule/rule_loader.rs
load_rules_from_file(path) -> parse_rules_from_json(content) -> convert_to_node_rules(rule_set)
```
- **入力モデル**: `rule_input_model.rs` - JSON入力専用データ構造定義
- **ファイル読み込み**: `load_rules_from_file(path)`
- **JSON解析**: `parse_rules_from_json(content)`
- **変換処理**: `convert_to_node_rules(rule_set)` → `action-system` ノードに変換
- **フォールバック**: JSON読み込み失敗時はハードコードルールを使用

## 🔄 データフロー設計（クレート間）
```
チーム戦闘システム:
入力 → bevy-ui クレート → battle クレート(TeamBattle) → action-system クレート → bevy-ui クレート → 結果表示
      ↑                   ↑                                                                     ↓
   hello-bevy (root)    json-rule クレート (JSON読み込み)                                   画面描画
```

## 📦 クレート依存関係ルール

### 依存関係の階層構造（ワークスペース）
```
hello-bevy (root バイナリ)
├── bevy-ui クレート
│   └── battle クレート
│       ├── json-rule クレート
│       │   └── action-system クレート
│       └── action-system クレート
├── ui-core クレート
└── 直接依存: action-system, json-rule, battle, bevy-ui
```

### クレート間依存関係の制約ルール

1. **階層依存のみ許可（循環依存回避）**
   - `hello-bevy` (root) → `bevy-ui`, `battle`, `json-rule`, `action-system` 依存可能
   - `bevy-ui` → `battle`, `ui-core` のみ依存
   - `battle` → `json-rule`, `action-system` 依存
   - `json-rule` → `action-system` のみ依存
   - `action-system` → 外部クレートのみ依存（完全独立）
   - **逆方向依存は禁止** (下位クレートが上位クレートに依存してはいけない)

2. **同一層内の相互依存は禁止**
   - `bevy-ui` ↔ `json-rule` の直接依存は禁止（`battle`経由で利用）

3. **Character/Team型の配置戦略**
   - `action-system` クレートに`Character`, `Team`, `TeamSide`型を配置（循環依存回避）
   - `battle` が `action-system::Character` を再エクスポート

4. **許可される依存パターン**
   ```rust
   // ✅ 許可
   hello-bevy → bevy-ui, battle, json-rule, action-system
   bevy-ui → battle, ui-core
   battle → json-rule, action-system
   json-rule → action-system
   
   // ❌ 禁止
   action-system → json-rule (逆方向)
   bevy-ui → json-rule (同一層)
   action-system → bevy-ui (逆方向)
   ```

5. **新クレート追加時のルール**
   - 依存関係を明確に定義してから実装開始
   - 循環依存が発生しないことを確認
   - より下位の層に配置できないか検討
   - ワークスペースのCargo.tomlに追加

## 🧪 テスト設計（クレート別）
### 統合テスト (59テスト)
- **`action-system` クレート**: 19テスト - アクションシステム・乱数テスト
  - ActionResolver, Token, 各種トークンの動作テスト
  - ActionCalculationSystemの統合テスト
  - **seed固定乱数テスト**: 複数seed・複数実行の検証
    - `test_multiple_seeds_produce_different_results`: 複数seedで異なる結果が出ることを検証
    - `test_same_seed_multiple_executions_can_differ`: 同一seedで複数回実行時のRNG状態変化検証
    - `test_single_rng_multiple_evaluations_differ`: RandomConditionNodeで1つのRNGでの複数評価検証
    - `test_single_rng_multiple_character_selections_vary`: RandomCharacterNodeで1つのRNGでの複数選択検証
- **`battle` クレート**: 3テスト - チーム戦闘専用テスト
  - TeamBattle, Team構造体のテスト
  - **チーム戦闘テスト**: TeamBattle, Team構造体のテスト
    - `test_team_battle_creation`: チーム戦闘作成テスト
    - `test_team_battle_turn_execution`: ターン実行テスト
    - `test_team_battle_complete_round`: 完全ラウンドテスト
- **`json-rule` クレート**: 12テスト - ルール読み込み・変換テスト
  - JSON読み込み・解析テスト
  - TokenConfig → ActionResolver変換テスト
  - エラーハンドリングテスト
- **`ui-core` クレート**: 19テスト - UIロジック・統合テスト
  - ルール管理・変換システムテスト
  - UIトークン変換・バリデーションテスト
  - ゲームステート管理テスト
- **`bevy-ui` クレート**: 6テスト - Bevy UI表示テスト
  - UI表示・フォーマットテスト
  - トークン表示テキストテスト
- その他クレート: 0テスト

### テスト実行方法
```bash
# 全クレートのテスト実行（推奨）
cargo test --workspace

# 個別クレートのテスト
cargo test -p action-system
cargo test -p battle
cargo test -p json-rule
cargo test -p ui-core
cargo test -p bevy-ui

# 特定テストパターン
cargo test -p action-system -- seed  # seed固定乱数テスト
cargo test -p battle -- integration_tests
cargo test -p battle -- team_battle  # チーム戦闘テスト
cargo test -p json-rule -- loader
```
