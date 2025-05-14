use bevy::{audio::{PlaybackMode, Volume}, ecs::observer::TriggerTargets, math::bounding::{Aabb2d, BoundingCircle, BoundingVolume}, prelude::*};
use bevy_rapier2d::{prelude::{Collider, CollisionEvent, Velocity}, rapier::prelude::CollisionEventFlags};

use crate::{audio_settings::AudioSettings, level::level_layout::CollisionType, Cweampuff, FloorCollider, CWEAMPUFF_DIAMETER};

const CWEAMPUFF_SPEED: f32 = 500.0;
const MAX_CWEAMPUFF_VERTICAL_VELOCITY: f32 = 800.0;
const MAX_DASH_IMPULSE: f32 = 1250.0;
const MAX_WALL_DESCEND_VELOCITY: f32 = 200.0;

#[derive(Component)]
pub struct Jumper {
    pub jump_impulse: f32,
    pub is_jumping: bool,
    pub is_jump_available: bool,
    pub is_next_jump_doublejump: bool,
    pub coyote_jump_buffer_duration: f32,
    pub time_passed_since_stopped_touching_ground: Option<f32>
}

#[derive(Component)]
pub struct Dasher {
    pub is_dash_available: bool,
    pub dash_impulse: f32,
    pub dash_cooldown: f32,
    pub time_passed_since_dash: f32
}

#[derive(Component)]
pub struct Movable {
    pub facing_right: bool,
    pub hugging_left_wall: bool,
    pub hugging_right_wall: bool,
    pub touching_ground: bool,
    pub is_upside_down: bool,
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

    if keyboard_input.any_just_released([KeyCode::ArrowLeft, KeyCode::KeyA, KeyCode::KeyD, KeyCode::ArrowRight]) {
        cweampuff_velocity.linvel.x = 0.;
    } 

