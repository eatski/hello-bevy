use bevy::prelude::*;

mod action_system;
mod battle_system;
use battle_system::{Battle, Character as GameCharacter};

#[derive(Resource)]
struct GameFont {
    font: Handle<Font>,
}

#[derive(Resource)]
struct GameBattle(Battle);

#[derive(Component)]
struct BattleUI;

#[derive(Component)]
struct LogUI;

#[derive(Component)]
struct LatestLogUI;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (load_font, setup_battle).chain())
        .add_systems(Update, (handle_battle_input, update_battle_ui, update_log_ui, update_latest_log_ui))
        .run();
}

fn load_font(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/NotoSansCJK-Regular.ttc");
    commands.insert_resource(GameFont { font });
}

fn setup_battle(mut commands: Commands, game_font: Res<GameFont>) {
    commands.spawn(Camera2d);
    
    let player = GameCharacter::new("勇者".to_string(), 100, 50, 25, true);
    let enemy = GameCharacter::new("スライム".to_string(), 60, 30, 15, false);
    let battle = Battle::new(player, enemy);
    
    commands.insert_resource(GameBattle(battle));
    
    // メインUI表示（左側）
    commands.spawn((
        Text::new("RPGバトル開始！\nスペースキーで攻撃"),
        TextFont {
            font: game_font.font.clone(),
            font_size: 24.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(20.0),
            left: Val::Px(20.0),
            ..default()
        },
        BattleUI,
    ));
    
    // ログ表示（右側）
    commands.spawn((
        Text::new("戦闘ログ:\n"),
        TextFont {
            font: game_font.font.clone(),
            font_size: 18.0,
            ..default()
        },
        TextColor(Color::srgb(0.8, 0.8, 0.8)),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(20.0),
            right: Val::Px(20.0),
            width: Val::Px(400.0),
            ..default()
        },
        LogUI,
    ));
    
    // 最新ログ表示（中央上部）
    commands.spawn((
        Text::new(""),
        TextFont {
            font: game_font.font.clone(),
            font_size: 20.0,
            ..default()
        },
        TextColor(Color::srgb(1.0, 1.0, 0.0)),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(300.0),
            left: Val::Px(300.0),
            ..default()
        },
        LatestLogUI,
    ));
}

fn handle_battle_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut game_battle: ResMut<GameBattle>,
) {
    if game_battle.0.battle_over {
        if keyboard_input.just_pressed(KeyCode::ShiftLeft) || keyboard_input.just_pressed(KeyCode::ShiftRight) {
            let player = GameCharacter::new("勇者".to_string(), 100, 50, 25, true);
            let enemy = GameCharacter::new("スライム".to_string(), 60, 30, 15, false);
            game_battle.0 = Battle::new(player, enemy);
        }
        return;
    }
    
    if keyboard_input.just_pressed(KeyCode::Space) {
        if game_battle.0.is_player_turn() {
            game_battle.0.execute_player_action();
        } else {
            game_battle.0.execute_enemy_action();
        }
    }
}

fn create_hp_bar(current_hp: i32, max_hp: i32) -> String {
    let bar_length = 20;
    let filled_length = if max_hp > 0 {
        (current_hp * bar_length / max_hp).max(0)
    } else {
        0
    };
    
    let mut bar = String::new();
    bar.push('[');
    for i in 0..bar_length {
        if i < filled_length {
            bar.push('█');
        } else {
            bar.push('░');
        }
    }
    bar.push(']');
    bar
}

fn create_mp_bar(current_mp: i32, max_mp: i32) -> String {
    let bar_length = 20;
    let filled_length = if max_mp > 0 {
        (current_mp * bar_length / max_mp).max(0)
    } else {
        0
    };
    
    let mut bar = String::new();
    bar.push('[');
    for i in 0..bar_length {
        if i < filled_length {
            bar.push('◆');
        } else {
            bar.push('◇');
        }
    }
    bar.push(']');
    bar
}

fn update_battle_ui(
    game_battle: Res<GameBattle>,
    mut ui_query: Query<&mut Text, With<BattleUI>>,
) {
    for mut text in ui_query.iter_mut() {
        let battle = &game_battle.0;
        let mut display_text = String::new();
        
        let player_hp_bar = create_hp_bar(battle.player.hp, battle.player.max_hp);
        let player_mp_bar = create_mp_bar(battle.player.mp, battle.player.max_mp);
        display_text.push_str(&format!(
            "{}: HP {}/{} {} MP {}/{} {}\n",
            battle.player.name, battle.player.hp, battle.player.max_hp, player_hp_bar,
            battle.player.mp, battle.player.max_mp, player_mp_bar
        ));
        
        let enemy_hp_bar = create_hp_bar(battle.enemy.hp, battle.enemy.max_hp);
        let enemy_mp_bar = create_mp_bar(battle.enemy.mp, battle.enemy.max_mp);
        display_text.push_str(&format!(
            "{}: HP {}/{} {} MP {}/{} {}\n",
            battle.enemy.name, battle.enemy.hp, battle.enemy.max_hp, enemy_hp_bar,
            battle.enemy.mp, battle.enemy.max_mp, enemy_mp_bar
        ));
        
        display_text.push_str("\n");
        
        if battle.battle_over {
            if let Some(winner) = &battle.winner {
                display_text.push_str(&format!("バトル終了！{}の勝利！\n", winner));
                display_text.push_str("Shiftキーで再戦！\n");
            }
        } else {
            display_text.push_str(&format!("{}のターン\n", battle.get_current_player_name()));
            if battle.is_player_turn() {
                display_text.push_str("スペースキーで攻撃！\n");
            }
        }
        
        text.0 = display_text;
    }
}

fn update_log_ui(
    game_battle: Res<GameBattle>,
    mut log_query: Query<&mut Text, (With<LogUI>, Without<BattleUI>, Without<LatestLogUI>)>,
) {
    for mut text in log_query.iter_mut() {
        let battle = &game_battle.0;
        let mut log_text = String::from("戦闘ログ:\n");
        
        for log in battle.get_recent_logs(8) {
            log_text.push_str(&format!("{}\n", log));
        }
        
        text.0 = log_text;
    }
}

fn update_latest_log_ui(
    game_battle: Res<GameBattle>,
    mut latest_log_query: Query<&mut Text, (With<LatestLogUI>, Without<BattleUI>, Without<LogUI>)>,
) {
    for mut text in latest_log_query.iter_mut() {
        let battle = &game_battle.0;
        
        if let Some(latest_log) = battle.battle_log.last() {
            text.0 = format!(">>> {}", latest_log);
        } else {
            text.0 = String::new();
        }
    }
}