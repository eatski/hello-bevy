use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, game_loop)
        .run();
}

fn setup() {
    println!("Setting up the battle!");
    // Setup logic will go here
}

fn game_loop() {
    // Game logic will go here
}
