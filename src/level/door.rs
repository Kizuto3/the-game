use bevy::{audio::{PlaybackMode, Volume}, prelude::*};
use bevy_rapier2d::prelude::CollisionEvent;

use crate::{audio_settings::AudioSettings, interactable::{interaction_state::InteractionState, Interactable}, npc::NPC, Cweampuff};
use crate::movement::check_entities;
use super::{level_layout::{DoorCollider, DoorType}, manually_transition_to_level, transition_states::TransitionState, LevelLayout};

pub fn interactable_door_collision_reader(
    mut doors: Query<(Entity, &mut DoorCollider), (With<Interactable>, Without<NPC>)>,
    cweampuff: Single<Entity, With<Cweampuff>>,
    mut contact_events: EventReader<CollisionEvent>,
    mut interaction_state: ResMut<NextState<InteractionState>> 
) {
    for event in contact_events.read() {
        if let CollisionEvent::Stopped(h1, h2, _flags) = event {
            for (door_entity, mut door) in doors.iter_mut() {
                if check_entities(h1, h2, &door_entity, &cweampuff) {
                    door.is_active = false;
                    interaction_state.set(InteractionState::NotReady);

                    return;
                }
            }
        }
    
        if let CollisionEvent::Started(h1, h2, _flags) = event {
            for (door_entity, mut door) in doors.iter_mut() {
                if check_entities(h1, h2, &door_entity, &cweampuff) {
                    door.is_active = true;
                    interaction_state.set(InteractionState::Ready);

                    return;
                }
            }
        }
    }
}

pub fn door_start_interaction_input_reader(
    keyboard_input: Res<ButtonInput<KeyCode>>, 
    doors: Query<&DoorCollider, (With<Interactable>, Without<NPC>)>,
    cweampuff: Single<&Cweampuff, With<Cweampuff>>,
    mut commands: Commands,
    current_level_layout: Query<Entity, With<LevelLayout>>,
    mut transition_state: ResMut<NextState<TransitionState>>,
    asset_server: Res<AssetServer>,
    audio_settings: Res<AudioSettings>,
) {
    if !keyboard_input.just_pressed(KeyCode::KeyE) {
        return;
    }

    if let Some(door) = doors.iter().find(|f| f.is_active) {
        let mut playback_settings = PlaybackSettings::default().with_volume(Volume::new(audio_settings.sfx_volume));
        playback_settings.mode = PlaybackMode::Despawn;
    
        match door.door_type {
            DoorType::Door => {
                commands.spawn((
                    AudioPlayer::new(asset_server.load("sfx/door.wav")),
                    playback_settings
                ));
            },
            DoorType::Teleport => {
                commands.spawn((
                    AudioPlayer::new(asset_server.load("sfx/gravity.wav")),
                    playback_settings
                ));
            }
        }
        
        manually_transition_to_level(&current_level_layout, &mut transition_state, &cweampuff, &mut commands, door.transition_to_level, door.safe_position);
    }
}