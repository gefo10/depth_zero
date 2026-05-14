use avian2d::{math::*, prelude::*};
use bevy::{math::VectorSpace, prelude::*};

pub struct CharacterControllerPlugin;

impl Plugin for CharacterControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<MovementAction>().add_systems(
            Update,
            (
                setup_casters,
                reset_is_moving,
                keyboard_input,
                gamepad_input,
                update_grounded,
                update_wall_contact,
                movement,
                apply_movement_damping,
            )
                .chain(),
        );
    }
}

#[derive(Message)]
pub enum MovementAction {
    Move(Vector2),
    Jump,
}

#[derive(Component)]
pub struct IsMoving(bool);

#[derive(Component)]
pub struct IsGrounded(bool);

#[derive(Component)]
pub struct CharacterController;

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Grounded;

#[derive(Component)]
pub struct MovementAcceleration(Scalar);

#[derive(Component)]
pub struct MovementDampingFactor(Scalar);

#[derive(Component)]
pub struct JumpImpulse(Scalar);

#[derive(Component)]
pub struct MaxSlopeAngle(Scalar);

// Marker components for caster child entities
#[derive(Component)]
pub struct GroundCaster;

#[derive(Component)]
pub struct WallCasterLeft;

#[derive(Component)]
pub struct WallCasterRight;

#[derive(Component)]
pub enum TouchingWall {
    Left,
    Right,
}

#[derive(Bundle)]
pub struct CharacterControllerBundle {
    character_controller: CharacterController,
    body: RigidBody,
    collider: Collider,
    locked_axes: LockedAxes,
    movement: MovementBundle,
}

#[derive(Bundle)]
pub struct MovementBundle {
    acceleration: MovementAcceleration,
    damping: MovementDampingFactor,
    jump_impulse: JumpImpulse,
    max_slope_angle: MaxSlopeAngle,
    is_moving: IsMoving,
    is_grounded: IsGrounded,
}

impl MovementBundle {
    pub const fn new(
        acceleration: Scalar,
        damping: Scalar,
        jump_impulse: Scalar,
        max_slope_angle: Scalar,
    ) -> Self {
        Self {
            acceleration: MovementAcceleration(acceleration),
            damping: MovementDampingFactor(damping),
            jump_impulse: JumpImpulse(jump_impulse),
            max_slope_angle: MaxSlopeAngle(max_slope_angle),
            is_moving: IsMoving(false),
            is_grounded: IsGrounded(false),
        }
    }
}

impl Default for MovementBundle {
    fn default() -> Self {
        Self::new(30.0, 0.9, 7.0, PI * 0.45)
    }
}

impl CharacterControllerBundle {
    pub fn new(collider: Collider) -> Self {
        Self {
            character_controller: CharacterController,
            body: RigidBody::Dynamic,
            collider,
            locked_axes: LockedAxes::ROTATION_LOCKED,
            movement: MovementBundle::default(),
        }
    }

    pub fn with_movement(
        mut self,
        acceleration: Scalar,
        damping: Scalar,
        jump_impulse: Scalar,
        max_slope_angle: Scalar,
    ) -> Self {
        self.movement = MovementBundle::new(acceleration, damping, jump_impulse, max_slope_angle);
        self
    }
}

/// Spawns caster child entities for any newly added CharacterController.
fn setup_casters(
    mut commands: Commands,
    controllers: Query<(Entity, &Collider), Added<CharacterController>>,
) {
    for (entity, collider) in &controllers {
        let aabb = collider.shape().compute_local_aabb();
        let half_height = aabb.half_extents().y;
        let half_width = aabb.half_extents().x;

        commands.entity(entity).with_children(|parent| {
            // Ground caster — thin rectangle at feet, cast downward
            parent.spawn((
                ShapeCaster::new(
                    Collider::rectangle(half_width * 1.8, 4.0),
                    Vector::new(0.0, -half_height),
                    0.0,
                    Dir2::NEG_Y,
                )
                .with_max_distance(0.2)
                .with_query_filter(SpatialQueryFilter::default().with_excluded_entities([entity])),
                GroundCaster,
                Transform::default(),
            ));

            // Left wall caster — tall thin rectangle at left side, cast left
            parent.spawn((
                ShapeCaster::new(
                    Collider::rectangle(3.0, half_height * 1.2),
                    Vector::new(-half_width, 0.3),
                    0.0,
                    Dir2::NEG_X,
                )
                .with_max_distance(0.2)
                .with_query_filter(SpatialQueryFilter::default().with_excluded_entities([entity])),
                WallCasterLeft,
                Transform::default(),
            ));

            // Right wall caster — tall thin rectangle at right side, cast right
            parent.spawn((
                ShapeCaster::new(
                    Collider::rectangle(3.0, half_height * 1.2),
                    Vector::new(half_width, 0.3),
                    0.0,
                    Dir2::X,
                )
                .with_max_distance(0.2)
                .with_query_filter(SpatialQueryFilter::default().with_excluded_entities([entity])),
                WallCasterRight,
                Transform::default(),
            ));
        });
    }
}

