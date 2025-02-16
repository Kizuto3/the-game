pub mod conversation_state;

use bevy::{ecs::observer::TriggerTargets, prelude::*};
use bevy_rapier2d::prelude::CollisionEvent;
use conversation_state::ConversationState;

use crate::{interactable::{interaction_state::InteractionState, Interactable}, level::level_layout::FloorInfo, Cweampuf};

#[derive(Component)]
pub struct DialogNode;

#[derive(Component, Clone, Copy)]
pub struct NPC {
    pub floor_info: FloorInfo,
    pub is_active: bool
}

pub fn npc_collision_reader(
    mut npcs: Query<(Entity, &mut NPC), (With<NPC>, With<Interactable>)>,
    cweampuf: Single<Entity, With<Cweampuf>>,
    mut contact_events: EventReader<CollisionEvent>,
    mut npc_interaction_state: ResMut<NextState<InteractionState>> 
) {
    for event in contact_events.read() {
        if let CollisionEvent::Stopped(h1, h2, _flags) = event {
            for (npc_entity, mut npc) in npcs.iter_mut() {
                if h1.entities().iter().any(|f| *f == npc_entity || *f == *cweampuf) && 
                   h2.entities().iter().any(|f| *f == npc_entity || *f == *cweampuf) {
                    npc.is_active = false;
                    npc_interaction_state.set(InteractionState::NotReady);

                    return;
                }
            }
        }
    
        if let CollisionEvent::Started(h1, h2, _flags) = event {
            for (npc_entity, mut npc) in npcs.iter_mut() {
                if h1.entities().iter().any(|f| *f == npc_entity || *f == *cweampuf) && 
                   h2.entities().iter().any(|f| *f == npc_entity || *f == *cweampuf) {
                    npc.is_active = true;
                    npc_interaction_state.set(InteractionState::Ready);

                    return;
                }
            }
        }
    }
}

pub fn npc_start_interaction_input_reader(
    keyboard_input: Res<ButtonInput<KeyCode>>, 
    npcs: Query<&NPC, (With<NPC>, With<Interactable>)>,
    mut dialog_state: ResMut<NextState<ConversationState>>,
    mut npc_interaction_state: ResMut<NextState<InteractionState>> 
) {
    if !keyboard_input.just_pressed(KeyCode::KeyE) {
        return;
    }

    for _npc in npcs.iter().find(|f| f.is_active).iter() {
        dialog_state.set(ConversationState::Started);
        npc_interaction_state.set(InteractionState::NotReady);
    }
}

pub fn spawn_conversation_resources(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(35.0),
                top: Val::Percent(65.0),
                left: Val::Percent(0.),
                border: UiRect::all(Val::Px(10.)),
                justify_content: JustifyContent::Start,
                ..default()
            },
            DialogNode,
            BorderColor(Color::Srgba(Srgba { red: 0.75, green: 0.75, blue: 0.75, alpha: 1.0 })),
            BackgroundColor(Color::Srgba(Srgba { red: 0.1, green: 0.1, blue: 0.1, alpha: 0.95 }))
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Text::new("Dialog box yrty ertjyh retjyh kertjyh kretj yhrejthy kejrthy krjetyh kerjtyh rkejtyh krejt yhrkejth ysdf gsdfuy gsduifgeuryg yuergt hbert jerbthjrtbrejthyb rejt bjhreb twekrjhtb fgbd gb sdhfg"),
                    TextFont {
                        font: asset_server.load("fonts\\Shadows Into Light.ttf"),
                        font_size: 33.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                    TextLayout::new(JustifyText::Left, LineBreak::WordBoundary)
                ));
        });
}

pub fn despawn_conversation_resources(
    mut commands: Commands,
    entities: Query<Entity, (With<DialogNode>, Without<Camera2d>)>
) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn conversation_input_reader(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut conversation_state: ResMut<NextState<ConversationState>>
) {
    if keyboard_input.any_just_pressed([KeyCode::KeyE, KeyCode::Space, KeyCode::Enter]) {
        conversation_state.set(ConversationState::Finished);
    }
}