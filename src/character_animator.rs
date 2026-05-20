use bevy::prelude::*;

use crate::character_controller::CharacterController;
use avian2d::prelude::*;

/// Owns visual representation and procedural animation of the character.
/// Reads state from `character_controller` (read-only); never writes back.
pub struct CharacterAnimatorPlugin;

impl Plugin for CharacterAnimatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, animate_torso);
    }
}

// === Body part markers ===
// Attached to the child sprite entities so animation systems can query each
// part individually. Zero size — just identity tags.
#[derive(Component)]
pub struct Head;
#[derive(Component)]
pub struct Torso;
#[derive(Component)]
pub struct LeftArm;
#[derive(Component)]
pub struct RightArm;
#[derive(Component)]
pub struct LeftLeg;
#[derive(Component)]
pub struct RightLeg;

/// The neutral pose a body part eases back to. Every procedural animation
/// system reads this as the "where do I belong when nothing is happening"
/// baseline and computes deviations relative to it.
#[derive(Component, Clone, Copy)]
pub struct RestPose {
    pub position: Vec2,
    pub scale: Vec2,
    pub rotation: f32,
}

fn animate_torso(
    time: Res<Time>,
    player: Query<(&LinearVelocity, &Children), With<CharacterController>>,
    mut torsos: Query<&mut Transform, With<Torso>>,
) {
    for (velocity, children) in &player {
        let speed_y = velocity.y.abs();
        let target_scale_y = (1.0 + speed_y * 0.002).min(1.4);
        let target_scale_x = 1.0 / target_scale_y;

        let alpha = 1.0 - (-10.0 * time.delta_secs()).exp();

        for child in children.iter() {
            let Ok(mut t) = torsos.get_mut(child) else {
                continue;
            };
            t.scale.y = t.scale.y.lerp(target_scale_y, alpha);
            t.scale.x = t.scale.x.lerp(target_scale_x, alpha);
        }
    }
}
