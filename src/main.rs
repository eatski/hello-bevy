use bevy::prelude::*;

#[derive(Component)]
struct HelloText;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, handle_shift_key)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    
    commands.spawn((
        Text::new("Hello Bevy"),
        TextFont {
            font_size: 60.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Percent(45.0),
            left: Val::Percent(35.0),
            ..default()
        },
        HelloText,
    ));
}

fn handle_shift_key(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut text_query: Query<&mut Text, With<HelloText>>,
) {
    if keyboard_input.pressed(KeyCode::ShiftLeft) || keyboard_input.pressed(KeyCode::ShiftRight) {
        for mut text in text_query.iter_mut() {
            text.0 = "Thank you".to_string();
        }
    } else {
        for mut text in text_query.iter_mut() {
            text.0 = "Hello Bevy".to_string();
        }
    }
}
