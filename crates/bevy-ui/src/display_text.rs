// Display text logic for UI tokens - Bevy UI specific

use token_input::FlatTokenInput;

pub trait UITokenDisplay {
    fn display_text(&self) -> String;
}

impl UITokenDisplay for FlatTokenInput {
    fn display_text(&self) -> String {
        token_input::FlatTokenInput::display_text(self)
    }
}

// ルールトークンを整形した文字列に変換
pub fn format_rule_tokens(rule_row: &[FlatTokenInput]) -> String {
    if rule_row.is_empty() {
        "(空)".to_string()
    } else {
        rule_row.iter()
            .map(|token| token.display_text())
            .collect::<Vec<_>>()
            .join(" → ")
    }
}

