# hello-bevy 設計サマリ

## 🚀 最新アップデート (GameNumeric trait統一化)
### 設計変更サマリ
- **GameNumeric trait**: CharacterHPとi32値を統一的に扱うtraitを新規追加
  - Max, Min, GreaterThan等の数値演算で型混在をサポート
  - `crates/action-system/src/core/game_numeric.rs` に実装
  - **YAGNI原則適用**: 未使用の`from_i32()`メソッドを削除し、シンプルな設計に変更
- **統一化ノード**: GameNumericMaxNode, GameNumericMinNode, GameNumericGreaterThanNodeを追加
  - 既存のMax/MinノードはAPI後方互換性を維持
  - CharacterHPとi32の両方を同じインターフェースで処理可能
- **トークン拡張**: GameNumericMax, GameNumericMin トークンをUI入力システムに追加
  - FlatTokenInput, StructuredTokenInputの両方をサポート
- **型安全性**: CharacterHP vs i32 の比較演算も統一的に処理
- **テスト追加**: GameNumeric trait の機能テスト (crates/action-system/src/core/game_numeric.rs:43-79)

### ファイル変更箇所
- 新規: `crates/action-system/src/core/game_numeric.rs` - GameNumeric trait定義
- 新規: `crates/action-system/src/nodes/array/game_numeric_max_min_node.rs` - 統一Max/Minノード
- 新規: `crates/action-system/src/nodes/condition/game_numeric_greater_than_node.rs` - 統一GreaterThanノード
- 更新: `crates/token-input/src/flat_token.rs` - GameNumericMax/Min トークン追加
- 更新: `crates/token-input/src/structured_token.rs` - 構造化トークン拡張
- 更新: `crates/token-input/src/structured_to_node.rs` - 変換ロジック拡張
- 更新: 各種mod.rs, lib.rs - エクスポート追加

## 📝　重要
タスク完了時に必ず以下を実施するように事前にタスク化すること
- crates/ui-core/src/integration_tests.rs にテストケースの追加（必要に応じて）
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
cargo test -p action-system
cargo test -p token-input
cargo test -p json-rule
cargo test -p battle
cargo test -p ui-core
cargo test -p bevy-ui

# ドキュメンテーションテスト
cargo test --workspace --doc

# リリースビルド
cargo build --workspace --release
```


## 🗣️ 開発ガイドライン

### ❌ してはいけないこと
- **フォールバック使用**: エラーの無視、フォールバック機構の使用は完全に禁止
- **Silent failure**: エラーを隠蔽する実装は禁止
- **循環依存**: クレート間の循環依存を作成すること
- **同一層依存**: 同じ階層レベルのクレート間で相互依存すること
- **1v1戦闘実装**: 1vs1戦闘システムは完全に削除済み、再実装禁止
- **UI直接変換**: UIからaction-systemへの直接変換は禁止
- **Bevy依存の混在**: ui-coreにBevy依存コードを追加すること
- **コンパイル確認なし**: 変更後にcargo checkを実行しないこと

### ✅ 必ずやるべきこと
- **厳密なエラーハンドリング**: 全てのエラーケースで適切な処理を実装
- **統一パイプライン使用**: UI入力→FlatTokenInput→StructuredTokenInput→Nodeの変換パイプライン利用
- **クレート分離原則**: 各クレートの責任境界を明確に保つ
- **チーム戦闘統一**: TeamBattleクラス、Team構造体を使用した戦闘システム実装
- **原子的トークン設計**: ActingCharacterとHPを個別トークンとして管理
- **ID指定ターゲティング**: ActionトレイトのtargetをIDで指定する実装
- **設定可能ターゲット**: StrikeとHealアクションで標的をUI/JSONから設定可能に
- **統合テスト追加**: 新機能実装時はcrates/ui-core/src/integration_tests.rsに追加
- **ドキュメント更新**: 設計変更、新機能追加時はこのCLAUDE.mdを更新

### 🔧 開発プロセス
- **コンパイル確認**: 変更後は必ず`cargo check --workspace`を実行
- **テスト実行**: 全crateのテストを`cargo test --workspace`で実行
- **UI分離**: 具体的なキャラクター設定はmain.rsに、汎用的なUIロジックはui.rsに分離
- **JSON設定**: キャラクターのruleはJSON外部ファイルから読み込み可能
- **main.rs役割**: 起動処理のみに集中、具体的なロジックをbevy-uiクレートに委譲（DI的なアーキテクチャ）

## 🏗️ アーキテクチャ設計

### 📁 ファイル構成（クレート分割後）
```
├── Cargo.toml          - ワークスペース設定
├── src/
│   └── main.rs         - アプリケーション起動（DI的な役割）
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
│   ├── token-input/    - トークン入力統一化システム
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs              - クレートエントリポイント
│   │       ├── flat_token.rs       - FlatTokenInput定義（UI入力用）
│   │       ├── structured_token.rs - StructuredTokenInput定義（JSON入力用）
│   │       └── converter.rs        - 変換ロジック統合
│   ├── json-rule/      - JSON ルール読み込み・変換システム
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs              - クレートエントリポイント
│   │       └── rule_loader.rs      - JSON形式ルール読み込み
│   ├── battle/         - バトル管理・戦闘ロジック
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs  - クレートエントリポイント
│   │       └── battle.rs - バトル管理ロジック
│   ├── ui-core/        - Bevy非依存UIロジック
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs          - クレートエントリポイント
│   │       ├── game_state.rs   - ゲーム状態管理
│   │       ├── rule_management.rs - ルール管理ロジック
│   │       └── integration_tests.rs - 統合テスト
│   └── bevy-ui/        - Bevy UIコンポーネント・システム・プラグイン
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs     - クレートエントリポイント
│           ├── ui.rs      - UI表示・コンポーネント定義
│           ├── systems.rs - ゲームシステム実装
│           ├── display_text.rs - UI表示テキスト管理
│           └── plugin.rs  - Bevyプラグイン統合
└── rules/
    └── enemy_rules.json  - 敵のデフォルトルール設定
