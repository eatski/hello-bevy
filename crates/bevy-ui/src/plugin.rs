// Game plugin for easy system registration
use bevy::prelude::*;

use crate::{
    load_font, setup_ui, setup_team_battle,
    handle_rule_editing, handle_battle_reset, handle_team_restart, handle_team_battle_input,
    handle_screenshot,
    apply_rules_to_battle, update_rule_display, update_token_inventory_display, 
    update_instruction_display, update_right_panel_visibility,
    update_team_battle_ui, update_team_latest_log_ui
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            // Startup systems
            .add_systems(Startup, (
                load_font,
                setup_ui,
                setup_team_battle,
            ).chain())
            
            // Update systems
            .add_systems(Update, (
                // Input handling
                handle_team_battle_input,
                handle_team_restart,
                handle_rule_editing,
                handle_battle_reset,
                handle_screenshot,
                
                // Game logic
                apply_rules_to_battle,
                
                // UI updates
                update_team_battle_ui,
                update_team_latest_log_ui,
                update_rule_display,
                update_token_inventory_display,
                update_instruction_display,
                update_right_panel_visibility,
            ));
    }
}