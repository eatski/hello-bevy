// UI crate - Bevy UI components and systems

pub mod ui;
pub mod display_text;
pub mod systems;
pub mod plugin;
pub mod default_teams;
pub mod battle_display;

// Re-export public types  
pub use ui::{GameFont, GameTeamBattle, BevyGameState, BevyCurrentRules, load_font, setup_ui, handle_rule_editing, update_rule_display, update_battle_rule_display, update_token_inventory_display, update_instruction_display, handle_battle_reset, handle_screenshot, update_right_panel_visibility, update_rule_editor_position};

// Re-export system functions
pub use systems::{setup_team_battle, handle_team_restart, apply_rules_to_battle, handle_team_battle_input, update_team_battle_ui, update_team_latest_log_ui};

// Re-export plugin
pub use plugin::GamePlugin;

// Re-export ui-core types for convenience
pub use ui_core::{GameState, GameMode, CurrentRules, FlatTokenInput, convert_flat_rules_to_nodes};
// Re-export display text functions
pub use display_text::format_rule_tokens;