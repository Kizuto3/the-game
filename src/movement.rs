use bevy::{ecs::observer::TriggerTargets, math::bounding::{Aabb2d, BoundingCircle, BoundingVolume}, prelude::*};
use bevy_rapier2d::prelude::{Collider, CollisionEvent, Velocity};

use crate::{Cweampuf, FloorCollider, CWEAMPUF_DIAMETER};

const CWEAMPUF_SPEED: f32 = 500.0;
const MAX_CWEAMPUF_VERTICAL_VELOCITY: f32 = 800.0;
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

pub fn cweampuf_move(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut cweampuf_transform_velocity: Single<(&mut Velocity, &mut Movable), With<Cweampuf>>) {
    let mut direction;
    let (cweampuf_velocity, cweampuf_movable) = &mut *cweampuf_transform_velocity;

    if cweampuf_movable.is_stunlocked {
        return;
    }

    if keyboard_input.just_released(KeyCode::ArrowLeft) || keyboard_input.just_released(KeyCode::ArrowRight) {
        cweampuf_velocity.linvel.x = 0.;
    } 

    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        if cweampuf_movable.hugging_wall && !cweampuf_movable.facing_right {
            cweampuf_velocity.linvel.x = 0.;
            return;
        }

        direction = -1.0;
        cweampuf_movable.facing_right = false;

        let new_velocity = direction * CWEAMPUF_SPEED;

        if cweampuf_velocity.linvel.x > new_velocity {
            cweampuf_velocity.linvel.x = new_velocity;
        }
    }

    if keyboard_input.pressed(KeyCode::ArrowRight) {
        if cweampuf_movable.hugging_wall && cweampuf_movable.facing_right {
            cweampuf_velocity.linvel.x = 0.;
            return;
        }

        direction = 1.0;
        cweampuf_movable.facing_right = true;

        let new_velocity = direction * CWEAMPUF_SPEED;

        if cweampuf_velocity.linvel.x < new_velocity {
            cweampuf_velocity.linvel.x = new_velocity;
        }
    }
}

pub fn cweampuf_jump(keyboard_input: Res<ButtonInput<KeyCode>>,
    mut cweampuf_jumper: Single<(&mut Jumper, &mut Velocity, &mut Movable, &Cweampuf), With<Cweampuf>>) {
    let (jumper, velocity, movable, cweampuff) = &mut *cweampuf_jumper;

    let jump_released = keyboard_input.just_released(KeyCode::Space);

    if jump_released {
        jumper.is_jump_available = true;
    }

    if movable.is_stunlocked {
        return;
    }

    if jump_released {
        // Kill current vertical velocity 
        if velocity.linvel.y > 0.0 {
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
                wall_jump_hor_velocity = -CWEAMPUF_SPEED;
                movable.facing_right = false;
            }
            else {
                movable.facing_right = true;
                wall_jump_hor_velocity = CWEAMPUF_SPEED;
            }
    
            movable.is_stunlocked = true;
            velocity.linvel.x = wall_jump_hor_velocity;
        }
    
        if jumper.is_next_jump_doublejump {
            jumper.is_next_jump_doublejump = false;
        }
    }
}

