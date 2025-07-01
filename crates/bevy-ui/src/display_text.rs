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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ui_token_display_text() {
        assert_eq!(FlatTokenInput::Check.display_text(), "Check");
        assert_eq!(FlatTokenInput::Strike.display_text(), "Strike");
        assert_eq!(FlatTokenInput::Heal.display_text(), "Heal");
        assert_eq!(FlatTokenInput::Number(50).display_text(), "50");
        assert_eq!(FlatTokenInput::Number(100).display_text(), "Num");
        assert_eq!(FlatTokenInput::ActingCharacter.display_text(), "ActingChar");
        assert_eq!(FlatTokenInput::AllCharacters.display_text(), "AllChars");
        assert_eq!(FlatTokenInput::RandomPick.display_text(), "RandomPick");
        assert_eq!(FlatTokenInput::HP.display_text(), "HP");
        assert_eq!(FlatTokenInput::GreaterThan.display_text(), "L-gt-R");
        assert_eq!(FlatTokenInput::TrueOrFalse.display_text(), "50/50");
    }
    
    #[test]
    fn test_format_rule_tokens() {
        let empty_rule = vec![];
        assert_eq!(format_rule_tokens(&empty_rule), "(空)");
        
        let simple_rule = vec![FlatTokenInput::Strike];
        assert_eq!(format_rule_tokens(&simple_rule), "Strike");
        
        let complex_rule = vec![
            FlatTokenInput::Check,
            FlatTokenInput::Number(50),
            FlatTokenInput::GreaterThan,
            FlatTokenInput::ActingCharacter,
            FlatTokenInput::HP,
            FlatTokenInput::Heal
        ];
        let formatted = format_rule_tokens(&complex_rule);
        assert_eq!(formatted, "Check → 50 → L-gt-R → ActingChar → HP → Heal");
    }
    
    #[test]
    fn test_ui_token_display_and_formatting() {
        let rule_tokens = vec![
            FlatTokenInput::Check,
            FlatTokenInput::Number(50),
            FlatTokenInput::GreaterThan,
            FlatTokenInput::ActingCharacter,
            FlatTokenInput::HP,
            FlatTokenInput::Heal
        ];
        
        let formatted = format_rule_tokens(&rule_tokens);
        assert_eq!(formatted, "Check → 50 → L-gt-R → ActingChar → HP → Heal");
        
        // Test empty rule
        let empty_rule = vec![];
        assert_eq!(format_rule_tokens(&empty_rule), "(空)");
        
        // Test single token
        let single_token = vec![FlatTokenInput::Strike];
        assert_eq!(format_rule_tokens(&single_token), "Strike");
    }
}