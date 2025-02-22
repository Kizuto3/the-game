pub mod conversation_state;
pub mod conversation_entry;
pub mod dialog_state;

use bevy::{ecs::observer::TriggerTargets, prelude::*, ui::widget::NodeImageMode};
use bevy_rapier2d::prelude::CollisionEvent;
use conversation_entry::{ConversationEntry, ConversationPosition};
use conversation_state::ConversationState;
use dialog_state::DialogState;

use crate::{fade_in_fade_out::FADE_DELTA, interactable::{interaction_state::InteractionState, Interactable}, level::level_layout::FloorInfo, Cweampuf};

#[derive(Component)]
pub struct DialogNode;

#[derive(Component)]
pub struct DialogTextNode;

#[derive(Component)]
pub struct DialogText;

#[derive(Component)]
pub struct LeftCharacterImageNode;

#[derive(Component)]
pub struct RightCharacterImageNode;


#[derive(Component, Clone)]
pub struct NPC {
    pub floor_info: FloorInfo,
    pub is_active: bool,
    pub conversation: Vec<ConversationEntry>,
    pub current_conversation_index: usize,
    pub after_conversation_func: fn(&mut Cweampuf)
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
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                ..default()
            },
            PickingBehavior::IGNORE,
            BackgroundColor(Color::Srgba(Srgba { red: 0.1, green: 0.1, blue: 0.1, alpha: 0.95 })),
            DialogNode
        ))
        .with_children(|parent| {

            // DIALOG BOX
            parent
                .spawn((
                    Node {
                        width: Val::Percent(40.0),
                        height: Val::Percent(35.0),
                        top: Val::Percent(65.0),
                        left: Val::Percent(0.),
                        position_type: PositionType::Absolute,
                        border: UiRect::all(Val::Px(10.)),
                        justify_content: JustifyContent::Start,
                        padding: UiRect::all(Val::Px(10.)),
                        ..default()
                    },
                    DialogTextNode,
                    BorderColor(Color::Srgba(Srgba { red: 0.75, green: 0.75, blue: 0.75, alpha: 1.0 })),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new(""),
                        TextFont {
                            font: asset_server.load("fonts/Shadows Into Light.ttf"),
                            font_size: 50.0,
                            ..default()
                        },
                        DialogText,
                        TextColor(Color::srgb(0.9, 0.9, 0.9)),
                        TextLayout::new(JustifyText::Left, LineBreak::WordBoundary)
                    ));
                });

            // LEFT CHARACTER IMAGE
            parent.spawn((
                    ImageNode::default()
                        .with_color(Color::Srgba(Srgba::new(1.0, 1.0, 1.0, 0.0)))
                        .with_mode(NodeImageMode::Auto),
                    Node {
                        width: Val::Percent(35.0),
                        height: Val::Percent(65.0),
                        top: Val::Percent(35.0),
                        left: Val::Percent(0.),
                        justify_content: JustifyContent::Start,
                        position_type: PositionType::Absolute,
                        ..default()
                    },
                    LeftCharacterImageNode,
                ));
            
            // RIGHT CHARACTER IMAGE
            parent.spawn((
                    ImageNode::default()
                        .with_flip_x()
                        .with_color(Color::Srgba(Srgba::new(1.0, 1.0, 1.0, 0.0)))
                        .with_mode(NodeImageMode::Auto),
                    Node {
                        width: Val::Percent(35.0),
                        height: Val::Percent(65.0),
                        top: Val::Percent(35.0),
                        left: Val::Percent(65.),
                        position_type: PositionType::Absolute,
                        justify_content: JustifyContent::Start,
                        ..default()
                    },
                    RightCharacterImageNode,
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
    mut cweampuff: Single<&mut Cweampuf, With<Cweampuf>>,
    mut conversation_state: ResMut<NextState<ConversationState>>,
    mut text_query: Query<&mut Text, With<DialogText>>,
    mut text_node_query: Query<&mut Node, With<DialogTextNode>>,
    mut left_npc_image: Single<&mut ImageNode, (With<LeftCharacterImageNode>, Without<RightCharacterImageNode>)>,
    mut right_npc_image: Single<&mut ImageNode, (With<RightCharacterImageNode>, Without<LeftCharacterImageNode>)>,
    mut npcs_query: Query<&mut NPC, With<NPC>>,
    mut next_dialog_state: ResMut<NextState<DialogState>>,
    asset_server: Res<AssetServer>
) {
    if let Some(mut npc) = npcs_query.iter_mut().find(|f| f.is_active) {
        let continue_conversation = keyboard_input.any_just_pressed([KeyCode::KeyE, KeyCode::Space, KeyCode::Enter]);

        if !continue_conversation && npc.current_conversation_index != 0 {
            return;
        }

        if continue_conversation {
            npc.current_conversation_index += 1;
        }

        {
            let current_conversation_info = match npc.conversation.get(npc.current_conversation_index) {
                Some(info) => info,
                None => {
                    conversation_state.set(ConversationState::Finished);
                    npc.current_conversation_index = 0;
                    next_dialog_state.set(DialogState::None);
                    (npc.after_conversation_func)(&mut cweampuff);

                    return;
                }
            };
        
            for mut text in text_query.iter_mut() {
                if npc.current_conversation_index != 0 {
                    **text = String::new();
                }
            }

            let npc_image_path = format!("npcs/{}/{}.png", current_conversation_info.npc_name, current_conversation_info.emotion);

            match current_conversation_info.position {
                ConversationPosition::Left => {
                    let new_image_handle = asset_server.load(npc_image_path);
                    
                    if new_image_handle != left_npc_image.image {
                        left_npc_image.color.set_alpha(0.);
                        left_npc_image.image = new_image_handle;
                    }

                    next_dialog_state.set(DialogState::LeftCharacterTalking);

                    for mut node in text_node_query.iter_mut() {
                        node.left = Val::Percent(35.);
                    }
                },
                ConversationPosition::Right => {
                    let new_image_handle = asset_server.load(npc_image_path);
                    
                    if new_image_handle != right_npc_image.image {
                        right_npc_image.color.set_alpha(0.);
                        right_npc_image.image = new_image_handle;
                    }
                    
                    next_dialog_state.set(DialogState::RightCharacterTalking);

                    for mut node in text_node_query.iter_mut() {
                        node.left = Val::Percent(25.);
                    }
                }
            }
        }
    }    
}

pub fn left_character_talking(
    mut left_npc_image: Single<&mut ImageNode, (With<LeftCharacterImageNode>, Without<RightCharacterImageNode>)>,
    mut right_npc_image: Single<&mut ImageNode, (With<RightCharacterImageNode>, Without<LeftCharacterImageNode>)>,
    time: Res<Time>,
) {
    let left_npc_alpha = left_npc_image.color.alpha();

    if left_npc_alpha < 1.0 {
        left_npc_image.color.set_alpha(left_npc_alpha + time.delta_secs() * FADE_DELTA);
    }

    let right_npc_alpha = right_npc_image.color.alpha();

    if right_npc_alpha > 0.0 {
        right_npc_image.color.set_alpha(right_npc_alpha - time.delta_secs() * FADE_DELTA * 2.);
    }
}

pub fn right_character_talking(
    mut left_npc_image: Single<&mut ImageNode, (With<LeftCharacterImageNode>, Without<RightCharacterImageNode>)>,
    mut right_npc_image: Single<&mut ImageNode, (With<RightCharacterImageNode>, Without<LeftCharacterImageNode>)>,
    time: Res<Time>,
) {
    let right_npc_alpha = right_npc_image.color.alpha();

    if right_npc_alpha < 1.0 {
        right_npc_image.color.set_alpha(right_npc_alpha + time.delta_secs() * FADE_DELTA);
    }

    let left_npc_alpha = left_npc_image.color.alpha();

    if left_npc_alpha > 0.0 {
        left_npc_image.color.set_alpha(left_npc_alpha - time.delta_secs() * FADE_DELTA * 2.);
    }
}

pub fn dialog_box_text_writer(
    mut text_query: Query<&mut Text, With<DialogText>>,
    mut npcs_query: Query<&mut NPC, With<NPC>>,
    time: Res<Time>,
) {
    if let Some(npc) = npcs_query.iter_mut().find(|f| f.is_active) {
        let current_conversation_info = match npc.conversation.get(npc.current_conversation_index) {
            Some(info) => info,
            None => return,
        };
    
        for mut text in text_query.iter_mut() {
            let mut conversation_text = current_conversation_info.text.to_string();

            let text_length = text.chars().count().max(1);
            let chars_to_display = (time.delta_secs().ceil() as usize + text_length).min(conversation_text.chars().count());

            let _ = conversation_text.split_off(chars_to_display);

            if conversation_text.chars().count() != text_length {
                **text = conversation_text;
            }
        }
    }
}