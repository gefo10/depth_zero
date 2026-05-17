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

/// Movement-test "gym" — every section stresses one or two verbs so the
/// controller can be tuned in isolation. Built left-to-right; each section
/// is labelled with the verb it exists to test.
fn setup_world(mut commands: Commands) {
    let c = &mut commands;

    // === SPAWN + RUN-UP ===
    // Long flat ground. Tests acceleration, max speed, damping.
    platform(c, -450.0, -250.0, 500.0, 20.0);

    // === JUMP GAPS ===
    // Three small islands with progressively wider gaps. Reveals the speed
    // at which a stand-still jump becomes uncrossable vs. a running jump.
    platform(c, -130.0, -250.0, 60.0, 20.0); // gap = 40
    platform(c,   10.0, -250.0, 80.0, 20.0); // gap = 70
    platform(c,  170.0, -250.0, 80.0, 20.0); // gap = 80

    // === WALL JUMP TEST ===
    // A single tall wall. Slide down it, push off it. Add a facing wall
    // later if you want to test alternating wall jumps for climbing.
    platform(c, 250.0, -50.0, 20.0, 400.0); // top at y=150

    // === LEDGE-GRAB TOWER ===
    // Staggered platforms with exposed edges. Each lip should be reachable
    // from the wall below + a jump, then grabbed and mantled.
    platform(c, 340.0,  60.0, 80.0, 16.0); // low ledge (top at y=68)
    platform(c, 450.0, 160.0, 80.0, 16.0); // mid ledge
    platform(c, 560.0, 260.0, 80.0, 16.0); // high ledge

    // Right boundary so the player can't leap off into infinity.
    platform(c, 720.0, 0.0, 20.0, 600.0);
}

/// Spawns the player as a parent physics body with child sprite "parts"
/// arranged as a humanoid silhouette. The parts are static for now; future
/// procedural animation will animate their Transforms based on velocity,
/// contact state, and `Hanging`.
fn setup_player(mut commands: Commands) {
    // Bright cream against dark-gray platforms — high contrast silhouette.
    let body_color = Color::srgb(0.95, 0.92, 0.85);

    commands
        .spawn((
            Transform::from_xyz(-500.0, -180.0, 0.0),
            Visibility::default(),
            CharacterControllerBundle::new(Collider::capsule(10.0, 60.0)).with_movement(
                350.0,
                0.01,
                300.0,
                (30.0 as Scalar).to_radians(),
            ),
            Friction::ZERO.with_combine_rule(CoefficientCombine::Min),
            Restitution::ZERO.with_combine_rule(CoefficientCombine::Min),
            GravityScale(30.0),
        ))
        .with_children(|parent| {
            // Head
            parent.spawn((
                Transform::from_xyz(0.0, 30.0, 1.0),
                Sprite {
                    color: body_color,
                    custom_size: Some(Vec2::new(14.0, 14.0)),
                    ..default()
                },
            ));
            // Torso
            parent.spawn((
                Transform::from_xyz(0.0, 7.0, 1.0),
                Sprite {
                    color: body_color,
                    custom_size: Some(Vec2::new(14.0, 28.0)),
                    ..default()
                },
            ));
            // Left arm
            parent.spawn((
                Transform::from_xyz(-10.0, 7.0, 1.0),
                Sprite {
                    color: body_color,
                    custom_size: Some(Vec2::new(5.0, 28.0)),
                    ..default()
                },
            ));
            // Right arm
            parent.spawn((
                Transform::from_xyz(10.0, 7.0, 1.0),
                Sprite {
                    color: body_color,
                    custom_size: Some(Vec2::new(5.0, 28.0)),
                    ..default()
                },
            ));
            // Left leg
            parent.spawn((
                Transform::from_xyz(-4.0, -25.0, 1.0),
                Sprite {
                    color: body_color,
                    custom_size: Some(Vec2::new(6.0, 30.0)),
                    ..default()
                },
            ));
            // Right leg
            parent.spawn((
                Transform::from_xyz(4.0, -25.0, 1.0),
                Sprite {
                    color: body_color,
                    custom_size: Some(Vec2::new(6.0, 30.0)),
                    ..default()
                },
            ));
        });
}
