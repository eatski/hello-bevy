// Rule management logic - independent of Bevy

use battle::RuleNode;
use token_input::{FlatTokenInput, convert_flat_rules_to_nodes};

#[derive(Default, Clone, Debug)]
pub struct CurrentRules {
    pub rules: Vec<Vec<FlatTokenInput>>,
    pub selected_row: usize,
}

impl CurrentRules {
    pub fn new() -> Self {
        Self {
            rules: vec![
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
            ],
            selected_row: 0,
        }
    }
    
    pub fn with_rules(rules: Vec<Vec<FlatTokenInput>>) -> Self {
        Self {
            rules,
            selected_row: 0,
        }
    }

    // UIのFlatTokenInputからtoken-inputを経由してaction-systemのRuleNodeに変換
    pub fn convert_to_rule_nodes(&self) -> Vec<RuleNode> {
        convert_flat_rules_to_nodes(&self.rules)
    }
    
    // ルール行の追加
    pub fn add_token_to_current_row(&mut self, token: FlatTokenInput) {
        if self.selected_row < self.rules.len() {
            self.rules[self.selected_row].push(token);
        }
    }
    
    // 現在の行からトークンを削除
    pub fn remove_last_token_from_current_row(&mut self) {
        if self.selected_row < self.rules.len() {
            self.rules[self.selected_row].pop();
        }
    }
    
    // 現在の行をクリア
    pub fn clear_current_row(&mut self) {
        if self.selected_row < self.rules.len() {
            self.rules[self.selected_row].clear();
        }
    }
    
    // 全てのルールをクリア
    pub fn clear_all(&mut self) {
        for rule_row in &mut self.rules {
            rule_row.clear();
        }
        self.selected_row = 0;
    }
    
    // 行の選択
    pub fn select_row(&mut self, row: usize) {
        if row < self.rules.len() {
            self.selected_row = row;
        }
    }
    
    // 次の行に移動
    pub fn select_next_row(&mut self) {
        if self.selected_row + 1 < self.rules.len() {
            self.selected_row += 1;
        }
    }
    
    // 前の行に移動
    pub fn select_previous_row(&mut self) {
        if self.selected_row > 0 {
            self.selected_row -= 1;
        }
    }
    
    // 現在の行が空かどうか
    pub fn is_current_row_empty(&self) -> bool {
        if let Some(current_row) = self.rules.get(self.selected_row) {
            current_row.is_empty()
        } else {
            true
        }
    }
    
    // 空でない行の数
    pub fn non_empty_rule_count(&self) -> usize {
        self.rules.iter().filter(|row| !row.is_empty()).count()
    }
    
    // ルールが有効かどうか（少なくとも1つの非空行がある）
    pub fn has_valid_rules(&self) -> bool {
        self.non_empty_rule_count() > 0
    }
}

// Note: String formatting logic moved to bevy-ui crate

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_current_rules_creation() {
        let rules = CurrentRules::new();
        assert_eq!(rules.rules.len(), 5);
        assert_eq!(rules.selected_row, 0);
        assert_eq!(rules.is_current_row_empty(), true);
        assert_eq!(rules.has_valid_rules(), false);
    }
    
    #[test]
    fn test_token_manipulation() {
        let mut rules = CurrentRules::new();
        
        // Add tokens to current row
        rules.add_token_to_current_row(FlatTokenInput::Strike);
        rules.add_token_to_current_row(FlatTokenInput::ActingCharacter);
        rules.add_token_to_current_row(FlatTokenInput::Heal);
        rules.add_token_to_current_row(FlatTokenInput::ActingCharacter);
        
        assert_eq!(rules.rules[0].len(), 4);
        assert_eq!(rules.is_current_row_empty(), false);
        assert_eq!(rules.has_valid_rules(), true);
        
        // Remove last token
        rules.remove_last_token_from_current_row();
        assert_eq!(rules.rules[0].len(), 3);
        
        // Clear current row
        rules.clear_current_row();
        assert_eq!(rules.is_current_row_empty(), true);
        assert_eq!(rules.has_valid_rules(), false);
    }
    
    #[test]
    fn test_row_navigation() {
        let mut rules = CurrentRules::new();
        
        assert_eq!(rules.selected_row, 0);
        
        rules.select_next_row();
        assert_eq!(rules.selected_row, 1);
        
        rules.select_previous_row();
        assert_eq!(rules.selected_row, 0);
        
        // Test bounds
        rules.select_row(10); // Out of bounds
        assert_eq!(rules.selected_row, 0); // Should not change
        
        rules.select_row(4); // Valid row
        assert_eq!(rules.selected_row, 4);
        
        rules.select_next_row(); // At max, should not change
        assert_eq!(rules.selected_row, 4);
    }
    
    #[test]
    fn test_rule_count_and_validation() {
        let mut rules = CurrentRules::new();
        
        assert_eq!(rules.non_empty_rule_count(), 0);
        assert_eq!(rules.has_valid_rules(), false);
        
        // Add tokens to first row
        rules.add_token_to_current_row(FlatTokenInput::Strike);
        rules.add_token_to_current_row(FlatTokenInput::RandomCharacter);
        assert_eq!(rules.non_empty_rule_count(), 1);
        assert_eq!(rules.has_valid_rules(), true);
        
        // Add tokens to second row
        rules.select_next_row();
        rules.add_token_to_current_row(FlatTokenInput::Heal);
        rules.add_token_to_current_row(FlatTokenInput::ActingCharacter);
        assert_eq!(rules.non_empty_rule_count(), 2);
        
        // Clear all
        rules.clear_all();
        assert_eq!(rules.non_empty_rule_count(), 0);
        assert_eq!(rules.has_valid_rules(), false);
        assert_eq!(rules.selected_row, 0);
    }
    
    #[test]
    fn test_complex_rule_creation() {
        let mut rules = CurrentRules::new();
        
        // Create a complex rule pattern
        rules.add_token_to_current_row(FlatTokenInput::Check);
        rules.add_token_to_current_row(FlatTokenInput::GreaterThan);
        rules.add_token_to_current_row(FlatTokenInput::Number(50));
        rules.add_token_to_current_row(FlatTokenInput::HP);
        rules.add_token_to_current_row(FlatTokenInput::ActingCharacter);
        rules.add_token_to_current_row(FlatTokenInput::Heal);
        rules.add_token_to_current_row(FlatTokenInput::ActingCharacter);
        
        let rule_nodes = rules.convert_to_rule_nodes();
        assert_ne!(rule_nodes.len(), 0, "Should convert to valid rule nodes");
    }
    
    // Note: String formatting tests moved to bevy-ui crate
    
    #[test]
    fn test_with_rules_constructor() {
        let initial_rules = vec![
            vec![FlatTokenInput::Strike],
            vec![FlatTokenInput::Heal],
            vec![],
        ];
        
        let rules = CurrentRules::with_rules(initial_rules.clone());
        assert_eq!(rules.rules, initial_rules);
        assert_eq!(rules.selected_row, 0);
        assert_eq!(rules.non_empty_rule_count(), 2);
        assert_eq!(rules.has_valid_rules(), true);
    }
}