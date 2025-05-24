use std::time::Duration;

use bevy::{audio::{PlaybackMode, Volume}, prelude::*};
use bevy_rapier2d::{prelude::*, rapier::prelude::CollisionEventFlags};

use crate::{audio_settings::AudioSettings, interactable::{interaction_state::InteractionState, Interactable}, movement::{Jumper, Movable}, npc::NPC, Cweampuff};
use crate::movement::check_entities;
use super::level_layout::{BreakableWall, DoorCollider, FloorAssetType, FloorCollider, GravityInverter, JumpPad, TimeTrial};

const JUMP_PAD_VELOCITY_DELTA: f32 = 2.;

#[derive(Component)]
pub struct TimeTrialTimer {
    pub timer: Timer,
    pub entity_id: u32
}

pub fn jump_pad_collision_reader(
    jump_pads: Query<Entity, (With<Sensor>, With<JumpPad>, Without<NPC>, Without<DoorCollider>)>,
    mut cweampuff: Single<(Entity, &mut Velocity, &Jumper), With<Cweampuff>>,
    mut contact_events: EventReader<CollisionEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio_settings: Res<AudioSettings>,
) {
    let (cweampuff_entity, cweampuff_velocity, cweampuff_jumper) = &mut *cweampuff;
    for event in contact_events.read() {    
        if let CollisionEvent::Started(h1, h2, _) = event {
            for jump_pad_entity in jump_pads.iter() {
                if check_entities(h1, h2, &jump_pad_entity, cweampuff_entity) {
                    cweampuff_velocity.linvel.y = cweampuff_jumper.jump_impulse * JUMP_PAD_VELOCITY_DELTA;

                    let mut playback_settings = PlaybackSettings::default().with_volume(Volume::Linear(audio_settings.sfx_volume));
                    playback_settings.mode = PlaybackMode::Despawn;
                
                    commands.spawn((
                        AudioPlayer::new(asset_server.load("sfx/woosh.wav")),
                        playback_settings
                    ));

                    return;
                }
            }
        }
    }
}

pub fn gravity_inverter_collision_reader(
    jump_pads: Query<Entity, (With<Sensor>, With<GravityInverter>, Without<NPC>, Without<DoorCollider>)>,
    mut cweampuff: Single<(Entity, &mut Jumper, &mut GravityScale, &mut Movable), With<Cweampuff>>,
    mut contact_events: EventReader<CollisionEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio_settings: Res<AudioSettings>,
) {
    let (cweampuff_entity, cweampuff_jumper, cweampuff_gravity, cweampuff_movable) = &mut *cweampuff;
    for event in contact_events.read() {    
        if let CollisionEvent::Started(h1, h2, _) = event {
            for jump_pad_entity in jump_pads.iter() {
                if check_entities(h1, h2, &jump_pad_entity, cweampuff_entity) {
                    cweampuff_gravity.0 = -cweampuff_gravity.0;
                    cweampuff_jumper.jump_impulse = -cweampuff_jumper.jump_impulse;
                    cweampuff_movable.is_upside_down = true;

                    let mut playback_settings = PlaybackSettings::default().with_volume(Volume::Linear(audio_settings.sfx_volume));
                    playback_settings.mode = PlaybackMode::Despawn;
                
                    commands.spawn((
                        AudioPlayer::new(asset_server.load("sfx/gravity.wav")),
                        playback_settings
                    ));

                    return;
                }
            }
        }

        if let CollisionEvent::Stopped(h1, h2, flags) = event {
            if *flags == CollisionEventFlags::REMOVED {
                cweampuff_gravity.0 = cweampuff_gravity.0.abs();
                cweampuff_jumper.jump_impulse = cweampuff_jumper.jump_impulse.abs();
                cweampuff_movable.is_upside_down = false;
            }

            for jump_pad_entity in jump_pads.iter() {
                if check_entities(h1, h2, &jump_pad_entity, cweampuff_entity) {
                    cweampuff_gravity.0 = cweampuff_gravity.0.abs();
                    cweampuff_jumper.jump_impulse = cweampuff_jumper.jump_impulse.abs();
                    cweampuff_movable.is_upside_down = false;

                    return;
                }
            }
        }
    }
}