fn keyboard_input(
    mut movement_writer: MessageWriter<MovementAction>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let left = keyboard_input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]);
    let right = keyboard_input.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]);

    let horizontal = right as i8 - left as i8;
    let direction = Vector2::new(horizontal as Scalar, 0.0);

    if direction != Vector2::ZERO {
        movement_writer.write(MovementAction::Move(direction));
    }

    if keyboard_input.just_pressed(KeyCode::Space) {
        movement_writer.write(MovementAction::Jump);
    }
}

fn gamepad_input(mut movement_writer: MessageWriter<MovementAction>, gamepads: Query<&Gamepad>) {
    for gamepad in gamepads.iter() {
        if let Some(x) = gamepad.get(GamepadAxis::LeftStickX) {
            let direction = Vector2::new(x as Scalar, 0.0).clamp_length_max(1.0);
            if direction != Vector2::ZERO {
                movement_writer.write(MovementAction::Move(direction));
            }
        }

        if gamepad.just_pressed(GamepadButton::South) {
            movement_writer.write(MovementAction::Jump);
        }
    }
}

/// Updates Grounded by reading the ground caster child's ShapeHits.
fn update_grounded(
    mut commands: Commands,
    mut controllers: Query<
        (
            Entity,
            &Children,
            Option<&MaxSlopeAngle>,
            &Rotation,
            &mut IsGrounded,
        ),
        With<CharacterController>,
    >,
    ground_casters: Query<&ShapeHits, With<GroundCaster>>,
) {
    for (entity, children, max_slope_angle, rotation, mut is_grounded) in &mut controllers {
        is_grounded.0 = children.iter().any(|child| {
            let Ok(hits) = ground_casters.get(child) else {
                return false;
            };
            hits.iter().any(|hit| {
                if let Some(angle) = max_slope_angle {
                    (rotation * hit.normal1).angle_to(Vector::Y).abs() <= angle.0
                } else {
                    true
                }
            })
        });

        if is_grounded.0 {
            commands.entity(entity).insert(Grounded);
        } else {
            commands.entity(entity).remove::<Grounded>();
        }
    }
}

fn update_wall_contact(
    mut commands: Commands,
    controllers: Query<(Entity, &Children), With<CharacterController>>,
    wall_casters: Query<(
        &ShapeHits,
        Option<&WallCasterLeft>,
        Option<&WallCasterRight>,
    )>,
) {
    for (entity, children) in &controllers {
        commands.entity(entity).remove::<TouchingWall>();
        for child in children.iter() {
            let Ok((hits, left, right)) = wall_casters.get(child) else {
                continue;
            };

            // normal1 is the separation direction for the caster shape.
            // Left caster pushing off a wall on the left → normal1 points right (x > 0).
            // Right caster pushing off a wall on the right → normal1 points left (x < 0).
            // The 0.5 threshold rejects floors/ceilings (mostly-vertical normals).
            if left.is_some() && hits.iter().any(|hit| hit.normal1.x > 0.5) {
                commands.entity(entity).insert(TouchingWall::Left);
            }

            if right.is_some() && hits.iter().any(|hit| hit.normal1.x < -0.5) {
                commands.entity(entity).insert(TouchingWall::Right);
            }
        }
    }
}
fn movement(
    time: Res<Time>,
    mut movement_reader: MessageReader<MovementAction>,
    mut controllers: Query<(
        &MovementAcceleration,
        &JumpImpulse,
        &mut LinearVelocity,
        &IsGrounded,
        &mut IsMoving,
        Option<&TouchingWall>,
    )>,
) {
    let delta_time = time.delta_secs_f64().adjust_precision();

    for event in movement_reader.read() {
        for (
            movement_acceleration,
            jump_impulse,
            mut linear_velocity,
            is_grounded,
            mut is_moving,
            touching_wall,
        ) in &mut controllers
        {
            match event {
                MovementAction::Move(direction) => {
                    is_moving.0 = true;
                    linear_velocity.x += direction.x * movement_acceleration.0 * delta_time;

                    if !is_grounded.0 && touching_wall.is_some() {
                        linear_velocity.y = linear_velocity.y.max(-50.0);
                    }
                }
                MovementAction::Jump => {
                    println!(
                        "jump: grounded={} vel_y={:.1} wall={}",
                        is_grounded.0,
                        linear_velocity.y,
                        touching_wall.is_some(),
                    );
                    if is_grounded.0 {
                        linear_velocity.y = jump_impulse.0;
                    } else if let Some(wall_dir) = touching_wall {
                        if linear_velocity.y > 0.0 {
                            return;
                        }
                        linear_velocity.y = jump_impulse.0;
                        linear_velocity.x = match wall_dir {
                            TouchingWall::Left => jump_impulse.0,
                            TouchingWall::Right => -jump_impulse.0,
                        }
                    }
                }
            }
        }
    }
}

fn apply_movement_damping(
    time: Res<Time>,
    mut query: Query<(&MovementDampingFactor, &mut LinearVelocity, &IsMoving)>,
) {
    for (damping_factor, mut linear_velocity, is_moving) in &mut query {
        if !is_moving.0 {
            linear_velocity.x *= damping_factor
                .0
                .powf(time.delta_secs_f64().adjust_precision());
        }
    }
}

fn reset_is_moving(mut query: Query<&mut IsMoving>) {
    for mut is_moving in &mut query {
        is_moving.0 = false;
    }
}
