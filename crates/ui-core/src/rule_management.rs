// Rule management logic - independent of Bevy

use action_system::RuleNode;
use token_input::{FlatTokenInput, convert_flat_to_structured, compiler::Compiler};

#[derive(Default, Clone, Debug)]
pub struct CurrentRules {
    pub rules: Vec<Vec<FlatTokenInput>>,
    pub selected_row: usize,
}

impl CurrentRules {
    pub fn new() -> Self {
        Self {
            rules: vec![
                // Default rule: Strike → RandomPick → AllCharacters
                vec![FlatTokenInput::Strike, FlatTokenInput::RandomPick, FlatTokenInput::AllCharacters],
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
        let compiler = Compiler::new();
        
        self.rules
            .iter()
            .filter(|rule_row| !rule_row.is_empty())
            .filter_map(|rule_row| {
                match convert_flat_to_structured(rule_row) {
                    Ok(structured_tokens) => {
                        if structured_tokens.is_empty() {
                            return None;
                        }
                        
                        // コンパイラを使用してStructuredTokenInputをRuleNodeに変換
                        match compiler.compile(&structured_tokens[0]) {
                            Ok(rule_node) => Some(rule_node),
                            Err(_) => None,
                        }
                    }
                    Err(_) => None,
                }
            })
            .collect()
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


#[cfg(test)]
mod tests {
    use super::*;

    
    #[test]
    fn test_token_manipulation() {
        let mut rules = CurrentRules::new();
        
        // Clear default rule and add tokens to current row
        rules.clear_current_row();
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
        
        assert_eq!(rules.non_empty_rule_count(), 1); // Default rule exists
        assert_eq!(rules.has_valid_rules(), true); // Default rule is valid
        
        // Clear first row and add new tokens
        rules.clear_current_row();
        rules.add_token_to_current_row(FlatTokenInput::Strike);
        rules.add_token_to_current_row(FlatTokenInput::RandomPick);
        rules.add_token_to_current_row(FlatTokenInput::AllCharacters);
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
        rules.add_token_to_current_row(FlatTokenInput::CharacterToHp);
        rules.add_token_to_current_row(FlatTokenInput::ActingCharacter);
        rules.add_token_to_current_row(FlatTokenInput::Heal);
        rules.add_token_to_current_row(FlatTokenInput::ActingCharacter);
        
        let rule_nodes = rules.convert_to_rule_nodes();
        assert_ne!(rule_nodes.len(), 0, "Should convert to valid rule nodes");
    }
    
    // Note: String formatting tests moved to bevy-ui crate
    
}