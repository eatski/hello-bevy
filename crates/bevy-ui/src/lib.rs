// UI crate - Bevy UI components and systems

pub mod ui;
pub mod display_text;

// Re-export public types  
pub use ui::{GameFont, GameTeamBattle, BevyGameState, BevyCurrentRules, load_font, setup_ui, handle_rule_editing, update_rule_display, update_token_inventory_display, update_instruction_display, handle_battle_reset, update_right_panel_visibility, update_battle_info_display};

// Re-export ui-core types for convenience
pub use ui_core::{GameState, GameMode, CurrentRules, UITokenType, convert_ui_rules_to_nodes};
// Re-export display text functions
pub use display_text::format_rule_tokens;