mod movement;
mod app_states;
mod main_menu;
mod cutscene;
mod level;
mod camera;
mod interactable;
mod npc;
mod fade_in_fade_out;

use app_states::AppState;
use bevy::prelude::*;
use bevy_rapier2d::{plugin::{NoUserData, RapierPhysicsPlugin}, prelude::{Collider, Friction, GravityScale, LockedAxes, RigidBody, Velocity}};
use camera::{cweampuff_camera_adjustment, spawn_camera};
use cutscene::{cutscene_event_reader, cutscene_input_reader, cutscene_player, despawn_cutscene_resources, spawn_cutscene_resources, wait_for_resources_to_load, CutsceneEvent};
use fade_in_fade_out::{despawn_fade_in_fade_out_node, fade_in, fade_out, set_fade_in_state, set_fade_out_state, spawn_fade_in_fade_out_node, FadeInFadeOutNode, FadeState};
use interactable::{despawn_interaction_prompt, interaction_state::InteractionState, spawn_interaction_prompt};
use level::{despawn_current_level, door::{door_start_interaction_input_reader, interactable_door_collision_reader}, floor_modification::{gravity_inverter_collision_reader, jump_pad_collision_reader}, level_layout::FloorCollider, level_transition_collision_reader, progression::Progression, spawn_new_level, transition_states::TransitionState};
use main_menu::{button_interactions_handler, button_visuals_handler, spawn_main_menu_buttons};
use movement::*;
use npc::{conversation_input_reader, conversation_state::ConversationState, despawn_conversation_resources, dialog_box_text_writer, dialog_state::DialogState, left_character_talking, npc_collision_reader, npc_start_interaction_input_reader, right_character_talking, spawn_conversation_resources};

// We set the z-value of Cweampuff to 2 so it renders on top in the case of overlapping sprites.
pub const CWEAMPUFF_Z_INDEX: f32 = 2.0;
const CWEAMPUFF_STARTING_POSITION: Vec3 = Vec3::new(0.0, 550.0, CWEAMPUFF_Z_INDEX);
const CWEAMPUFF_JUMP_IMPULSE: f32 = 800.;
const CWEAMPUFF_DASH_IMPULSE: f32 = 650.;
pub const CWEAMPUFF_DIAMETER: f32 = 30.;
pub const CWEAMPUFF_GRAVITY_SCALE: f32 = 1.5;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
       .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(125.0));

    app.init_state::<AppState>();
    app.init_state::<TransitionState>();
    app.init_state::<InteractionState>();
    app.init_state::<ConversationState>();
    app.init_state::<FadeState>();
    app.init_state::<DialogState>();

    app.add_event::<CutsceneEvent>();

    app.add_systems(Startup, spawn_camera)

    // MAIN MENU SYSTEMS
        .add_systems(OnEnter(AppState::MainMenu), spawn_main_menu_buttons)
        .add_systems(Update, (
            button_visuals_handler,
            button_interactions_handler
        ).run_if(in_state(AppState::MainMenu)))
        .add_systems(OnExit(AppState::MainMenu), clean_nodes)

    // CUTSCENE SYSTEMS
        .add_systems(OnEnter(AppState::Cutscene), spawn_cutscene_resources)
        .add_systems(Update, cutscene_event_reader)
        .add_systems(Update, cutscene_input_reader.run_if(in_state(AppState::Cutscene)))
        .add_systems(FixedUpdate, wait_for_resources_to_load.run_if(in_state(AppState::Cutscene)).run_if(in_state(FadeState::FadeInFinished)))
        .add_systems(OnEnter(FadeState::FadeInFinished), (despawn_current_level, cutscene_player).chain().run_if(in_state(AppState::Cutscene)))
        .add_systems(OnExit(AppState::Cutscene), despawn_cutscene_resources)

    // FADE IN FADE OUT SYSTEMS
        .add_systems(OnEnter(FadeState::None), despawn_fade_in_fade_out_node)
        .add_systems(OnExit(FadeState::None), spawn_fade_in_fade_out_node)
        .add_systems(FixedUpdate, fade_in.run_if(in_state(FadeState::FadeIn)))
        .add_systems(FixedUpdate, fade_out.run_if(in_state(FadeState::FadeOut)))

    // LEVEL TRANSITION SYSTEMS
        .add_systems(OnEnter(TransitionState::Started), set_fade_in_state)
        .add_systems(OnEnter(TransitionState::Finished), (set_fade_out_state, reset_abilities).chain())
        .add_systems(OnEnter(FadeState::FadeInFinished), (despawn_current_level, spawn_new_level).run_if(in_state(TransitionState::Started)).chain())

    // INTERACTION SYSTEMS
        .add_systems(OnEnter(InteractionState::Ready), spawn_interaction_prompt)
        .add_systems(Update, (
            npc_start_interaction_input_reader, 
            door_start_interaction_input_reader
        ).run_if(in_state(InteractionState::Ready)))
        .add_systems(OnExit(InteractionState::Ready), despawn_interaction_prompt)
        .add_systems(OnEnter(ConversationState::Started), spawn_conversation_resources)
        .add_systems(Update,conversation_input_reader.run_if(in_state(ConversationState::Started)))
        .add_systems(FixedUpdate, left_character_talking.run_if(in_state(ConversationState::Started)).run_if(in_state(DialogState::LeftCharacterTalking)))
        .add_systems(FixedUpdate, right_character_talking.run_if(in_state(ConversationState::Started)).run_if(in_state(DialogState::RightCharacterTalking)))
        .add_systems(FixedUpdate, dialog_box_text_writer.run_if(in_state(ConversationState::Started)))
        .add_systems(OnExit(ConversationState::Started), despawn_conversation_resources)

    // GAMEPLAY SYSTEMS
        .add_systems(OnEnter(AppState::InGame), setup_cweampuff)
        .add_systems(Update, (
            cweampuff_dash,
            cweampuff_jump,
            cweampuff_move,
            cweampuff_camera_adjustment,
            level_transition_collision_reader
        ).chain().run_if(in_state(AppState::InGame)).run_if(in_state(TransitionState::Finished)).run_if(in_state(ConversationState::Finished)).run_if(in_state(FadeState::None)))
        .add_systems(FixedUpdate, (
            dash_reset,
            jump_reset,
            coyote_jump_buffer_monitor,
            velocity_limiter,
            stunlock_reset,
            cweampuff_asset_direction_monitor,
            npc_collision_reader,
            interactable_door_collision_reader,
            jump_pad_collision_reader,
            gravity_inverter_collision_reader
        ).run_if(in_state(AppState::InGame)).run_if(in_state(TransitionState::Finished)).run_if(in_state(ConversationState::Finished)).run_if(in_state(FadeState::None)))
        .run();
}

