use bevy::{ecs::observer::TriggerTargets, math::bounding::{Aabb2d, BoundingCircle, BoundingVolume}, prelude::*};
use bevy_rapier2d::prelude::{Collider, CollisionEvent, Velocity};

use crate::{level::level_layout::CollisionType, Cweampuff, FloorCollider, CWEAMPUFF_DIAMETER};

const CWEAMPUFF_SPEED: f32 = 500.0;
const MAX_CWEAMPUFF_VERTICAL_VELOCITY: f32 = 800.0;
const MAX_DASH_IMPULSE: f32 = 1250.0;
const MAX_WALL_DESCEND_VELOCITY: f32 = 200.0;

#[derive(Component)]
pub struct Jumper {
    pub jump_impulse: f32,
    pub is_jumping: bool,
    pub is_jump_available: bool,
    pub is_next_jump_doublejump: bool
}

#[derive(Component)]
pub struct Dasher {
    pub dash_impulse: f32,
    pub dash_cooldown: f32,
    pub time_passed_since_dash: f32
}

#[derive(Component)]
pub struct Movable {
    pub facing_right: bool,
    pub hugging_wall: bool,
    pub is_stunlocked: bool,
    pub stun_duration: f32,
    pub time_passed_since_stun: f32
}

pub fn cweampuff_move(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut cweampuff_transform_velocity: Single<(&mut Velocity, &mut Movable), With<Cweampuff>>
) {
    let (cweampuff_velocity, cweampuff_movable) = &mut *cweampuff_transform_velocity;

    if cweampuff_movable.is_stunlocked {
        return;
    }

    if keyboard_input.just_released(KeyCode::ArrowLeft) || keyboard_input.just_released(KeyCode::ArrowRight) {
        cweampuff_velocity.linvel.x = 0.;
    } 

    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        if cweampuff_movable.hugging_wall && !cweampuff_movable.facing_right {
            cweampuff_velocity.linvel.x = 0.;
            return;
        }

        cweampuff_movable.facing_right = false;

        let new_velocity = -CWEAMPUFF_SPEED;

        if cweampuff_velocity.linvel.x > new_velocity {
            cweampuff_velocity.linvel.x = new_velocity;
        }
    }

    if keyboard_input.pressed(KeyCode::ArrowRight) {
        if cweampuff_movable.hugging_wall && cweampuff_movable.facing_right {
            cweampuff_velocity.linvel.x = 0.;
            return;
        }

        cweampuff_movable.facing_right = true;

        let new_velocity = CWEAMPUFF_SPEED;

        if cweampuff_velocity.linvel.x < new_velocity {
            cweampuff_velocity.linvel.x = new_velocity;
        }
    }
}

pub fn cweampuff_jump(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut cweampuff_jumper: Single<(&mut Jumper, &mut Velocity, &mut Movable, &Cweampuff), With<Cweampuff>>
) {
    let (jumper, velocity, movable, cweampuff) = &mut *cweampuff_jumper;

    let jump_released = keyboard_input.just_released(KeyCode::Space);

    if jump_released {
        jumper.is_jump_available = true;
    }

    if movable.is_stunlocked {
        return;
    }

    if jump_released {
        // Kill current vertical velocity 
        if velocity.linvel.y > 0.0 && velocity.linvel.y < jumper.jump_impulse {
            velocity.linvel.y = 0.;
        }
    }

    if keyboard_input.just_pressed(KeyCode::Space) && jumper.is_jump_available && (!jumper.is_jumping || (jumper.is_next_jump_doublejump && cweampuff.has_double_jump)) {
        velocity.linvel.y = jumper.jump_impulse;
        jumper.is_jumping = true;
        jumper.is_jump_available = false;
    
        if movable.hugging_wall && cweampuff.has_wall_jump {
            let wall_jump_hor_velocity;
    
            if movable.facing_right {
                wall_jump_hor_velocity = -CWEAMPUFF_SPEED;
                movable.facing_right = false;
            }
            else {
                movable.facing_right = true;
                wall_jump_hor_velocity = CWEAMPUFF_SPEED;
            }
    
            movable.is_stunlocked = true;
            velocity.linvel.x = wall_jump_hor_velocity;
        }
    
        if jumper.is_next_jump_doublejump {
            jumper.is_next_jump_doublejump = false;
        }
    }
}

