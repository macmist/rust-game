use bevy::prelude::*;
use character::CharacterPlugin;
mod character;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, CharacterPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