pub fn cweampuf_dash(
    keyboard_input: Res<ButtonInput<KeyCode>>, 
    mut cweampuf_dasher: Single<(&mut Dasher, &mut Velocity, &mut Movable), With<Cweampuf>>) {
    let (dasher, velocity, movable) = &mut *cweampuf_dasher;

    if movable.is_stunlocked {
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

    velocity.linvel = Vec2::new((dash_impulse + velocity.linvel.x).clamp(-MAX_DASH_IMPULSE, MAX_DASH_IMPULSE), vertical_velocity).into();

    dasher.time_passed_since_dash = 0.;
}

pub fn reset_abilities(
    mut cweampuf: Query<(&mut Jumper, &mut Movable, &mut Dasher), With<Cweampuf>>,
) {
    for (mut jumper, mut movable, mut dasher) in cweampuf.iter_mut() {
        jumper.is_jumping = false;
        jumper.is_next_jump_doublejump = false;
        jumper.is_jump_available = true;
    
        movable.hugging_wall = false;
        movable.is_stunlocked = false;
        movable.time_passed_since_stun = movable.stun_duration + 0.1;
    
        dasher.time_passed_since_dash = dasher.dash_cooldown + 0.1;
    }
}

pub fn velocity_limiter(mut cweampuf: Single<(&mut Velocity, &Cweampuf, &Movable), With<Cweampuf>>) {
    let (cweampuf_velocity, cweampuff, cweampuf_movable) = &mut *cweampuf;

    if cweampuf_movable.hugging_wall && cweampuff.has_wall_jump && cweampuf_velocity.linvel.y < 0. {
        cweampuf_velocity.linvel.y = -MAX_WALL_DESCEND_VELOCITY;
    }

    cweampuf_velocity.linvel.y = cweampuf_velocity.linvel.y.clamp(-MAX_CWEAMPUF_VERTICAL_VELOCITY, MAX_CWEAMPUF_VERTICAL_VELOCITY);
}

pub fn dash_reset(mut cweampuf_dasher: Single<&mut Dasher, With<Cweampuf>>, time: Res<Time>) {
    if cweampuf_dasher.time_passed_since_dash <= cweampuf_dasher.dash_cooldown {
        cweampuf_dasher.time_passed_since_dash += time.delta_secs();
    } 
}

pub fn stunlock_reset(mut cweampuf_movable: Single<&mut Movable, With<Cweampuf>>, time: Res<Time>) {
    if !cweampuf_movable.is_stunlocked {
        return;
    }

    if cweampuf_movable.time_passed_since_stun <= cweampuf_movable.stun_duration {
        cweampuf_movable.time_passed_since_stun += time.delta_secs();
        return;
    }

    cweampuf_movable.is_stunlocked = false;
    cweampuf_movable.time_passed_since_stun = 0.;
}

pub fn jump_reset(
    mut cweampuf: Single<(Entity, &Cweampuf, &mut Jumper, &mut Movable, &Transform, &mut Velocity), With<Cweampuf>>,
    mut colliders: Query<(Entity, &Transform, &Collider, &mut FloorCollider), With<FloorCollider>>,
    mut contact_events: EventReader<CollisionEvent>) {
    let (cweampuf_entity, cweampuff, cweampuf_jumper, cweampuf_movable, cweampuf_transform, cweampuf_velocity) = &mut *cweampuf;

    for contact_event in contact_events.read() {
        detect_floor_and_wall_collision(*cweampuf_entity, &cweampuff, cweampuf_jumper, cweampuf_transform, cweampuf_movable, cweampuf_velocity, contact_event, &mut colliders);
    }
}

fn detect_floor_and_wall_collision(cweampuf_entity: Entity, 
                                   cweampuff: &Cweampuf,
                                   jumper: &mut Jumper, 
                                   cweampuf_transform: &Transform,
                                   cweampuf_movable: &mut Movable,
                                   cweampuf_velocity: &mut Velocity,
                                   event: &CollisionEvent, 
                                   colliders: &mut Query<(Entity, &Transform, &Collider, &mut FloorCollider), With<FloorCollider>>) {
    
    if let CollisionEvent::Stopped(h1, h2, _flags) = event {
        for (collider_entity, _collider_transform, _collider, mut floor_collider) in colliders.iter_mut() {
            if h1.entities().iter().any(|f| *f == collider_entity || *f == cweampuf_entity) && 
               h2.entities().iter().any(|f| *f == collider_entity || *f == cweampuf_entity) {
                if cweampuf_movable.hugging_wall && floor_collider.entity_index != 0 {
                    cweampuf_movable.hugging_wall = false;
                    floor_collider.entity_index = 0;
                }
                else {
                    jumper.is_jumping = true;
                    jumper.is_next_jump_doublejump = true;
                }

                if cweampuff.has_wall_jump {
                    jumper.is_next_jump_doublejump = true;
                }
            }
        }
    }

    if let CollisionEvent::Started(h1, h2, _flags) = event {
        for (collider_entity, collider_transform, collider, mut floor_collider) in colliders.iter_mut() {
            if h1.entities().iter().any(|f| *f == collider_entity || *f == cweampuf_entity) && 
               h2.entities().iter().any(|f| *f == collider_entity || *f == cweampuf_entity) {
                if let Some(cuboid) = collider.as_cuboid() {
                    let cweampuf_bounds = BoundingCircle::new(cweampuf_transform.translation.truncate(), CWEAMPUF_DIAMETER / 2.);
                    let collider_bounds = Aabb2d::new(collider_transform.translation.truncate(),
                                                              cuboid.half_extents());

                    if check_wall_collision(cweampuf_bounds, collider_bounds) {
                        cweampuf_movable.hugging_wall = true;
                        floor_collider.entity_index = collider_entity.index();

                        if cweampuff.has_wall_jump {
                            jumper.is_jumping = false;
                            jumper.is_next_jump_doublejump = false;
                        }
                    }
                    else {
                        cweampuf_velocity.linvel.y = 0.;
                        jumper.is_jumping = false;
                        jumper.is_next_jump_doublejump = false;
                    }
                }
            }
        }
    }
}

fn check_wall_collision(cweampuf_bounds: BoundingCircle, wall_bounds: Aabb2d) -> bool {
    let closest = wall_bounds.closest_point(cweampuf_bounds.center());
    let offset = cweampuf_bounds.center() - closest;

    return offset.x.abs() >= offset.y.abs();
}