```

### 🎯 クレート分離設計
- **アプリ層**: `turn-based-rpg` (root) - アプリケーション起動・DI的な役割
- **UI・システム層**: `bevy-ui` クレート - Bevy UIコンポーネント・システム・プラグイン統合
- **UI Core層**: `ui-core` クレート - Bevy非依存のUIロジック
- **戦闘層**: `battle` クレート - チーム戦闘管理・戦闘ロジック
- **設定層**: `json-rule` クレート - JSON ルール読み込み・変換システム
- **トークン統一層**: `token-input` クレート - FlatTokenInput⇔StructuredTokenInput変換統合
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

// action-system/character_hp.rs
struct CharacterHP {
    // HPの値とそのキャラクターを保持
    character: Character,
    hp_value: i32,
    // 数値演算・比較演算サポート
    // HpCharacterNodeでCharacterを取得可能
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
UI入力 → bevy-ui → ui-core → token-input(FlatTokenInput→StructuredTokenInput) → action-system → 結果表示
JSON入力 → json-rule → token-input(StructuredTokenInput) → action-system → battle → bevy-ui → 画面描画
         ↑                                                                 ↑              ↓
    turn-based-rpg (root)                                          battle クレート      表示レンダリング
```

## 📦 クレート依存関係ルール

### 依存関係の階層構造（ワークスペース）
```
turn-based-rpg (root バイナリ)
├── bevy-ui クレート
│   ├── ui-core クレート
│   │   └── token-input クレート
│   │       └── action-system クレート
│   ├── battle クレート
│   │   └── action-system クレート
│   ├── json-rule クレート
│   │   └── token-input クレート
│   └── token-input クレート
└── 直接依存: action-system, token-input, json-rule, battle, ui-core, bevy-ui
```

### クレート間依存関係の制約ルール

1. **階層依存のみ許可（循環依存回避）**
   - `turn-based-rpg` (root) → 全クレート依存可能
   - `bevy-ui` → `ui-core`, `battle`, `json-rule`, `token-input` 依存
   - `ui-core` → `token-input` のみ依存
   - `battle` → `action-system` のみ依存
   - `json-rule` → `token-input` のみ依存
   - `token-input` → `action-system` のみ依存
   - `action-system` → 外部クレートのみ依存（完全独立）
   - **逆方向依存は禁止** (下位クレートが上位クレートに依存してはいけない)

2. **同一層内の相互依存は禁止**
   - 同じ階層レベルのクレート間の直接依存は禁止

3. **Character/Team型の配置戦略**
   - `action-system` クレートに`Character`, `Team`, `TeamSide`型を配置（循環依存回避）
   - `battle` が `action-system::Character` を再エクスポート

4. **トークン変換の統一化**
   - `token-input` クレートで`FlatTokenInput`と`StructuredTokenInput`を統合管理
   - UI入力とJSON入力の両方を統一パイプラインで処理

5. **許可される依存パターン**
   ```rust
   // ✅ 許可
   turn-based-rpg → bevy-ui, ui-core, battle, json-rule, token-input, action-system
   bevy-ui → ui-core, battle, json-rule, token-input
   ui-core → token-input
   battle → action-system
   json-rule → token-input
   token-input → action-system
   
   // ❌ 禁止
   action-system → token-input (逆方向)
   token-input → json-rule (逆方向)
   ui-core → battle (同一層)
   ```

5. **新クレート追加時のルール**
   - 依存関係を明確に定義してから実装開始
   - 循環依存が発生しないことを確認
   - より下位の層に配置できないか検討
   - ワークスペースのCargo.tomlに追加

