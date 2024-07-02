use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

const SPEED: f32 = 150.;

#[derive(Component)]
enum Movement {
    NONE,
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn)
            .add_systems(Update, (input_system, movement).chain());
    }
}

fn spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(50.0, 100.0))),
            material: materials.add(Color::BLUE),
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        },
        Movement::NONE,
    ));
}

fn input_system(
    keys: Res<ButtonInput<KeyCode>>,
    mut sprite_position: Query<(&mut Transform, &mut Movement)>,
) {
    for (_, mut movement) in &mut sprite_position {
        *movement = Movement::NONE;
        if keys.any_pressed([KeyCode::ArrowUp, KeyCode::KeyW]) {
            *movement = Movement::UP;
        }
        if keys.any_pressed([KeyCode::ArrowDown, KeyCode::KeyS]) {
            *movement = Movement::DOWN;
        }
        if keys.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) {
            *movement = Movement::RIGHT;
        }
        if keys.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]) {
            *movement = Movement::LEFT;
        }
    }
}

fn movement(mut sprite_position: Query<(&mut Transform, &mut Movement)>, time: Res<Time>) {
    for (mut transform, movement) in &mut sprite_position {
        match *movement {
            Movement::DOWN => transform.translation.y -= SPEED * time.delta_seconds(),
            Movement::UP => transform.translation.y += SPEED * time.delta_seconds(),

            Movement::LEFT => transform.translation.x -= SPEED * time.delta_seconds(),

            Movement::RIGHT => transform.translation.x += SPEED * time.delta_seconds(),

            _ => (),
        }
    }
}
