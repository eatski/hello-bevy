use bevy::prelude::*;
use battle_system::Battle;

#[derive(Resource)]
pub struct GameFont {
    pub font: Handle<Font>,
}

#[derive(Resource)]
pub struct GameBattle(pub Battle);

#[derive(Component)]
pub struct BattleUI;

#[derive(Component)]
pub struct LogUI;

#[derive(Component)]
pub struct LatestLogUI;

#[derive(Component)]
pub struct HPBar;

#[derive(Component)]
pub struct MPBar;



pub fn load_font(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/NotoSansCJK-Regular.ttc");
    commands.insert_resource(GameFont { font });
}

pub fn setup_ui(
    mut commands: Commands, 
    game_font: Res<GameFont>,
) {
    commands.spawn(Camera2d);
    
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

pub fn handle_battle_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut game_battle: ResMut<GameBattle>,
) {
    if game_battle.0.battle_over {
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






pub fn update_battle_ui(
    game_battle: Res<GameBattle>,
    mut ui_query: Query<&mut Text, With<BattleUI>>,
) {
    for mut text in ui_query.iter_mut() {
        let battle = &game_battle.0;
        let mut display_text = String::new();
        
        display_text.push_str(&format!("{}:\n", battle.player.name));
        display_text.push_str(&format!("HP {}/{}\n", battle.player.hp, battle.player.max_hp));
        display_text.push_str(&format!("MP {}/{}\n", battle.player.mp, battle.player.max_mp));
        
        display_text.push_str("\n");
        
        display_text.push_str(&format!("{}:\n", battle.enemy.name));
        display_text.push_str(&format!("HP {}/{}\n", battle.enemy.hp, battle.enemy.max_hp));
        display_text.push_str(&format!("MP {}/{}\n", battle.enemy.mp, battle.enemy.max_mp));
        
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

pub fn update_log_ui(
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

pub fn update_latest_log_ui(
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