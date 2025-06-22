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
- **JSON設定**: キャラクターのruleはJSON外部ファイルから読み込み可能になった（フォールバック機構付き）

## 🏗️ アーキテクチャ設計

### 📁 ファイル構成
```
src/
├── main.rs             - Bevyエンジン統合・ゲーム固有設定
├── ui.rs               - 汎用的なUI表示・入力処理
├── battle_system.rs    - バトル管理・キャラクター定義
├── action_system.rs    - トークンベース行動計算システム
├── rule_loader.rs      - JSON形式ルール読み込み・変換機能
└── rule_input_model.rs - JSON入力用ルールデータモデル定義
rules/
├── player_rules.json - プレイヤーのデフォルトルール設定
└── enemy_rules.json  - 敵のデフォルトルール設定
```

### 🎯 モジュール分離設計
- **アプリ層**: `main.rs` - Bevyエンジン統合・ゲーム固有設定
- **UI層**: `ui.rs` - 汎用的なUI表示・入力処理
- **ドメイン層**: `battle_system.rs` - ゲームロジック
- **計算層**: `action_system.rs` - AI行動システム
- **設定層**: `rule_loader.rs` - JSON外部ルール読み込み
- **入力モデル層**: `rule_input_model.rs` - JSON入力用データ構造定義

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
- **カスタムルール**: JSON外部ファイルからルール設定可能（フォールバック機構付き）

### 5. JSON設定システム
```rust
// rule_input_model.rs
RuleSet { rules: [RuleChain{ tokens: [TokenConfig] }] }
TokenConfig: Strike | Heal | Check{args} | GreaterThan{args} | etc.

// rule_loader.rs
load_rules_from_file(path) -> parse_rules_from_json(content) -> convert_to_token_rules(rule_set)
```
- **入力モデル**: `rule_input_model.rs` - JSON入力専用データ構造定義
- **ファイル読み込み**: `load_rules_from_file(path)`
- **JSON解析**: `parse_rules_from_json(content)`
- **変換処理**: `convert_to_token_rules(rule_set)`
- **フォールバック**: JSON読み込み失敗時はハードコードルールを使用

## 🔄 データフロー設計
```
入力 → ui.rs → battle_system.rs → action_system.rs → ui.rs → 結果表示
      ↑                                                         ↓
   main.rs (ゲーム設定)                                    画面描画
```

## 📦 モジュール依存関係ルール

### 依存関係の階層構造
```
main.rs
├── ui.rs (UI層)
├── battle_system.rs (ドメイン層)
│   └── action_system.rs (計算層)
├── rule_loader.rs (設定層)
│   └── rule_input_model.rs (入力モデル層)
└── rule_input_model.rs (入力モデル層)
```

### 依存関係の制約ルール

1. **上位層 → 下位層の依存のみ許可**
   - `main.rs` → 全モジュール依存可能
   - `battle_system.rs` → `action_system.rs`のみ依存
   - `rule_loader.rs` → `rule_input_model.rs` + `action_system.rs`依存
   - **逆方向依存は禁止** (下位層が上位層に依存してはいけない)

2. **同一層内の相互依存は禁止**
   - `ui.rs` ↔ `battle_system.rs`の直接依存は禁止
   - `rule_loader.rs` ↔ `ui.rs`の直接依存は禁止

3. **独立性を保つモジュール**
   - `rule_input_model.rs`: 他モジュールに依存しない完全独立
   - `action_system.rs`: `battle_system.rs`にのみ依存（Characterの型定義）

4. **許可される依存パターン**
   ```rust
   // ✅ 許可
   main.rs → ui.rs, battle_system.rs, rule_loader.rs
   battle_system.rs → action_system.rs
   rule_loader.rs → rule_input_model.rs, action_system.rs
   
   // ❌ 禁止
   action_system.rs → battle_system.rs (逆方向)
   ui.rs → rule_loader.rs (同一層)
   rule_input_model.rs → any module (完全独立)
   ```

5. **新モジュール追加時のルール**
   - 依存関係を明確に定義してから実装開始
   - 循環依存が発生しないことを確認
   - より下位の層に配置できないか検討

## 🧪 テスト設計
### 統合テスト (31テスト)
- **action_system.rs**: 単体テスト + 統合テスト
- **battle_system.rs**: 単体テスト + 様々なルールパターンテスト
- **ルールパターン例**:
  - 攻撃専用/回復専用ルール
  - 複雑なルールチェイン
  - 空ルール・決定論的実行テスト
