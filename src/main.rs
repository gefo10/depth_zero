mod character_controller;

use avian2d::{math::*, prelude::*};
use bevy::prelude::*;
use character_controller::*;

use crate::character_controller::CharacterControllerPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default(),
            CharacterControllerPlugin,
        ))
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
        Transform::from_xyz(0.0, 3.0, 0.0),
        Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(20.0, 60.0)),
            ..default()
        },
        CharacterControllerBundle::new(Collider::capsule(0.4, 1.0)).with_movement(
            30.0,
            0.92,
            7.0,
            (30.0 as Scalar).to_radians(),
        ),
        Friction::ZERO.with_combine_rule(CoefficientCombine::Min),
        Restitution::ZERO.with_combine_rule(CoefficientCombine::Min),
        GravityScale(2.0),
    ));
}