fn setup_cweampuff(
    mut commands: Commands,
    cweampuff_query: Query<&Cweampuff, With<Cweampuff>>,
    asset_server: Res<AssetServer>,
) {
    if !cweampuff_query.is_empty() {
        return;
    }

    let cweampuff_model_handle = asset_server.load("npcs/cweampuff/Model.png");
    
    // Cweampuff
    commands.spawn((
        RigidBody::Dynamic,
        Transform::from_translation(CWEAMPUFF_STARTING_POSITION).with_scale(Vec2::splat(CWEAMPUFF_DIAMETER).extend(CWEAMPUFF_Z_INDEX)),
        Cweampuff { progression: Progression::None, has_double_jump: false, has_wall_jump: false, has_dash: false },
        Sprite {
            image: cweampuff_model_handle,
            anchor: bevy::sprite::Anchor::Center,
            custom_size: Some(Vec2::new(1.42, 1.)),
            image_mode: SpriteImageMode::Auto,
            ..default()
        },
        Velocity {
            linvel: Vec2::new(0.0, 0.0),
            angvel: 0.,
        },
        GravityScale(CWEAMPUFF_GRAVITY_SCALE),
        Friction::coefficient(0.7),
        Collider::ball(0.5),
        Jumper { jump_impulse: CWEAMPUFF_JUMP_IMPULSE, is_jump_available: true, is_jumping: false, is_next_jump_doublejump: false, coyote_jump_buffer_duration: 0.075, time_passed_since_stopped_touching_ground: None },
        Dasher { is_dash_available: false, dash_impulse: CWEAMPUFF_DASH_IMPULSE, dash_cooldown: 0.5, time_passed_since_dash: 0. },
        LockedAxes::ROTATION_LOCKED,
        Movable { is_upside_down: false, touching_ground: false, facing_right: true, hugging_left_wall: false, hugging_right_wall: false, is_stunlocked: false, stun_duration: 0.2, time_passed_since_stun: 0. },
    ));
}

fn clean_nodes(
    mut commands: Commands, 
    query: Query<Entity, (With<Node>, Without<Camera2d>, Without<FadeInFadeOutNode>)>
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

#[derive(Component, Clone, Copy)]
struct Cweampuff {
    progression: Progression,
    has_double_jump: bool,
    has_wall_jump: bool,
    has_dash: bool
}