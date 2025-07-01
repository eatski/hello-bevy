# 🛡️ 戦闘AIローグライク

プレイヤーがトークンベースの行動計算システムを組み上げて戦うRPGバトルゲームです。

## 🎮 ゲーム概要

Bevy Engineで開発されたRustベースのターンベースRPGバトルゲームです。プレイヤーは「勇者」として登場し、「スライム」などの敵と戦います。このゲームの最大の特徴は、従来のコマンド選択ではなく、プレイヤーが**トークンベースの行動計算システム**を設計してキャラクターの行動パターンを決定することです。

### 🎯 ゲームの特徴

- **戦闘システム**: ターンベースの戦略的なバトル（HP、MP、攻撃力の管理）
- **プログラマティック戦闘**: プレイヤーがトークンを組み合わせて行動ロジックを構築
- **戦略性**: 状況に応じた行動計算ロジックを構築する楽しさ
- **カスタマイズ性**: JSON設定ファイルで行動ルールをカスタマイズ可能
- **ワークスペース設計**: モジュラーなRustクレート構成で高い拡張性を実現

### 🎲 ゲームプレイフロー

1. **戦闘準備**: プレイヤーの行動ルールを設定（JSON設定またはデフォルト）
2. **ターンベース戦闘**: プレイヤーと敵が交互に行動
3. **行動計算**: 設定されたトークンルールに基づいて自動的に行動を決定
4. **戦闘結果**: HPが0になった方が敗北

## ⚙️ 行動計算システム

トークンを組み合わせることで、キャラクターの行動を決定するシステムです。

### 🔧 基本ルール

1. **複数行設定**: トークンを複数行に配置可能
2. **順次実行**: 1行目から順番に計算を実行
3. **アクション決定**: `break`せずに`Action`を決定できた行のアクションを実行
4. **失敗時**: どの行でも行動を決定できない場合は行動なし

### 📝 設定例

```javascript
[
    [Check(TrueOrFalseRandom), Heal],  // 50%の確率で回復
    [Attack]                           // 上記が失敗したら攻撃
]
```

### 🩺 HP条件での設定例

```javascript
[
    [Check(GreaterThanToken(Number(50), CharacterHP(ActingCharacter))), Check(TrueOrFalseRandom), Heal],  // HPが50未満かつ50%の確率で回復
    [Check(TrueOrFalseRandom), Strike]                                                                  // 50%の確率で攻撃
]
```

この設定により：
- HPが50未満の場合、50%の確率で回復を試行
- それ以外の場合、50%の確率で攻撃を実行
- 各行でアクションが決定されない場合は「何もしない」となる

### 🎯 標的指定の設定例

```javascript
[
    [Strike, RandomCharacter],        // ランダムな敵を攻撃
    [Heal, ActingCharacter]           // 自分を回復
]
```

**重要**: 標的指定は必須です。指定しない場合はエラーになります：

```javascript
[
    [Strike, ActingCharacter],     // 自分を攻撃
    [Heal, ActingCharacter]        // 自分を回復
]
```

## 📚 用語集

### 🎯 制御フロー

| 用語 | 説明 |
|------|------|
| **arguments** | トークンが受け取る引数。以降のトークンの計算結果を参照 |
| **continue** | 同じ行の次のトークンへ処理を継続 |
| **break** | 現在の行を中断し、次の行から計算を開始 |

### 🧩 トークンタイプ

#### 🔍 条件系
- **Check**: 引数が`True`なら`continue`、`False`なら`break`
- **TrueOrFalseRandom**: ランダムで`True`または`False`を返す
- **GreaterThanToken**: 2つの引数（数値）を比較して、最初が大きい（`>`） であれば `True` を返す

#### 固定値系
- **Number**: 特定の数値を返す（1~100）

#### 状況系
- **CharacterHP**: 引数のキャラクターのHPを返す
- **ActingCharacter**: ロジックを計算しているキャラクター自身を返す
- **TeamCharacters**: ロジックを計算しているキャラクターが所属するするチームのキャラクターの配列を返す

#### ⚔️ アクション系
キャラクターが実際に行動を実行（コスト不足時は`break`）

- **Strike**: 基本攻撃（オプション：target指定で標的選択）
- **Heal**: 回復魔法（MP消費、オプション：target指定で標的選択）

##### アクション標的指定例（JSON）
```json
{
  "type": "Strike",
  "target": {
    "type": "RandomCharacter"
  }
}
```

**注意**: JSON設定では標的指定は必須です。指定しない場合はエラーになります：
```json
{
  "type": "Heal",
  "target": {
    "type": "ActingCharacter"
  }
}
```

#### 配列系

- **Element**: 配列操作時に使用できる配列の要素
- **RandomPick**: 配列から1つ要素を取り出す
- **FilterList**: 配列から条件に当てはまる要素を絞る

