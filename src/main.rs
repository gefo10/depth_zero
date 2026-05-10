use avian2d::prelude::*;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PhysicsPlugins::default()))
        .add_systems(Startup, (setup_camera, setup_test_floor, setup_test))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

fn setup_test_floor(mut commands: Commands) {
    commands.spawn((
        RigidBody::Static,
        Collider::rectangle(800.0, 20.0),
        Transform::from_xyz(0.0, -200.0, 0.0),
        Sprite {
            color: Color::srgb(0.3, 0.3, 0.3),
            custom_size: Some(Vec2::new(800.0, 20.0)),
            ..default()
        },
    ));
}
fn setup_test(mut commands: Commands) {
    commands.spawn((
        RigidBody::Dynamic,
        Collider::rectangle(20.0, 60.0),
        Transform::from_xyz(0.0, 3.0, 0.0),
        Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(20.0, 60.0)),
            ..default()
        },
    ));
}
