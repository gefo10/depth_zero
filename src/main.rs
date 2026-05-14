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
            PhysicsDebugPlugin::default(),
            CharacterControllerPlugin,
        ))
        .add_systems(Startup, (setup_camera, setup_world, setup_player))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

fn platform(commands: &mut Commands, x: f32, y: f32, w: f32, h: f32) {
    commands.spawn((
        RigidBody::Static,
        Collider::rectangle(w, h),
        Transform::from_xyz(x, y, 0.0),
        Sprite {
            color: Color::srgb(0.25, 0.25, 0.28),
            custom_size: Some(Vec2::new(w, h)),
            ..default()
        },
    ));
}

fn setup_world(mut commands: Commands) {
    // Ground
    platform(&mut commands, 0.0, -250.0, 1200.0, 20.0);

    // Left tall wall — good for wall sliding
    platform(&mut commands, -500.0, 0.0, 20.0, 500.0);

    // Right tall wall
    platform(&mut commands, 500.0, 0.0, 20.0, 500.0);

    // Low platform left — first jump
    platform(&mut commands, -280.0, -130.0, 160.0, 16.0);

    // Mid platform — ledge grab target (exposed right edge)
    platform(&mut commands, -60.0, -30.0, 200.0, 16.0);

    // High platform right — requires wall jump to reach
    platform(&mut commands, 280.0, 100.0, 160.0, 16.0);

    // Small step between ground and low platform
    platform(&mut commands, -380.0, -200.0, 60.0, 16.0);

    // Tall pillar in the middle — creates a wall jump corridor
    platform(&mut commands, 100.0, -80.0, 20.0, 200.0);
}

fn setup_player(mut commands: Commands) {
    commands.spawn((
        Transform::from_xyz(-400.0, -180.0, 0.0),
        Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(20.0, 60.0)),
            ..default()
        },
        CharacterControllerBundle::new(Collider::capsule(10.0, 60.0)).with_movement(
            350.0,
            0.01,
            300.0,
            (30.0 as Scalar).to_radians(),
        ),
        Friction::ZERO.with_combine_rule(CoefficientCombine::Min),
        Restitution::ZERO.with_combine_rule(CoefficientCombine::Min),
        GravityScale(30.0),
    ));
}
