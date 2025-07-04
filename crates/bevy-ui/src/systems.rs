// Bevy system integration - all game systems setup
use bevy::prelude::*;
use rand::rngs::StdRng;
use rand::SeedableRng;

use crate::{GameTeamBattle, GameMode, BevyGameState, BevyCurrentRules};
use crate::ui::{BattleUI, LatestLogUI};
use battle::{TeamBattle, Character as GameCharacter, Team};
use json_rule::load_rules_from_file;
use token_input::convert_ruleset_to_nodes;

// チーム戦闘のセットアップ
pub fn setup_team_battle(mut commands: Commands, current_rules: Res<BevyCurrentRules>) {
    // プレイヤーチーム
    let player_team = Team::new("勇者パーティー".to_string(), vec![
        GameCharacter::new(1, "勇者".to_string(), 100, 80, 25),
        GameCharacter::new(2, "戦士".to_string(), 120, 50, 30),
        GameCharacter::new(3, "魔法使い".to_string(), 70, 100, 15),
    ]);
    
    // 敵チーム
    let enemy_team = Team::new("モンスター軍団".to_string(), vec![
        GameCharacter::new(4, "オーク".to_string(), 150, 30, 20),
        GameCharacter::new(5, "ゴブリン".to_string(), 80, 40, 15),
        GameCharacter::new(6, "スライム".to_string(), 60, 60, 10),
    ]);
    
    // Use UI-configured rules for each player character (3 characters)
    let player_rules_per_character = vec![
        current_rules.0.convert_to_rule_nodes(),
        current_rules.0.convert_to_rule_nodes(),
        current_rules.0.convert_to_rule_nodes(),
    ];
    
    // Load enemy rules from JSON file for each character
    let enemy_rule_set = load_rules_from_file("rules/enemy_rules.json")
        .expect("Failed to load enemy rules from JSON file");
    
    // Create rules for each enemy character (3 characters)
    let enemy_rules_per_character = vec![
        convert_ruleset_to_nodes(&enemy_rule_set),
        convert_ruleset_to_nodes(&enemy_rule_set),
        convert_ruleset_to_nodes(&enemy_rule_set),
    ];
    
    println!("Loaded team battle rules: UI rules for players, JSON for enemies");
    
    let rng = StdRng::from_entropy();
    let team_battle = TeamBattle::new(
        player_team.clone(), 
        enemy_team.clone(), 
        player_rules_per_character, 
        enemy_rules_per_character, 
        rng
    );
    
    commands.insert_resource(GameTeamBattle(team_battle));
}

