// Bevy system integration - all game systems setup
use bevy::prelude::*;

use crate::{GameTeamBattle, BevyGameState, BevyCurrentRules};
use crate::ui::{BattleUI, LatestLogUI};
use crate::default_teams::{create_default_player_team, create_default_enemy_team, DEFAULT_ENEMY_RULES_PATH};
use crate::battle_display::{format_battle_display, format_latest_log};
use ui_core::BattleOrchestrator;
use json_rule::load_rules_from_file;

// チーム戦闘のセットアップ
pub fn setup_team_battle(mut commands: Commands, current_rules: Res<BevyCurrentRules>) {
    let player_team = create_default_player_team();
    let enemy_team = create_default_enemy_team();
    
    // Load enemy rules from JSON file
    let enemy_rule_set = load_rules_from_file(DEFAULT_ENEMY_RULES_PATH)
        .expect("Failed to load enemy rules from JSON file");
    
    let team_battle = BattleOrchestrator::create_battle(
        &current_rules.0,
        player_team,
        enemy_team,
        &enemy_rule_set,
    );
    println!("Loaded team battle rules: UI rules for players, JSON for enemies");
    commands.insert_resource(GameTeamBattle(team_battle));
}

// チーム戦闘リスタート処理
pub fn handle_team_restart(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut game_team_battle: ResMut<GameTeamBattle>,
    current_rules: Res<BevyCurrentRules>,
) {
    let shift_pressed = keyboard_input.just_pressed(KeyCode::ShiftLeft) ||
                       keyboard_input.just_pressed(KeyCode::ShiftRight);
    
    if BattleOrchestrator::should_restart_battle(game_team_battle.0.battle_over, shift_pressed) {
        let player_team = create_default_player_team();
        let enemy_team = create_default_enemy_team();
        
        // Load enemy rules from JSON file
        let enemy_rule_set = load_rules_from_file(DEFAULT_ENEMY_RULES_PATH)
            .expect("Failed to load enemy rules from JSON file");
        
        game_team_battle.0 = BattleOrchestrator::create_battle(
            &current_rules.0,
            player_team,
            enemy_team,
            &enemy_rule_set,
        );
        println!("チーム戦闘をリスタートしました");
    }
}

// UIで作成したルールをチーム戦闘システムに適用する
pub fn apply_rules_to_battle(
    game_state: Res<BevyGameState>,
    current_rules: Res<BevyCurrentRules>,
    mut game_team_battle: ResMut<GameTeamBattle>,
) {
    if BattleOrchestrator::should_start_new_battle(&game_state.0, game_state.is_changed()) {
        let player_team = create_default_player_team();
        let enemy_team = create_default_enemy_team();
        
        // Load enemy rules from JSON file
        let enemy_rule_set = load_rules_from_file(DEFAULT_ENEMY_RULES_PATH)
            .expect("Failed to load enemy rules from JSON file");
        
        game_team_battle.0 = BattleOrchestrator::create_battle(
            &current_rules.0,
            player_team,
            enemy_team,
            &enemy_rule_set,
        );
        println!("新しいチーム戦闘を開始しました。");
    }
}

// チーム戦闘の入力処理
pub fn handle_team_battle_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    game_state: Res<BevyGameState>,
    mut game_team_battle: ResMut<GameTeamBattle>,
) {
    let space_pressed = keyboard_input.just_pressed(KeyCode::Space);
    
    if BattleOrchestrator::should_execute_turn(
        &game_state.0,
        game_team_battle.0.battle_over,
        space_pressed,
    ) {
        println!("スペースキーが押されました - ターン実行");
        let (executed, current_turn) = BattleOrchestrator::execute_turn(&mut game_team_battle.0);
        if executed {
            println!("ターン実行完了 - 現在ターン: {}", current_turn);
        }
    }
}

// チーム戦闘専用のUI更新
pub fn update_team_battle_ui(
    game_state: Res<BevyGameState>,
    game_team_battle: Res<GameTeamBattle>,
    mut ui_query: Query<&mut Text, With<BattleUI>>,
) {
    for mut text in ui_query.iter_mut() {
        if let Some(ui_text) = BattleOrchestrator::get_battle_ui_text(&game_state.0) {
            text.0 = match ui_text {
                "rule_creation_mode" => "ルール作成中...\nスペースキーで戦闘開始".to_string(),
                _ => String::new(),
            };
        } else {
            text.0 = format_battle_display(&game_team_battle.0);
        }
    }
}

// チーム戦闘専用の最新ログUI更新
pub fn update_team_latest_log_ui(
    game_state: Res<BevyGameState>,
    game_team_battle: Res<GameTeamBattle>,
    mut latest_log_query: Query<&mut Text, (With<LatestLogUI>, Without<BattleUI>)>
) {
    for mut text in latest_log_query.iter_mut() {
        if let Some(log_text) = BattleOrchestrator::get_log_ui_text(&game_state.0) {
            text.0 = match log_text {
                "rule_creation_log" => "ルール作成モード：トークンを組み合わせて行動ルールを作成してください".to_string(),
                _ => String::new(),
            };
        } else {
            text.0 = format_latest_log(&game_team_battle.0);
        }
    }
}