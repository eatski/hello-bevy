use bevy::prelude::*;
use rand::Rng;

#[derive(Component)]
struct Character {
    name: String,
    hp: i32,
    max_hp: i32,
    attack: i32,
    is_player: bool,
}

#[derive(Resource)]
struct BattleState {
    current_turn: usize,
    battle_over: bool,
    winner: Option<String>,
    battle_log: Vec<String>,
}

#[derive(Resource)]
struct GameFont {
    font: Handle<Font>,
}

#[derive(Component)]
struct BattleUI;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(BattleState {
            current_turn: 0,
            battle_over: false,
            winner: None,
            battle_log: Vec::new(),
        })
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
    
    // プレイヤーキャラクター
    commands.spawn(Character {
        name: "勇者".to_string(),
        hp: 100,
        max_hp: 100,
        attack: 25,
        is_player: true,
    });
    
    // 敵キャラクター
    commands.spawn(Character {
        name: "スライム".to_string(),
        hp: 60,
        max_hp: 60,
        attack: 15,
        is_player: false,
    });
    
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
    mut battle_state: ResMut<BattleState>,
    mut characters: Query<&mut Character>,
) {
    if battle_state.battle_over {
        return;
    }
    
    if keyboard_input.just_pressed(KeyCode::Space) {
        let mut player_char = None;
        let mut enemy_char = None;
        
        for character in characters.iter_mut() {
            if character.is_player {
                player_char = Some(character);
            } else {
                enemy_char = Some(character);
            }
        }
        
        if let (Some(mut player), Some(mut enemy)) = (player_char, enemy_char) {
            if battle_state.current_turn % 2 == 0 {
                // プレイヤーのターン
                enemy.hp -= player.attack;
                battle_state.battle_log.push(format!("{}が{}に{}のダメージ！", player.name, enemy.name, player.attack));
                
                if enemy.hp <= 0 {
                    enemy.hp = 0;
                    battle_state.battle_over = true;
                    battle_state.winner = Some(player.name.clone());
                    battle_state.battle_log.push(format!("{}の勝利！", player.name));
                }
            } else {
                // 敵のターン
                let mut rng = rand::thread_rng();
                let damage = rng.gen_range(10..=enemy.attack);
                player.hp -= damage;
                battle_state.battle_log.push(format!("{}が{}に{}のダメージ！", enemy.name, player.name, damage));
                
                if player.hp <= 0 {
                    player.hp = 0;
                    battle_state.battle_over = true;
                    battle_state.winner = Some(enemy.name.clone());
                    battle_state.battle_log.push(format!("{}の勝利！", enemy.name));
                }
            }
            
            battle_state.current_turn += 1;
        }
    }
}

fn update_battle_ui(
    battle_state: Res<BattleState>,
    characters: Query<&Character>,
    mut ui_query: Query<&mut Text, With<BattleUI>>,
) {
    for mut text in ui_query.iter_mut() {
        let mut display_text = String::new();
        
        for character in characters.iter() {
            display_text.push_str(&format!(
                "{}: HP {}/{}\n",
                character.name, character.hp, character.max_hp
            ));
        }
        
        display_text.push_str("\n");
        
        if battle_state.battle_over {
            if let Some(winner) = &battle_state.winner {
                display_text.push_str(&format!("バトル終了！{}の勝利！\n", winner));
            }
        } else {
            let current_player = if battle_state.current_turn % 2 == 0 { "プレイヤー" } else { "敵" };
            display_text.push_str(&format!("{}のターン\n", current_player));
            if battle_state.current_turn % 2 == 0 {
                display_text.push_str("スペースキーで攻撃！\n");
            }
        }
        
        display_text.push_str("\n戦闘ログ:\n");
        for log in battle_state.battle_log.iter().rev().take(5) {
            display_text.push_str(&format!("{}\n", log));
        }
        
        text.0 = display_text;
    }
}