// チーム戦闘リスタート処理
pub fn handle_team_restart(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut game_team_battle: ResMut<GameTeamBattle>,
    current_rules: Res<BevyCurrentRules>,
) {
    if game_team_battle.0.battle_over && 
       (keyboard_input.just_pressed(KeyCode::ShiftLeft) || keyboard_input.just_pressed(KeyCode::ShiftRight)) {
        // プレイヤーチーム
        let player_team = Team::new("勇者パーティー".to_string(), vec![
            GameCharacter::new(1, "勇者".to_string(), 100, 80, 25),
            GameCharacter::new(2, "戦士".to_string(), 120, 50, 30),
            GameCharacter::new(3, "魔法使い".to_string(), 70, 100, 15),
        ]);
        
        // 敵チーム
        let enemy_team = Team::new("モンスター軍団".to_string(), vec![
            GameCharacter::new(4, "オーク".to_string(), 150, 30, 20),
            GameCharacter::new(5, "ゴブリン".to_string(), 80, 40, 15),
            GameCharacter::new(6, "スライム".to_string(), 60, 60, 10),
        ]);
        
        // Use UI-configured rules for players
        let player_rules_per_character = vec![
            current_rules.0.convert_to_rule_nodes(),
            current_rules.0.convert_to_rule_nodes(),
            current_rules.0.convert_to_rule_nodes(),
        ];
        
        let enemy_rule_set = load_rules_from_file("rules/enemy_rules.json")
            .expect("Failed to load enemy rules from JSON file");
        let enemy_rules_per_character = vec![
            convert_ruleset_to_nodes(&enemy_rule_set),
            convert_ruleset_to_nodes(&enemy_rule_set),
            convert_ruleset_to_nodes(&enemy_rule_set),
        ];
        
        let rng = StdRng::from_entropy();
        game_team_battle.0 = TeamBattle::new(
            player_team,
            enemy_team,
            player_rules_per_character,
            enemy_rules_per_character,
            rng
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
    // ルール作成モードから戦闘モードに切り替わった瞬間に新しいチーム戦闘を開始
    if game_state.is_changed() && game_state.0.mode == GameMode::Battle {
        // プレイヤーチーム
        let player_team = Team::new("勇者パーティー".to_string(), vec![
            GameCharacter::new(1, "勇者".to_string(), 100, 80, 25),
            GameCharacter::new(2, "戦士".to_string(), 120, 50, 30),
            GameCharacter::new(3, "魔法使い".to_string(), 70, 100, 15),
        ]);
        
        // 敵チーム
        let enemy_team = Team::new("モンスター軍団".to_string(), vec![
            GameCharacter::new(4, "オーク".to_string(), 150, 30, 20),
            GameCharacter::new(5, "ゴブリン".to_string(), 80, 40, 15),
            GameCharacter::new(6, "スライム".to_string(), 60, 60, 10),
        ]);
        
        // UIで作成したルールを各プレイヤーキャラクターに適用
        let player_rules_per_character = vec![
            current_rules.0.convert_to_rule_nodes(),
            current_rules.0.convert_to_rule_nodes(),
            current_rules.0.convert_to_rule_nodes(),
        ];
        
        // 敵のルールをJSONから読み込み
        let enemy_rule_set = load_rules_from_file("rules/enemy_rules.json")
            .expect("Failed to load enemy rules from JSON file");
        let enemy_rules_per_character = vec![
            convert_ruleset_to_nodes(&enemy_rule_set),
            convert_ruleset_to_nodes(&enemy_rule_set),
            convert_ruleset_to_nodes(&enemy_rule_set),
        ];
        
        let rng = StdRng::from_entropy();
        game_team_battle.0 = TeamBattle::new(
            player_team,
            enemy_team,
            player_rules_per_character,
            enemy_rules_per_character,
            rng
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
    if game_state.0.mode == GameMode::Battle && !game_team_battle.0.battle_over {
        if keyboard_input.just_pressed(KeyCode::Space) {
            println!("スペースキーが押されました - ターン実行");
            game_team_battle.0.execute_turn();
            println!("ターン実行完了 - 現在ターン: {}", game_team_battle.0.current_turn);
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
        match game_state.0.mode {
            GameMode::RuleCreation => {
                text.0 = "ルール作成中...\nスペースキーで戦闘開始".to_string();
            }
            GameMode::Battle => {
                let battle = &game_team_battle.0;
                let mut display_text = String::new();
                
                display_text.push_str(&format!("=== チーム戦闘 (ターン {}) ===\n", battle.current_turn + 1));
                
                // プレイヤーチーム情報
                display_text.push_str(&format!("\n【{}】\n", battle.player_team.name));
                for member in &battle.player_team.members {
                    let status = if member.is_alive() { "生存" } else { "戦闘不能" };
                    display_text.push_str(&format!("  {} - HP:{}/{} MP:{}/{} ({})\n", 
                        member.name, member.hp, member.max_hp, member.mp, member.max_mp, status));
                }
                
                // 敵チーム情報
                display_text.push_str(&format!("\n【{}】\n", battle.enemy_team.name));
                for member in &battle.enemy_team.members {
                    let status = if member.is_alive() { "生存" } else { "戦闘不能" };
                    display_text.push_str(&format!("  {} - HP:{}/{} MP:{}/{} ({})\n", 
                        member.name, member.hp, member.max_hp, member.mp, member.max_mp, status));
                }
                
                // 現在のターン情報
                if !battle.battle_over {
                    if let Some(current_character) = battle.get_current_acting_character() {
                        display_text.push_str(&format!("\n現在の行動キャラクター: {} ({})\n", 
                            current_character.name, battle.get_current_team_name()));
                        display_text.push_str("スペースキーでターン実行\n");
                    }
                } else {
                    if let Some(winner) = &battle.winner {
                        display_text.push_str(&format!("\n🎉 {} の勝利！\n", winner));
                        display_text.push_str("Shiftキーでリセット\n");
                    }
                }
                
                text.0 = display_text;
            }
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
        match game_state.0.mode {
            GameMode::RuleCreation => {
                text.0 = "ルール作成モード：トークンを組み合わせて行動ルールを作成してください".to_string();
            }
            GameMode::Battle => {
                let battle = &game_team_battle.0;
                
                if let Some(latest_log) = battle.battle_log.last() {
                    text.0 = format!(">>> {}", latest_log);
                } else {
                    text.0 = "チーム戦闘開始！スペースキーでターン実行".to_string();
                }
            }
        }
    }
}