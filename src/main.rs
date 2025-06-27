use bevy::prelude::*;
use rand::rngs::StdRng;
use rand::SeedableRng;

use bevy_frontend::{GameBattle, CurrentRules, GameState, GameMode, load_font, setup_ui, handle_battle_input, update_battle_ui, update_log_ui, update_latest_log_ui, handle_rule_editing, update_rule_display, update_token_inventory_display, update_instruction_display, handle_battle_reset, update_right_panel_visibility, update_battle_info_display};
use game_logic::{Battle, Character as GameCharacter};
use rule_parser::{load_rules_from_file, convert_to_node_rules};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (load_font, setup_ui, setup_battle).chain())
        .add_systems(Update, (
            handle_battle_input, 
            handle_restart, 
            handle_rule_editing,
            handle_battle_reset,
            apply_rules_to_battle,
            update_battle_ui, 
            update_log_ui, 
            update_latest_log_ui,
            update_rule_display,
            update_token_inventory_display,
            update_instruction_display,
            update_right_panel_visibility,
            update_battle_info_display
        ))
        .run();
}

fn setup_battle(mut commands: Commands) {
    let player = GameCharacter::new("勇者".to_string(), 100, 80, 25);
    let enemy = GameCharacter::new("スライム".to_string(), 200, 30, 15);
    
    // Load player rules from JSON file
    let rule_set = load_rules_from_file("rules/player_rules.json")
        .expect("Failed to load player rules from JSON file");
    let player_rules = convert_to_node_rules(&rule_set)
        .expect("Failed to convert player rules");
    println!("Loaded player rules from JSON");
    
    // Load enemy rules from JSON file
    let rule_set = load_rules_from_file("rules/enemy_rules.json")
        .expect("Failed to load enemy rules from JSON file");
    let enemy_rules = convert_to_node_rules(&rule_set)
        .expect("Failed to convert enemy rules");
    println!("Loaded enemy rules from JSON");
    
    let rng = StdRng::from_entropy();
    let battle = Battle::new(player, enemy, player_rules, enemy_rules, rng);
    
    commands.insert_resource(GameBattle(battle));
}


fn handle_restart(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut game_battle: ResMut<GameBattle>,
) {
    if game_battle.0.battle_over && 
       (keyboard_input.just_pressed(KeyCode::ShiftLeft) || keyboard_input.just_pressed(KeyCode::ShiftRight)) {
        let player = GameCharacter::new("勇者".to_string(), 100, 50, 25);
        let enemy = GameCharacter::new("スライム".to_string(), 60, 30, 15);
        
        // Load rules from JSON
        let rule_set = load_rules_from_file("rules/player_rules.json")
            .expect("Failed to load player rules from JSON file");
        let player_rules = convert_to_node_rules(&rule_set)
            .expect("Failed to convert player rules");
        
        let rule_set = load_rules_from_file("rules/enemy_rules.json")
            .expect("Failed to load enemy rules from JSON file");
        let enemy_rules = convert_to_node_rules(&rule_set)
            .expect("Failed to convert enemy rules");
        
        let rng = StdRng::from_entropy();
        game_battle.0 = Battle::new(player, enemy, player_rules, enemy_rules, rng);
    }
}

// UIで作成したルールを戦闘システムに適用する
fn apply_rules_to_battle(
    game_state: Res<GameState>,
    current_rules: Res<CurrentRules>,
    mut game_battle: ResMut<GameBattle>,
) {
    // ルール作成モードから戦闘モードに切り替わった瞬間に新しいバトルを開始
    if game_state.is_changed() && game_state.mode == GameMode::Battle {
        let player = GameCharacter::new("勇者".to_string(), 100, 50, 25);
        let enemy = GameCharacter::new("スライム".to_string(), 60, 30, 15);
        
        // UIで作成したルールを変換
        let player_rules = current_rules.convert_to_rule_nodes();
        
        // 敵のルールをJSONから読み込み
        let rule_set = load_rules_from_file("rules/enemy_rules.json")
            .expect("Failed to load enemy rules from JSON file");
        let enemy_rules = convert_to_node_rules(&rule_set)
            .expect("Failed to convert enemy rules");
        
        let rng = StdRng::from_entropy();
        game_battle.0 = Battle::new(player, enemy, player_rules, enemy_rules, rng);
        
        println!("新しいバトルを開始しました。プレイヤールール数: {}", current_rules.convert_to_rule_nodes().len());
    }
}