pub fn cweampuff_dash(
    keyboard_input: Res<ButtonInput<KeyCode>>, 
    mut cweampuff_dasher: Single<(&mut Dasher, &mut Velocity, &mut Movable, &Cweampuff), With<Cweampuff>>
) {
    let (dasher, velocity, movable, cweampuff) = &mut *cweampuff_dasher;

    if movable.is_stunlocked || !cweampuff.has_dash {
        return;
    }

    if !keyboard_input.just_pressed(KeyCode::KeyX) || dasher.dash_cooldown - dasher.time_passed_since_dash > 0.01  {
        return;
    }

    let mut vertical_velocity = velocity.linvel.y;

    if vertical_velocity < 0.0 {
        vertical_velocity = 0.0;
    }
 
    let mut dash_impulse = dasher.dash_impulse;

    if !movable.facing_right {
        dash_impulse = -dash_impulse;
    }

    velocity.linvel = Vec2::new((dash_impulse + velocity.linvel.x).clamp(-MAX_DASH_IMPULSE, MAX_DASH_IMPULSE), vertical_velocity);

    dasher.time_passed_since_dash = 0.;
}

pub fn reset_abilities(mut cweampuff: Query<(&mut Jumper, &mut Movable, &mut Dasher), With<Cweampuff>>) {
    for (mut jumper, mut movable, mut dasher) in cweampuff.iter_mut() {
        jumper.is_jumping = false;
        jumper.is_next_jump_doublejump = false;
        jumper.is_jump_available = true;
    
        movable.hugging_wall = false;
        movable.is_stunlocked = false;
        movable.time_passed_since_stun = 0.;
    
        dasher.time_passed_since_dash = dasher.dash_cooldown + 0.1;
    }
}

pub fn velocity_limiter(mut cweampuff: Single<(&mut Velocity, &Cweampuff, &Movable), With<Cweampuff>>) {
    let (cweampuff_velocity, cweampuff, cweampuff_movable) = &mut *cweampuff;

    if cweampuff_movable.hugging_wall && cweampuff.has_wall_jump && cweampuff_velocity.linvel.y < 0. {
        cweampuff_velocity.linvel.y = -MAX_WALL_DESCEND_VELOCITY;
    }

    cweampuff_velocity.linvel.y = cweampuff_velocity.linvel.y.max(-MAX_CWEAMPUFF_VERTICAL_VELOCITY);
}

pub fn dash_reset(mut cweampuff: Single<(&mut Dasher, &Cweampuff), With<Cweampuff>>, time: Res<Time>) {
    let (cweampuff_dasher, cweampuff) = &mut *cweampuff;

    if cweampuff.has_dash && cweampuff_dasher.time_passed_since_dash <= cweampuff_dasher.dash_cooldown {
        cweampuff_dasher.time_passed_since_dash += time.delta_secs();
    } 
}

pub fn stunlock_reset(mut cweampuff_movable: Single<&mut Movable, With<Cweampuff>>, time: Res<Time>) {
    if !cweampuff_movable.is_stunlocked {
        return;
    }

    if cweampuff_movable.time_passed_since_stun <= cweampuff_movable.stun_duration {
        cweampuff_movable.time_passed_since_stun += time.delta_secs();
        return;
    }

    cweampuff_movable.is_stunlocked = false;
    cweampuff_movable.time_passed_since_stun = 0.;
}