pub fn time_trial_collision_reader(
    mut time_trials: Query<(Entity, &mut TimeTrial), (With<Interactable>, Without<NPC>)>,
    cweampuff: Single<Entity, With<Cweampuff>>,
    mut contact_events: EventReader<CollisionEvent>,
    mut interaction_state: ResMut<NextState<InteractionState>> 
) {
    for event in contact_events.read() {
        if let CollisionEvent::Stopped(h1, h2, _) = event {
            for (time_trial_entity, mut time_trial) in time_trials.iter_mut() {
                if check_entities(h1, h2, &time_trial_entity, &cweampuff) {
                    time_trial.is_active = false;
                    interaction_state.set(InteractionState::NotReady);

                    return;
                }
            }
        }
    
        if let CollisionEvent::Started(h1, h2, _) = event {
            for (door_entity, mut door) in time_trials.iter_mut() {
                if check_entities(h1, h2, &door_entity, &cweampuff) {
                    door.is_active = true;
                    interaction_state.set(InteractionState::Ready);

                    return;
                }
            }
        }
    }
}

pub fn time_trial_start_interaction_input_reader(
    keyboard_input: Res<ButtonInput<KeyCode>>, 
    mut time_trials: Query<(&TimeTrial, &mut Sprite), (With<Interactable>, Without<NPC>)>,
    mut timers: Query<&mut TimeTrialTimer>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio_settings: Res<AudioSettings>,
) {
    if !keyboard_input.just_pressed(KeyCode::KeyE) {
        return;
    }

    if let Some((time_trial, mut time_trial_sprite)) = time_trials.iter_mut().find(|f| f.0.is_active) {
        let mut playback_settings = PlaybackSettings::default().with_volume(Volume::Linear(audio_settings.sfx_volume));
        playback_settings.mode = PlaybackMode::Despawn;
    
        commands.spawn((
            AudioPlayer::new(asset_server.load("sfx/lever.wav")),
            playback_settings
        ));

        if let Some(mut timer) = timers.iter_mut().find(|x| x.entity_id == time_trial.id) {
            timer.timer.reset();

            return;
        }

        let texture = asset_server.load("floor_modifications/Lever2.png");
        time_trial_sprite.image = texture;

        commands.spawn(
            TimeTrialTimer { timer: Timer::new(Duration::from_secs(time_trial.seconds_to_complete), TimerMode::Once), entity_id: time_trial.id }
        );

        for floor in time_trial.floor_infos.iter() {
            let tile_handle = match floor.floor_asset {
                FloorAssetType::Forest => asset_server.load("tiles/Forest.png"),
                FloorAssetType::CweamcatHouse => asset_server.load("tiles/CweamcatHouse.png"),
                FloorAssetType::Hell => asset_server.load("tiles/Hell.png"),
                FloorAssetType::Spaceship => asset_server.load("tiles/Spaceship.png"),
                FloorAssetType::Factory => asset_server.load("tiles/Factory.png")
            };

            let mut floor_command = commands.spawn((
                RigidBody::Fixed,
                Transform::from_translation(floor.position),
                Sprite {
                    image: tile_handle,
                    anchor: bevy::sprite::Anchor::Center,
                    custom_size: Some(Vec2::new(floor.size.x, floor.size.y)),
                    image_mode: SpriteImageMode::Sliced(TextureSlicer {
                        border: BorderRect { left: 18., right: 15., top: 38., bottom: 11. },
                        center_scale_mode: SliceScaleMode::Tile { stretch_value: 1. },
                        sides_scale_mode: SliceScaleMode::Tile { stretch_value: 1. },
                        max_corner_scale: 1.0
                    }),
                    ..default()
                },
                Collider::cuboid(floor.size.x / 2.0, floor.size.y / 2.0),
                Friction::coefficient(0.7),
                ActiveEvents::COLLISION_EVENTS,
                FloorCollider::default()
            ));

            if let Some(breakable_wall) = floor.breakable_wall {
                floor_command.insert(breakable_wall);
            }
        }
    }
}

pub fn tick_timer_trial_timer(
    mut commands: Commands,
    mut timers: Query<(Entity, &mut TimeTrialTimer)>,
    mut time_trials: Query<(&TimeTrial, &mut Sprite), (With<Interactable>, With<TimeTrial>, Without<NPC>)>,
    floors: Query<(Entity, &BreakableWall), With<BreakableWall>>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
) {
    for (timer_entity, mut timer) in timers.iter_mut() {
        timer.timer.tick(time.delta());

        if timer.timer.finished() {
            if let Some((_, mut time_trial_sprite)) = time_trials.iter_mut().find(|f| f.0.id == timer.entity_id) {
                let texture = asset_server.load("floor_modifications/Lever1.png");
                time_trial_sprite.image = texture;
            }

            commands.entity(timer_entity).despawn();

            for (floor_entity, breakable_wall) in floors.iter() {
                if breakable_wall.index != timer.entity_id {
                    continue;
                }
                
                commands.entity(floor_entity).despawn();
            }
        }
    }
}