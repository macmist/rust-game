use bevy::{prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}};



fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, movement)
        .run();
}

#[derive(Component)]
enum Direction {
    Up,
    Down
}

fn setup(mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((MaterialMesh2dBundle {
        mesh: Mesh2dHandle(meshes.add(Rectangle::new(50.0, 100.0))),
        material: materials.add(Color::BLUE),
        transform: Transform::from_xyz(0., 0., 0.,),
        ..default()
    }, Direction::Up));
}

fn movement(time: Res<Time>, mut sprite_position: Query<(&mut Direction, &mut Transform)>) {
    for (mut logo, mut transform) in &mut sprite_position {
        match *logo {
            Direction::Up => transform.translation.y += 150. * time.delta_seconds(),
            Direction::Down => transform.translation.y -= 150. * time.delta_seconds()
        }
        if transform.translation.y > 200. {
            *logo = Direction::Down;
        } else if transform.translation.y < -200. {
            *logo = Direction::Up;
        }
    }
}