use bevy::{ecs::observer::TriggerTargets, prelude::*};
use bevy_rapier2d::prelude::CollisionEvent;

use crate::{interactable::{interaction_state::InteractionState, Interactable}, npc::NPC, Cweampuf};

use super::{level_layout::DoorCollider, manually_transition_to_level, transition_states::TransitionState, LevelLayout, LevelTransitionEvent};

pub fn interactable_door_collision_reader(
    mut doors: Query<(Entity, &mut DoorCollider), (With<Interactable>, Without<NPC>)>,
    cweampuf: Single<Entity, With<Cweampuf>>,
    mut contact_events: EventReader<CollisionEvent>,
    mut interaction_state: ResMut<NextState<InteractionState>> 
) {
    for event in contact_events.read() {
        if let CollisionEvent::Stopped(h1, h2, _flags) = event {
            for (door_entity, mut door) in doors.iter_mut() {
                if h1.entities().iter().any(|f| *f == door_entity || *f == *cweampuf) && 
                   h2.entities().iter().any(|f| *f == door_entity || *f == *cweampuf) {
                    door.is_active = false;
                    interaction_state.set(InteractionState::NotReady);

                    return;
                }
            }
        }
    
        if let CollisionEvent::Started(h1, h2, _flags) = event {
            for (door_entity, mut door) in doors.iter_mut() {
                if h1.entities().iter().any(|f| *f == door_entity || *f == *cweampuf) && 
                   h2.entities().iter().any(|f| *f == door_entity || *f == *cweampuf) {
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
    cweampuff: Single<&Cweampuf, With<Cweampuf>>,
    mut commands: Commands,
    current_level_layout: Query<Entity, With<LevelLayout>>,
    mut transition_events: EventWriter<LevelTransitionEvent>,
    mut transition_state: ResMut<NextState<TransitionState>>,
) {
    if !keyboard_input.just_pressed(KeyCode::KeyE) {
        return;
    }

    for door in doors.iter().find(|f| f.is_active).iter() {
        manually_transition_to_level(&current_level_layout, &mut transition_events, &mut transition_state, &cweampuff, &mut commands, door.transition_to_level, door.safe_position);
    }
}