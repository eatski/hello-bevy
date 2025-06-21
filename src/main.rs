use bevy::prelude::*;

mod game_logic;
use game_logic::{Battle, Character as GameCharacter};

#[derive(Resource)]
struct GameFont {
    font: Handle<Font>,
}

#[derive(Resource)]
struct GameBattle {
    battle: Battle,
}

#[derive(Component)]
struct BattleUI;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (load_font, setup_battle).chain())
        .add_systems(Update, (handle_battle_input, update_battle_ui))
        .run();
}

fn load_font(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/NotoSansCJK-Regular.ttc");
    commands.insert_resource(GameFont { font });
}

fn setup_battle(mut commands: Commands, game_font: Res<GameFont>) {
    commands.spawn(Camera2d);
    
    let player = GameCharacter::new("勇者".to_string(), 100, 25, true);
    let enemy = GameCharacter::new("スライム".to_string(), 60, 15, false);
    let battle = Battle::new(player, enemy);
    
    commands.insert_resource(GameBattle { battle });
    
    // UI表示
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
}

fn handle_battle_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut game_battle: ResMut<GameBattle>,
) {
    if game_battle.battle.battle_over {
        return;
    }
    
    if keyboard_input.just_pressed(KeyCode::Space) {
        if game_battle.battle.is_player_turn() {
            game_battle.battle.execute_player_action();
        } else {
            game_battle.battle.execute_enemy_action();
        }
    }
}

fn update_battle_ui(
    game_battle: Res<GameBattle>,
    mut ui_query: Query<&mut Text, With<BattleUI>>,
) {
    for mut text in ui_query.iter_mut() {
        let battle = &game_battle.battle;
        let mut display_text = String::new();
        
        display_text.push_str(&format!(
            "{}: HP {}/{}\n",
            battle.player.name, battle.player.hp, battle.player.max_hp
        ));
        
        display_text.push_str(&format!(
            "{}: HP {}/{}\n",
            battle.enemy.name, battle.enemy.hp, battle.enemy.max_hp
        ));
        
        display_text.push_str("\n");
        
        if battle.battle_over {
            if let Some(winner) = &battle.winner {
                display_text.push_str(&format!("バトル終了！{}の勝利！\n", winner));
            }
        } else {
            display_text.push_str(&format!("{}のターン\n", battle.get_current_player_name()));
            if battle.is_player_turn() {
                display_text.push_str("スペースキーで攻撃！\n");
            }
        }
        
        display_text.push_str("\n戦闘ログ:\n");
        for log in battle.get_recent_logs(5) {
            display_text.push_str(&format!("{}\n", log));
        }
        
        text.0 = display_text;
    }
}