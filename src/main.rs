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
use bevy_rapier2d::{plugin::{NoUserData, RapierPhysicsPlugin}, prelude::{Collider, ExternalForce, Friction, GravityScale, LockedAxes, RigidBody, Velocity}};
use camera::{cweampuf_camera_adjustment, spawn_camera};
use cutscene::{cutscene_event_reader, cutscene_input_reader, cutscene_player, spawn_cutscene_resources, CutsceneEvent};
use fade_in_fade_out::{despawn_fade_in_fade_out_node, fade_in, fade_out, set_fade_in_state, set_fade_out_state, spawn_fade_in_fade_out_node, FadeInFadeOutNode, FadeState};
use interactable::{despawn_interaction_prompt, interaction_state::InteractionState, spawn_interaction_prompt};
use level::{despawn_current_level, door::{door_start_interaction_input_reader, interactable_door_collision_reader}, level_layout::FloorCollider, level_transition_collision_reader, spawn_new_level, transition_states::TransitionState};
use main_menu::{button_interactions_handler, button_visuals_handler, spawn_main_menu_buttons};
use movement::*;
use npc::{conversation_input_reader, conversation_state::ConversationState, despawn_conversation_resources, npc_collision_reader, npc_start_interaction_input_reader, spawn_conversation_resources};

const CWEAMPUF_COLOR: Color = Color::srgb(1.0, 0.5, 0.5);
// We set the z-value of the ball to 1 so it renders on top in the case of overlapping sprites.
const CWEAMPUF_STARTING_POSITION: Vec3 = Vec3::new(0.0, 150.0, 1.0);
const CWEAMPUF_JUMP_IMPULSE: f32 = 800.;
const CWEAMPUF_DASH_IMPULSE: f32 = 650.;
pub const CWEAMPUF_DIAMETER: f32 = 30.;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
       .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(125.0));

    app.init_state::<AppState>();
    app.init_state::<TransitionState>();
    app.init_state::<InteractionState>();
    app.init_state::<ConversationState>();
    app.init_state::<FadeState>();

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
        .add_systems(FixedUpdate, cutscene_event_reader)
        .add_systems(Update, (cutscene_input_reader).run_if(in_state(AppState::Cutscene)))
        .add_systems(OnEnter(FadeState::FadeInFinished), (cutscene_player).run_if(in_state(AppState::Cutscene)))
        .add_systems(OnExit(AppState::Cutscene), clean_nodes)

    // FADE IN FADE OUT SYSTEMS
        .add_systems(OnEnter(FadeState::None), despawn_fade_in_fade_out_node)
        .add_systems(OnExit(FadeState::None), spawn_fade_in_fade_out_node)
        .add_systems(FixedUpdate, (fade_in).run_if(in_state(FadeState::FadeIn)))
        .add_systems(FixedUpdate, (fade_out).run_if(in_state(FadeState::FadeOut)))

    // LEVEL TRANSITION SYSTEMS
        .add_systems(OnEnter(TransitionState::Started), set_fade_in_state)
        .add_systems(OnEnter(TransitionState::Finished), set_fade_out_state)
        .add_systems(OnEnter(FadeState::FadeInFinished), (despawn_current_level, spawn_new_level).run_if(in_state(TransitionState::Started)).chain())

    // INTERACTION SYSTEMS
        .add_systems(OnEnter(InteractionState::Ready), spawn_interaction_prompt)
        .add_systems(Update, (
            npc_start_interaction_input_reader, 
            door_start_interaction_input_reader
        ).run_if(in_state(InteractionState::Ready)))
        .add_systems(OnExit(InteractionState::Ready), despawn_interaction_prompt)
        .add_systems(OnEnter(ConversationState::Started), spawn_conversation_resources)
        .add_systems(Update,(conversation_input_reader).run_if(in_state(ConversationState::Started)))
        .add_systems(OnExit(ConversationState::Started), despawn_conversation_resources)

    // GAMEPLAY SYSTEMS
        .add_systems(OnEnter(AppState::InGame), setup_cweampuf)
        .add_systems(Update, (
            cweampuf_dash,
            cweampuf_jump,
            cweampuf_move,
            cweampuf_camera_adjustment
        ).chain().run_if(in_state(AppState::InGame)).run_if(in_state(TransitionState::Finished)).run_if(in_state(ConversationState::Finished)).run_if(in_state(FadeState::None)))
        .add_systems(FixedUpdate, (
            dash_reset,
            jump_reset,
            velocity_limiter,
            stunlock_reset,
            level_transition_collision_reader,
            npc_collision_reader,
            interactable_door_collision_reader
        ).run_if(in_state(AppState::InGame)).run_if(in_state(TransitionState::Finished)).run_if(in_state(ConversationState::Finished)).run_if(in_state(FadeState::None)))
        .run();
}

fn setup_cweampuf(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    //asset_server: Res<AssetServer>,
) {
    // Cweampuf
    commands.spawn((
        RigidBody::Dynamic,
        Mesh2d(meshes.add(Circle::default())),
        MeshMaterial2d(materials.add(CWEAMPUF_COLOR)),
        Transform::from_translation(CWEAMPUF_STARTING_POSITION).with_scale(Vec2::splat(CWEAMPUF_DIAMETER).extend(1.)),
        Cweampuf,
        Velocity {
            linvel: Vec2::new(0.0, 0.0),
            angvel: 0.,
        },
        GravityScale(1.5),
        Friction::coefficient(0.7),
        Collider::ball(0.5),
        Jumper { jump_impulse: CWEAMPUF_JUMP_IMPULSE, is_jump_available: true, is_jumping: false, is_next_jump_doublejump: false, is_double_jump_available: true},
        Dasher { dash_impulse: CWEAMPUF_DASH_IMPULSE, dash_cooldown: 1., time_passed_since_dash: 0. },
        LockedAxes::ROTATION_LOCKED,
        Movable { facing_right: true, hugging_wall: false, is_stunlocked: false, stun_duration: 0.2, time_passed_since_stun: 0. },
        ExternalForce::default(),
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

#[derive(Component)]
struct Cweampuf;