##### JSON設定例
HPが50より小さい味方キャラクターからランダムに1人ヒールする計算式
ElementはFilterListの第一引数であるTeamCharactersの要素
```json
{
  "type": "Heal",
  "target": {
    "type": "RandomPick",
    "array": {
      "type": "FilterList",
      "array": {
        "type": "TeamCharacters"
      },
      "condition": {
        "type": "GreaterThan",
        "left": {
          "type": "Number",
          "value": 50
        },
        "right": {
          "type": "CharacterHP",
          "character": {
            "type": "Element"
          }
        }
      }
    }
  }
}
```


## 🚀 技術スタック

- **ゲームエンジン**: Bevy Engine (Rust)
- **言語**: Rust 2021 Edition
- **UI**: 日本語フォント対応のBevy UI
- **アーキテクチャ**: クリーンアーキテクチャに基づくワークスペース設計

### 🏗️ アーキテクチャ概要

このプロジェクトは責任分離の原則に基づいて7つのクレートに分割されています：

#### 🎮 `turn-based-rpg` (ルートバイナリ)
- **役割**: Bevyエンジン統合・ゲーム統合バイナリ
- **責任**: ゲーム固有設定（「勇者」「スライム」などの具体的なキャラクター設定）

#### 🖼️ `bevy-ui` クレート
- **役割**: Bevy UI コンポーネント・システム（Bevy依存）
- **責任**: Bevy固有のUI表示・入力処理・画面描画、文字列表示ロジック
- **特徴**: Bevy Engineに依存したUI実装

#### 🎨 `ui-core` クレート
- **役割**: UI中核ロジック（Bevy非依存）
- **責任**: ルール管理、トークン変換、ゲーム状態管理
- **特徴**: 完全にBevy非依存の汎用的なUIロジック

#### ⚔️ `battle` クレート
- **役割**: バトル管理・戦闘ロジック
- **責任**: 戦闘状態管理、ターン制御、戦闘結果判定
- **テスト**: 26の統合テストで戦闘ロジックを完全カバー

#### 🎯 `token-input` クレート
- **役割**: トークン入力統一化システム
- **責任**: FlatTokenInput（UI入力）とStructuredTokenInput（JSON入力）の変換・統合
- **特徴**: UI入力→FlatTokenInput→StructuredTokenInput→Node の統一変換パイプライン

#### 📝 `json-rule` クレート
- **役割**: JSON ルール読み込み・変換システム
- **責任**: 外部設定ファイルの読み込み、JSON解析
- **特徴**: フォールバック機構付きでJSON読み込み失敗時も動作継続

#### 🧠 `action-system` クレート
- **役割**: トークンベース行動計算システム
- **責任**: AI行動決定、トークン処理、Character型定義
- **特徴**: 完全に独立したクレートで他のクレートに依存しない

### 🔗 クレート依存関係

```
turn-based-rpg (root)
├── bevy-ui ← ui-core ← battle ← json-rule ← action-system
├── bevy-ui ← token-input ← action-system
├── ui-core ← token-input ← action-system
├── battle ← action-system
├── json-rule ← token-input ← action-system
├── token-input ← action-system
└── action-system (完全独立)
```

**循環依存回避**: 階層的な依存関係により循環依存を完全に排除
**関心の分離**: Bevy依存とBevy非依存のUIを明確に分離

## 🧪 テスト・ビルド

### テスト実行
```bash
# 全ワークスペースのテスト実行（推奨）
cargo test --workspace

# 個別クレートのテスト
cargo test -p action-system    # 32テスト
cargo test -p token-input      # 3テスト
cargo test -p json-rule        # 5テスト  
cargo test -p battle           # 3テスト
cargo test -p ui-core          # 16テスト
cargo test -p bevy-ui          # 3テスト
```

### ビルド・チェック
```bash
# 全ワークスペースの型チェック（推奨）
cargo check --workspace

# リリースビルド
cargo build --workspace --release
```

## 🎮 実行方法

```bash
# ゲーム実行
cargo run

# デバッグモードでの実行
cargo run --bin turn-based-rpg
```

## 📁 プロジェクト構成

```
├── Cargo.toml          # ワークスペース設定
├── src/main.rs         # ゲーム統合バイナリ
├── crates/             # 各機能クレート
│   ├── action-system/  # トークンベース行動計算
│   ├── token-input/    # トークン入力統一化
│   ├── json-rule/      # JSON設定読み込み
│   ├── battle/         # 戦闘管理ロジック
│   ├── ui-core/        # UIロジック（Bevy非依存）
│   └── bevy-ui/        # Bevy UIシステム
└── rules/              # JSON設定ファイル
    └── enemy_rules.json
```

## 🔧 カスタマイズ

### ルール設定のカスタマイズ
プレイヤーの行動パターンはUI上でトークンを組み合わせてカスタマイズできます。敵の行動パターンは `rules/enemy_rules.json` を編集して変更できます。

### 新しいトークンの追加
`action-system` クレートの `Token` トレイトを実装することで新しいトークンタイプを追加できます。

### UI のカスタマイズ
`bevy-ui` クレートを編集することで、ゲームの見た目や操作感をカスタマイズできます。
`ui-core` クレートを編集することで、UIロジックをカスタマイズできます。

