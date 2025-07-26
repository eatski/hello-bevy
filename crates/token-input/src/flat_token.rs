// FlatTokenInput - UI入力用の平坦なトークン定義（ui-coreから移行）

#[derive(Clone, Debug, PartialEq)]
pub enum FlatTokenInput {
    Check,
    Strike,
    Heal,
    Number(u32),
    ActingCharacter,  // 行動するキャラクター
    AllCharacters,    // 全キャラクター配列
    TeamMembers,      // チームメンバー配列
    RandomPick,       // ランダム選択
    CharacterToHp, // CharacterからCharacterHP型の値を取得
    CharacterHpToCharacter, // CharacterHPからCharacterを取得
    GreaterThan,
    LessThan,
    TrueOrFalse,
    FilterList,       // リストフィルタリング
    Map,              // 配列マッピング
    Eq,               // 等価比較
    CharacterTeam,    // キャラクターのチーム取得
    Element,          // 現在の要素（フィルタリング中のキャラクター）
    Enemy,            // 敵チーム定数
    Hero,             // 味方チーム定数
    Max,              // 配列の最大値
    Min,              // 配列の最小値
}

// 表示テキストは元のUITokenTypeと同じ
impl FlatTokenInput {
    pub fn display_text(&self) -> String {
        match self {
            FlatTokenInput::Check => "Check".to_string(),
            FlatTokenInput::Strike => "Strike".to_string(),
            FlatTokenInput::Heal => "Heal".to_string(),
            FlatTokenInput::Number(n) => match n {
                50 => "50".to_string(),
                _ => "Num".to_string(),
            },
            FlatTokenInput::ActingCharacter => "ActingChar".to_string(),
            FlatTokenInput::AllCharacters => "AllChars".to_string(),
            FlatTokenInput::TeamMembers => "TeamMembers".to_string(),
            FlatTokenInput::RandomPick => "RandomPick".to_string(),
            FlatTokenInput::CharacterToHp => "CharToHp".to_string(),
            FlatTokenInput::CharacterHpToCharacter => "CharHpToChar".to_string(),
            FlatTokenInput::GreaterThan => "L-gt-R".to_string(),
            FlatTokenInput::LessThan => "L-lt-R".to_string(),
            FlatTokenInput::TrueOrFalse => "50/50".to_string(),
            FlatTokenInput::FilterList => "FilterList".to_string(),
            FlatTokenInput::Map => "Map".to_string(),
            FlatTokenInput::Eq => "Eq".to_string(),
            FlatTokenInput::CharacterTeam => "CharTeam".to_string(),
            FlatTokenInput::Element => "Element".to_string(),
            FlatTokenInput::Enemy => "Enemy".to_string(),
            FlatTokenInput::Hero => "Hero".to_string(),
            FlatTokenInput::Max => "Max".to_string(),
            FlatTokenInput::Min => "Min".to_string(),
        }
    }
}