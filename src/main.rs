use bevy::prelude::*;
use rand::rngs::StdRng;
use rand::SeedableRng;

mod action_system;
mod battle_system;
mod ui;

use ui::*;
use battle_system::{Battle, Character as GameCharacter};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (load_font, setup_ui, setup_battle).chain())
        .add_systems(Update, (handle_battle_input, handle_restart, update_battle_ui, update_log_ui, update_latest_log_ui))
        .run();
}

fn setup_battle(mut commands: Commands) {
    let player = GameCharacter::new("勇者".to_string(), 100, 50, 25);
    let enemy = GameCharacter::new("スライム".to_string(), 60, 30, 15);
    
    // HP-based healing rules with randomness
    let rules: Vec<Vec<Box<dyn action_system::Token>>> = vec![
        vec![
            Box::new(action_system::Check::new(
                action_system::GreaterThanToken::new(
                    action_system::Number::new(50),
                    action_system::CharacterHP::new(action_system::SelfCharacter),
                )
            )),
            Box::new(action_system::Check::new(action_system::TrueOrFalseRandom)),
            Box::new(action_system::Heal),
        ],
        vec![
            Box::new(action_system::Check::new(action_system::TrueOrFalseRandom)),
            Box::new(action_system::Strike),
        ],
    ];
    
    let rng = StdRng::from_entropy();
    let battle = Battle::new(player, enemy, rules, rng);
    
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
        
        // HP-based healing rules with randomness
        let rules: Vec<Vec<Box<dyn action_system::Token>>> = vec![
            vec![
                Box::new(action_system::Check::new(
                    action_system::GreaterThanToken::new(
                        action_system::Number::new(50),
                        action_system::CharacterHP::new(action_system::SelfCharacter),
                    )
                )),
                Box::new(action_system::Check::new(action_system::TrueOrFalseRandom)),
                Box::new(action_system::Heal),
            ],
            vec![
                Box::new(action_system::Check::new(action_system::TrueOrFalseRandom)),
                Box::new(action_system::Strike),
            ],
        ];
        
        let rng = StdRng::from_entropy();
        game_battle.0 = Battle::new(player, enemy, rules, rng);
    }
}