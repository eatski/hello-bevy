// Display text logic for UI tokens - Bevy UI specific

use ui_core::UITokenType;

pub trait UITokenDisplay {
    fn display_text(&self) -> &str;
}

impl UITokenDisplay for UITokenType {
    fn display_text(&self) -> &str {
        match self {
            UITokenType::Check => "Check",
            UITokenType::Strike => "Strike",
            UITokenType::Heal => "Heal",
            UITokenType::Number(n) => match n {
                50 => "50",
                _ => "Num",
            },
            UITokenType::ActingCharacter => "ActingChar",
            UITokenType::HP => "HP",
            UITokenType::GreaterThan => "L-gt-R",
            UITokenType::TrueOrFalse => "50/50",
        }
    }
}

// ルールトークンを整形した文字列に変換
pub fn format_rule_tokens(rule_row: &[UITokenType]) -> String {
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
    use super::UITokenDisplay;

    #[test]
    fn test_ui_token_display_text() {
        assert_eq!(UITokenType::Check.display_text(), "Check");
        assert_eq!(UITokenType::Strike.display_text(), "Strike");
        assert_eq!(UITokenType::Heal.display_text(), "Heal");
        assert_eq!(UITokenType::Number(50).display_text(), "50");
        assert_eq!(UITokenType::Number(100).display_text(), "Num");
        assert_eq!(UITokenType::ActingCharacter.display_text(), "ActingChar");
        assert_eq!(UITokenType::HP.display_text(), "HP");
        assert_eq!(UITokenType::GreaterThan.display_text(), "L-gt-R");
        assert_eq!(UITokenType::TrueOrFalse.display_text(), "50/50");
    }
    
    #[test]
    fn test_format_rule_tokens() {
        let empty_rule = vec![];
        assert_eq!(format_rule_tokens(&empty_rule), "(空)");
        
        let simple_rule = vec![UITokenType::Strike];
        assert_eq!(format_rule_tokens(&simple_rule), "Strike");
        
        let complex_rule = vec![
            UITokenType::Check,
            UITokenType::Number(50),
            UITokenType::GreaterThan,
            UITokenType::ActingCharacter,
            UITokenType::HP,
            UITokenType::Heal
        ];
        let formatted = format_rule_tokens(&complex_rule);
        assert_eq!(formatted, "Check → 50 → L-gt-R → ActingChar → HP → Heal");
    }
    
    #[test]
    fn test_ui_token_display_and_formatting() {
        let rule_tokens = vec![
            UITokenType::Check,
            UITokenType::Number(50),
            UITokenType::GreaterThan,
            UITokenType::ActingCharacter,
            UITokenType::HP,
            UITokenType::Heal
        ];
        
        let formatted = format_rule_tokens(&rule_tokens);
        assert_eq!(formatted, "Check → 50 → L-gt-R → ActingChar → HP → Heal");
        
        // Test empty rule
        let empty_rule = vec![];
        assert_eq!(format_rule_tokens(&empty_rule), "(空)");
        
        // Test single token
        let single_token = vec![UITokenType::Strike];
        assert_eq!(format_rule_tokens(&single_token), "Strike");
    }
}