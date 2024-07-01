use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

#[derive(Component)]
struct Stuff {
    name: String,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, movement)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(50.0, 100.0))),
            material: materials.add(Color::BLUE),
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        },
        Stuff {
            name: "Coucou".to_string(),
        },
    ));
}

fn movement(
    mut sprite_position: Query<(&mut Transform, &mut Stuff)>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
) {
    let (camera, camera_transform) = camera_query.single();

    let Some(cursor_position) = windows.single().cursor_position() else {
        return;
    };

    // Calculate a world position based on the cursor's position.
    let Some(point) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
        return;
    };

    println!("there are {} sprintes", sprite_position.iter().count());
    for ((mut transform, _)) in &mut sprite_position {
        *transform = Transform::from_xyz(point.x, point.y, 0.);
    }
}