## 🧪 テスト設計（クレート別）
### 統合テスト (144テスト)
- **`action-system` クレート**: 85テスト - アクションシステム・乱数テスト
  - ActionResolver, Token, 各種トークンの動作テスト
  - ActionCalculationSystemの統合テスト
  - **seed固定乱数テスト**: 複数seed・複数実行の検証
    - `test_multiple_seeds_produce_different_results`: 複数seedで異なる結果が出ることを検証
    - `test_same_seed_multiple_executions_can_differ`: 同一seedで複数回実行時のRNG状態変化検証
    - `test_single_rng_multiple_evaluations_differ`: RandomConditionNodeで1つのRNGでの複数評価検証
    - `test_single_rng_multiple_character_selections_vary`: RandomCharacterNodeで1つのRNGでの複数選択検証
- **`token-input` クレート**: 17テスト - トークン変換テスト
  - FlatTokenInput → StructuredTokenInput変換テスト
  - StructuredTokenInput → Node変換テスト
  - 統合変換パイプラインテスト
- **`battle` クレート**: 3テスト - チーム戦闘専用テスト
  - TeamBattle, Team構造体のテスト
  - **チーム戦闘テスト**: TeamBattle, Team構造体のテスト
    - `test_team_battle_creation`: チーム戦闘作成テスト
    - `test_team_battle_turn_execution`: ターン実行テスト
    - `test_team_battle_complete_round`: 完全ラウンドテスト
- **`json-rule` クレート**: 5テスト - ルール読み込み・変換テスト
  - JSON読み込み・解析テスト
  - RuleSet → ActionResolver変換テスト
  - エラーハンドリングテスト
- **`ui-core` クレート**: 31テスト - UIロジック・**エンドツーエンド統合テスト**
  - **統合テスト (22テスト)**: UIから入力したトークンで実際の戦闘を実行し、結果を検証
    - `test_basic_strike_ui_to_battle_integration`: 基本攻撃の実行と敵へのダメージ検証
    - `test_heal_ui_to_battle_integration`: 回復の実行とHP回復検証
    - `test_conditional_strike_ui_to_battle_integration`: 条件付き攻撃の実行検証
    - `test_low_hp_no_action_ui_to_battle_integration`: 条件不満時のアクション無実行検証
    - `test_target_specific_strike_ui_to_battle_integration`: 特定ターゲット攻撃検証
    - `test_multi_character_battle_ui_to_battle_integration`: 複数キャラクター戦闘検証
    - `test_team_vs_team_battle_ui_to_battle_integration`: チーム対チーム戦闘検証
    - `test_ui_rule_creation_to_battle_workflow`: UI規則作成→戦闘実行の完全ワークフロー検証
    - `test_multiple_rules_ui_to_battle_integration`: 複数ルール協働検証
    - `test_battle_completion_ui_to_battle_integration`: 戦闘終了検証
    - `test_empty_rules_ui_to_battle_integration`: 空ルール時の挙動検証
    - `test_complex_conditional_combinations_ui_to_battle_integration`: 複雑条件組み合わせ検証
    - `test_hp_threshold_variations_ui_to_battle_integration`: HP閾値バリエーション検証
    - `test_mp_constraint_healing_ui_to_battle_integration`: MP制約回復検証
    - `test_zero_hp_character_exclusion_ui_to_battle_integration`: 倒れたキャラクター除外検証
    - `test_random_pick_consistency_ui_to_battle_integration`: ランダム選択一貫性検証
    - `test_boundary_values_ui_to_battle_integration`: 境界値検証
    - `test_max_hp_characters_ui_to_battle_integration`: 最大HP時の挙動検証
    - `test_min_values_ui_to_battle_integration`: Min関数検証
    - `test_character_team_filtering_ui_to_battle_integration`: チームフィルタリング検証
    - `test_sequential_rule_execution_ui_to_battle_integration`: 順次ルール実行検証
    - `test_extended_battle_duration_ui_to_battle_integration`: 長期戦闘検証
    - `test_character_hp_type_integration`: CharacterHP型の機能検証
  - ゲームステート管理・ルール管理テスト (8テスト)
- **`bevy-ui` クレート**: 3テスト - Bevy UI表示テスト
  - UI表示・フォーマットテスト
  - トークン表示テキストテスト

### テスト実行方法
```bash
# 全クレートのテスト実行（推奨）
cargo test --workspace

# 個別クレートのテスト
cargo test -p action-system
cargo test -p token-input
cargo test -p battle
cargo test -p json-rule
cargo test -p ui-core
cargo test -p bevy-ui

# 特定テストパターン
cargo test -p action-system -- seed  # seed固定乱数テスト
cargo test -p token-input -- converter  # 変換テスト
cargo test -p battle -- integration_tests
cargo test -p battle -- team_battle  # チーム戦闘テスト
cargo test -p json-rule -- loader
```