    if keyboard_input.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]) {
        if cweampuff_movable.hugging_right_wall {
            cweampuff_velocity.linvel.x = 0.;
            return;
        }

        cweampuff_movable.facing_right = false;

        let new_velocity = -CWEAMPUFF_SPEED;

        if cweampuff_velocity.linvel.x > new_velocity {
            cweampuff_velocity.linvel.x = new_velocity;
        }
    }

    if keyboard_input.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) {
        if cweampuff_movable.hugging_left_wall {
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
    mut cweampuff_jumper: Single<(&mut Jumper, &mut Velocity, &mut Movable, &Cweampuff), With<Cweampuff>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio_settings: Res<AudioSettings>,
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
        if jumper.jump_impulse > 0. && velocity.linvel.y > 0.0 && velocity.linvel.y < jumper.jump_impulse {
            velocity.linvel.y = 0.;
        }
        else if jumper.jump_impulse < 0. && velocity.linvel.y < 0.0 {
            velocity.linvel.y = 0.;
        }
    }

    if keyboard_input.just_pressed(KeyCode::Space) && jumper.is_jump_available && (!jumper.is_jumping || (jumper.is_next_jump_doublejump && cweampuff.has_double_jump)) {
        velocity.linvel.y = jumper.jump_impulse;
        jumper.is_jumping = true;
        jumper.is_jump_available = false;

        let mut playback_settings = PlaybackSettings::default().with_volume(Volume::Linear(audio_settings.sfx_volume));
        playback_settings.mode = PlaybackMode::Despawn;
    
        commands.spawn((
            AudioPlayer::new(asset_server.load("sfx/jump2.wav")),
            playback_settings
        ));

        if jumper.is_next_jump_doublejump {
            jumper.is_next_jump_doublejump = false;
        }

        if jumper.time_passed_since_stopped_touching_ground.is_some() {
            jumper.is_next_jump_doublejump = true;
            movable.touching_ground = false;
            jumper.time_passed_since_stopped_touching_ground = None;
        }

        if (movable.hugging_left_wall || movable.hugging_right_wall) && cweampuff.has_wall_jump {
            let wall_jump_hor_velocity;
    
            if movable.hugging_left_wall {
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
    }
}

pub fn coyote_jump_buffer_monitor(
    mut cweampuff: Single<(&mut Jumper, &mut Movable), With<Cweampuff>>,
    time: Res<Time>
) {
    let (jumper, movable) = &mut *cweampuff;

    if let Some(time_passed_since_stopped_touching_ground) = jumper.time_passed_since_stopped_touching_ground {
        if time_passed_since_stopped_touching_ground <= jumper.coyote_jump_buffer_duration {
            jumper.time_passed_since_stopped_touching_ground = Some(time.delta_secs() + time_passed_since_stopped_touching_ground);
            return;
        } 

        jumper.is_jumping = true;
        jumper.is_next_jump_doublejump = true;
        movable.touching_ground = false;
        jumper.time_passed_since_stopped_touching_ground = None;
    }    
}

pub fn cweampuff_dash(
    keyboard_input: Res<ButtonInput<KeyCode>>, 
    mut cweampuff_dasher: Single<(&mut Dasher, &mut Velocity, &mut Movable, &Cweampuff), With<Cweampuff>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio_settings: Res<AudioSettings>,
) {
    let (dasher, velocity, movable, cweampuff) = &mut *cweampuff_dasher;

    if movable.is_stunlocked || !cweampuff.has_dash {
        return;
    }

    if !keyboard_input.just_pressed(KeyCode::KeyX) || dasher.dash_cooldown - dasher.time_passed_since_dash > 0.01 ||
        (!movable.touching_ground && !dasher.is_dash_available)  {
        return;
    }

    let mut playback_settings = PlaybackSettings::default().with_volume(Volume::Linear(audio_settings.sfx_volume));
    playback_settings.mode = PlaybackMode::Despawn;

    commands.spawn((
        AudioPlayer::new(asset_server.load("sfx/dash.wav")),
        playback_settings
    ));

    let mut vertical_velocity = velocity.linvel.y;

    if movable.is_upside_down {
        if vertical_velocity > 0.0 {
            vertical_velocity = 0.0;
        }
    }
    else if vertical_velocity < 0.0 {
        vertical_velocity = 0.0;
    }
 
    let mut dash_impulse = dasher.dash_impulse;

    if !movable.facing_right {
        dash_impulse = -dash_impulse;
    }

    velocity.linvel = Vec2::new((dash_impulse + velocity.linvel.x).clamp(-MAX_DASH_IMPULSE, MAX_DASH_IMPULSE), vertical_velocity);

    dasher.time_passed_since_dash = 0.;

    if !movable.touching_ground {
        dasher.is_dash_available = false;
    }
}

pub fn reset_abilities(mut cweampuff: Query<(&mut Jumper, &mut Movable, &mut Dasher), With<Cweampuff>>) {
    for (mut jumper, mut movable, mut dasher) in cweampuff.iter_mut() {
        jumper.is_jumping = false;
        jumper.is_next_jump_doublejump = false;
        jumper.is_jump_available = true;
        jumper.time_passed_since_stopped_touching_ground = None;
    
        movable.hugging_left_wall = false;
        movable.hugging_right_wall = false;
        movable.is_stunlocked = false;
        movable.time_passed_since_stun = 0.;
    
        dasher.time_passed_since_dash = dasher.dash_cooldown + 0.1;
        dasher.is_dash_available = true;
    }
}

pub fn velocity_limiter(mut cweampuff: Single<(&mut Velocity, &Cweampuff, &Movable), With<Cweampuff>>) {
    let (cweampuff_velocity, cweampuff, cweampuff_movable) = &mut *cweampuff;

    if (cweampuff_movable.hugging_left_wall || cweampuff_movable.hugging_right_wall) && cweampuff.has_wall_jump  {
        if cweampuff_movable.is_upside_down && cweampuff_velocity.linvel.y > 0. {
            cweampuff_velocity.linvel.y = MAX_WALL_DESCEND_VELOCITY;
        }
        else if cweampuff_velocity.linvel.y < 0. {
            cweampuff_velocity.linvel.y = -MAX_WALL_DESCEND_VELOCITY;
        }
    }

    if cweampuff_movable.is_upside_down {
        cweampuff_velocity.linvel.y = cweampuff_velocity.linvel.y.min(MAX_CWEAMPUFF_VERTICAL_VELOCITY);

        return;
    }

    cweampuff_velocity.linvel.y = cweampuff_velocity.linvel.y.max(-MAX_CWEAMPUFF_VERTICAL_VELOCITY);
}

pub fn dash_reset(mut cweampuff: Single<(&mut Dasher, &Cweampuff), With<Cweampuff>>, time: Res<Time>) {
    let (cweampuff_dasher, cweampuff) = &mut *cweampuff;

    if cweampuff.has_dash && cweampuff_dasher.time_passed_since_dash <= cweampuff_dasher.dash_cooldown {
        cweampuff_dasher.time_passed_since_dash += time.delta_secs();
    } 
}

pub fn cweampuff_asset_direction_monitor(mut cweampuff: Single<(&mut Movable, &mut Sprite), With<Cweampuff>>) {
    let (movable, sprite) = &mut *cweampuff;

    sprite.flip_x = !movable.facing_right;
    sprite.flip_y = movable.is_upside_down;
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
    mut cweampuff: Single<(Entity, &Cweampuff, &mut Jumper, &mut Movable, &mut Dasher, &Transform, &mut Velocity), With<Cweampuff>>,
    mut colliders: Query<(Entity, &Transform, &Collider, &mut FloorCollider), With<FloorCollider>>,
    mut contact_events: EventReader<CollisionEvent>) {
    let (cweampuff_entity, cweampuff, cweampuff_jumper, cweampuff_movable, cweampuff_dasher, cweampuff_transform, cweampuff_velocity) = &mut *cweampuff;

    for contact_event in contact_events.read() {
        detect_floor_and_wall_collision(*cweampuff_entity, cweampuff, cweampuff_jumper, cweampuff_transform, cweampuff_movable, cweampuff_dasher, cweampuff_velocity, contact_event, &mut colliders);
    }
}

fn detect_floor_and_wall_collision(
    cweampuff_entity: Entity, 
    cweampuff: &Cweampuff,
    jumper: &mut Jumper, 
    cweampuff_transform: &Transform,
    cweampuff_movable: &mut Movable,
    cweampuff_dasher: &mut Dasher,
    cweampuff_velocity: &mut Velocity,
    event: &CollisionEvent, 
    colliders: &mut Query<(Entity, &Transform, &Collider, &mut FloorCollider), With<FloorCollider>>
) {
    if let CollisionEvent::Stopped(h1, h2, flags) = event {
        if *flags == CollisionEventFlags::REMOVED {
            if cweampuff_movable.hugging_left_wall {
                cweampuff_movable.hugging_left_wall = false;

                if cweampuff.has_wall_jump {
                    if !jumper.is_jumping {
                        jumper.time_passed_since_stopped_touching_ground = Some(0.);
                    }
                    else {
                        jumper.is_next_jump_doublejump = true;
                    }
                }
            }
            if cweampuff_movable.hugging_right_wall {
                cweampuff_movable.hugging_right_wall = false;

                if cweampuff.has_wall_jump {
                    if !jumper.is_jumping {
                        jumper.time_passed_since_stopped_touching_ground = Some(0.);
                    }
                    else {
                        jumper.is_next_jump_doublejump = true;
                    }
                }
            }
            if !jumper.is_jumping {
                jumper.time_passed_since_stopped_touching_ground = Some(0.);
            }
            else {
                jumper.is_jumping = true;
                jumper.is_next_jump_doublejump = true;
                cweampuff_movable.touching_ground = false;
                jumper.time_passed_since_stopped_touching_ground = None;
            }

            return;
        }

        for (collider_entity, _collider_transform, _collider, mut floor_collider) in colliders.iter_mut() {
            if h1.entities().any(|f| f == collider_entity || f == cweampuff_entity) && 
               h2.entities().any(|f| f == collider_entity || f == cweampuff_entity) {
                if let Some(touching_side) = &floor_collider.currently_touching_side {
                    match touching_side {
                        CollisionType::Ceiling => { },
                        CollisionType::Floor => {
                            if !jumper.is_jumping {
                                jumper.time_passed_since_stopped_touching_ground = Some(0.);
                            }
                            else {
                                jumper.is_jumping = true;
                                jumper.is_next_jump_doublejump = true;
                                cweampuff_movable.touching_ground = false;
                                jumper.time_passed_since_stopped_touching_ground = None;
                            }
                        },
                        CollisionType::LeftWall => {
                            if cweampuff_movable.hugging_left_wall {
                                cweampuff_movable.hugging_left_wall = false;

                                if cweampuff.has_wall_jump {
                                    if !jumper.is_jumping {
                                        jumper.time_passed_since_stopped_touching_ground = Some(0.);
                                    }
                                    else {
                                        jumper.is_next_jump_doublejump = true;
                                    }
                                }
                            }
                        }, 
                        CollisionType::RightWall => {
                            if cweampuff_movable.hugging_right_wall {
                                cweampuff_movable.hugging_right_wall = false;

                                if cweampuff.has_wall_jump {
                                    if !jumper.is_jumping {
                                        jumper.time_passed_since_stopped_touching_ground = Some(0.);
                                    }
                                    else {
                                        jumper.is_next_jump_doublejump = true;
                                    }
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
            if check_entities(h1, h2, &collider_entity, &cweampuff_entity) {
                if let Some(cuboid) = collider.as_cuboid() {
                    let cweampuff_bounds = BoundingCircle::new(cweampuff_transform.translation.truncate(), CWEAMPUFF_DIAMETER / 2.);
                    let collider_bounds = Aabb2d::new(collider_transform.translation.truncate(),
                                                              cuboid.half_extents());

                    match check_collision(cweampuff_bounds, collider_bounds) {
                        CollisionType::LeftWall => {
                            cweampuff_movable.hugging_left_wall = true;
                            floor_collider.currently_touching_side = Some(CollisionType::LeftWall);
                            
                            if cweampuff.has_wall_jump {
                                jumper.time_passed_since_stopped_touching_ground = None;
                                jumper.is_jumping = false;
                                jumper.is_next_jump_doublejump = false;
                                cweampuff_dasher.is_dash_available = true;
                            }
                        },
                        CollisionType::RightWall => {
                            cweampuff_movable.hugging_right_wall = true;
                            floor_collider.currently_touching_side = Some(CollisionType::RightWall);

                            if cweampuff.has_wall_jump {
                                jumper.time_passed_since_stopped_touching_ground = None;
                                jumper.is_jumping = false;
                                jumper.is_next_jump_doublejump = false;
                                cweampuff_dasher.is_dash_available = true;
                            }
                        },
                        CollisionType::Floor => {
                            if !cweampuff_movable.is_upside_down {
                                floor_collider.currently_touching_side = Some(CollisionType::Floor);
                                cweampuff_velocity.linvel.y = 0.;
                                jumper.is_jumping = false;
                                jumper.is_next_jump_doublejump = false;
                                cweampuff_movable.touching_ground = true;
                                cweampuff_dasher.is_dash_available = true;
                                jumper.time_passed_since_stopped_touching_ground = None;
                            }
                            else {
                                floor_collider.currently_touching_side = Some(CollisionType::Ceiling);
                                cweampuff_velocity.linvel.y = 0.;
                            }
                            
                        },
                        CollisionType::Ceiling => {
                            if !cweampuff_movable.is_upside_down {
                                floor_collider.currently_touching_side = Some(CollisionType::Ceiling);
                                cweampuff_velocity.linvel.y = 0.;
                            }
                            else {
                                floor_collider.currently_touching_side = Some(CollisionType::Floor);
                                cweampuff_velocity.linvel.y = 0.;
                                jumper.is_jumping = false;
                                jumper.is_next_jump_doublejump = false;
                                cweampuff_movable.touching_ground = true;
                                cweampuff_dasher.is_dash_available = true;
                                jumper.time_passed_since_stopped_touching_ground = None;
                            }
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
        let center_offset = cweampuff_bounds.center() - wall_bounds.center();

        if offset.x < 0. || center_offset.x < 0. {
            return CollisionType::LeftWall;
        }

        return CollisionType::RightWall;
    }

    if offset.y > 0. {
        return CollisionType::Floor;
    }

    CollisionType::Ceiling
}

pub fn check_entities(h1: &Entity, h2: &Entity, collider: &Entity, cweampuff: &Entity) -> bool {
    h1.entities().any(|f| f == *collider || f == *cweampuff) && h2.entities().any(|f| f == *collider || f == *cweampuff)
}