pub fn jump_reset(
    mut cweampuff: Single<(Entity, &Cweampuff, &mut Jumper, &mut Movable, &Transform, &mut Velocity), With<Cweampuff>>,
    mut colliders: Query<(Entity, &Transform, &Collider, &mut FloorCollider), With<FloorCollider>>,
    mut contact_events: EventReader<CollisionEvent>) {
    let (cweampuff_entity, cweampuff, cweampuff_jumper, cweampuff_movable, cweampuff_transform, cweampuff_velocity) = &mut *cweampuff;

    for contact_event in contact_events.read() {
        detect_floor_and_wall_collision(*cweampuff_entity, cweampuff, cweampuff_jumper, cweampuff_transform, cweampuff_movable, cweampuff_velocity, contact_event, &mut colliders);
    }
}

fn detect_floor_and_wall_collision(
    cweampuff_entity: Entity, 
    cweampuff: &Cweampuff,
    jumper: &mut Jumper, 
    cweampuff_transform: &Transform,
    cweampuff_movable: &mut Movable,
    cweampuff_velocity: &mut Velocity,
    event: &CollisionEvent, 
    colliders: &mut Query<(Entity, &Transform, &Collider, &mut FloorCollider), With<FloorCollider>>
) {
    if let CollisionEvent::Stopped(h1, h2, _flags) = event {
        for (collider_entity, _collider_transform, _collider, mut floor_collider) in colliders.iter_mut() {
            if h1.entities().iter().any(|f| *f == collider_entity || *f == cweampuff_entity) && 
               h2.entities().iter().any(|f| *f == collider_entity || *f == cweampuff_entity) {
                if let Some(touching_side) = &floor_collider.currently_touching_side {
                    match touching_side {
                        CollisionType::Ceiling => { },
                        CollisionType::Floor => {
                            jumper.is_jumping = true;
                            jumper.is_next_jump_doublejump = true;
                        },
                        CollisionType::Wall => {
                            if cweampuff_movable.hugging_wall {
                                cweampuff_movable.hugging_wall = false;

                                if cweampuff.has_wall_jump {
                                    jumper.is_next_jump_doublejump = true;
                                }
                            }
                        }
                    };

                    floor_collider.currently_touching_side = None;
                }
            }
        }
    }

    if let CollisionEvent::Started(h1, h2, _flags) = event {
        for (collider_entity, collider_transform, collider, mut floor_collider) in colliders.iter_mut() {
            if h1.entities().iter().any(|f| *f == collider_entity || *f == cweampuff_entity) && 
               h2.entities().iter().any(|f| *f == collider_entity || *f == cweampuff_entity) {
                if let Some(cuboid) = collider.as_cuboid() {
                    let cweampuff_bounds = BoundingCircle::new(cweampuff_transform.translation.truncate(), CWEAMPUFF_DIAMETER / 2.);
                    let collider_bounds = Aabb2d::new(collider_transform.translation.truncate(),
                                                              cuboid.half_extents());

                    match check_collision(cweampuff_bounds, collider_bounds) {
                        CollisionType::Wall => {
                            cweampuff_movable.hugging_wall = true;
                            floor_collider.currently_touching_side = Some(CollisionType::Wall);
                            
                            if cweampuff.has_wall_jump {
                                jumper.is_jumping = false;
                                jumper.is_next_jump_doublejump = false;
                            }
                        },
                        CollisionType::Floor => {
                            floor_collider.currently_touching_side = Some(CollisionType::Floor);
                            cweampuff_velocity.linvel.y = 0.;
                            jumper.is_jumping = false;
                            jumper.is_next_jump_doublejump = false;
                        },
                        CollisionType::Ceiling => {
                            floor_collider.currently_touching_side = Some(CollisionType::Ceiling);
                            cweampuff_velocity.linvel.y = 0.;
                        }               
                    };
                }
            }
        }
    }
}

fn check_collision(cweampuff_bounds: BoundingCircle, wall_bounds: Aabb2d) -> CollisionType {
    let closest = wall_bounds.closest_point(cweampuff_bounds.center());
    let offset = cweampuff_bounds.center() - closest;

    if offset.x.abs() >= offset.y.abs() {
        return CollisionType::Wall;
    }

    if offset.y > 0. {
        return CollisionType::Floor;
    }

    CollisionType::Ceiling
}