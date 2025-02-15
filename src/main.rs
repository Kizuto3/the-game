mod movement;
mod app_states;
mod main_menu;
mod cutscene;
mod level;

use app_states::AppState;
use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_rapier2d::{plugin::{NoUserData, RapierPhysicsPlugin}, prelude::{Collider, ExternalForce, Friction, GravityScale, LockedAxes, RigidBody, Velocity}};
use cutscene::{cutscene_event_reader, cutscene_player, spawn_cutscene_resources, CutsceneEvent};
use level::{despawn_current_level, level_layout::FloorCollider, level_transition_collision_reader, level_transition_event_reader, spawn_new_level, transition_states::TransitionState, LevelLayout, LevelTransitionEvent};
use main_menu::{button_interactions_handler, button_visuals_handler, spawn_main_menu_buttons};
use movement::*;

const CWEAMPUF_COLOR: Color = Color::srgb(1.0, 0.5, 0.5);
// We set the z-value of the ball to 1 so it renders on top in the case of overlapping sprites.
const CWEAMPUF_STARTING_POSITION: Vec3 = Vec3::new(0.0, 150.0, 1.0);
const CWEAMPUF_JUMP_IMPULSE: f32 = 800.;
const CWEAMPUF_DASH_IMPULSE: f32 = 650.;
pub const CWEAMPUF_DIAMETER: f32 = 30.;
const CAMERA_TRANSFORM: Vec3 = Vec3::new(0.0, 3.0, 0.0);
const CAMERA_DECAY_RATE: f32 = 10.;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
       .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(125.0));

    app.init_state::<AppState>();
    app.init_state::<TransitionState>();

    app.add_event::<CutsceneEvent>();
    app.add_event::<LevelTransitionEvent>();

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
        .add_systems(FixedUpdate, (cutscene_player).run_if(in_state(AppState::Cutscene)))
        .add_systems(OnExit(AppState::Cutscene), clean_nodes)

    // LEVEL TRANSITION SYSTEMS
        .add_systems(OnEnter(TransitionState::Started), (despawn_current_level, spawn_new_level).chain())
        .add_systems(FixedUpdate, (level_transition_event_reader).run_if(in_state(TransitionState::Started)))

    // GAMEPLAY SYSTEMS
        .add_systems(OnEnter(AppState::InGame), setup_cweampuf)
        .add_systems(Update, (
            cweampuf_dash,
            cweampuf_jump,
            cweampuf_move,
            cweampuf_camera_adjustment
        ).chain().run_if(in_state(AppState::InGame)).run_if(in_state(TransitionState::Finished)))
        .add_systems(FixedUpdate, (
            dash_reset,
            jump_reset,
            velocity_limiter,
            stunlock_reset,
            level_transition_collision_reader
        ).run_if(in_state(AppState::InGame)).run_if(in_state(TransitionState::Finished)))
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

fn cweampuf_camera_adjustment(
    keyboard_input: Res<ButtonInput<KeyCode>>, 
    cweampuf: Single<&Transform, (With<Cweampuf>, Without<Camera2d>)>,
    mut camera: Single<(&mut Transform, &mut CameraUpDownMovalbe), With<Camera2d>>,
    level_layout_query: Query<&LevelLayout, With<LevelLayout>>,
    time: Res<Time>,
) {
    let (camera_transform, camera_movable) = &mut *camera;

    if keyboard_input.just_released(KeyCode::ArrowUp) || keyboard_input.just_released(KeyCode::ArrowDown) {
        camera_movable.look_up_down_duration = 0.;
    }

    let mut direction = 1.;

    if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::ArrowDown) {
        if camera_movable.look_up_down_duration < camera_movable.look_up_down_invoke_threshold {
            camera_movable.look_up_down_duration += time.delta_secs();
        }

        if keyboard_input.pressed(KeyCode::ArrowDown) {
            direction = -1.;
        }
    }

    let mut offset = Vec3::new(0., 0., 0.);

    if camera_movable.look_up_down_duration >= camera_movable.look_up_down_invoke_threshold {
        offset.y = camera_movable.camera_offset * direction;
    }

    let mut min_x = f32::MAX;
    let mut min_y = f32::MAX;
    let mut max_x = f32::MIN;
    let mut max_y = f32::MIN;

    for level_layout in level_layout_query.iter() {
        for layout in level_layout.floor_layout.iter() {
            if layout.position.x > max_x {
                max_x = layout.position.x;
            }
            if layout.position.y > max_y {
                max_y = layout.position.y;
            }
            if layout.position.x < min_x {
                min_x = layout.position.x;
            }
            if layout.position.y < min_y {
                min_y = layout.position.y;
            }
        }
    }

    max_x -= 960.;
    max_y -= 540.;
    min_x += 960.;
    min_y += 540.;

    let mut new_camera_position = cweampuf.translation + offset + CAMERA_TRANSFORM;
    new_camera_position.x = new_camera_position.x.clamp(min_x, max_x);
    new_camera_position.y = new_camera_position.y.clamp(min_y, max_y);

    camera_transform.translation.smooth_nudge(&new_camera_position, CAMERA_DECAY_RATE, time.delta_secs());
}

fn spawn_camera (
    mut commands: Commands
) {
    let mut projection = OrthographicProjection::default_2d();
    projection.scaling_mode = ScalingMode::AutoMin { min_width: 1920., min_height: 1080. };

    // Camera
    commands.spawn((
        Camera2d,
        Camera{
            ..default()
        },
        Transform::from_translation(CAMERA_TRANSFORM),
        CameraUpDownMovalbe { look_up_down_duration: 0., look_up_down_invoke_threshold: 0.3, camera_offset: 360. },
        projection
    ));
}

fn clean_nodes(
    mut commands: Commands, 
    query: Query<Entity, (With<Node>, Without<Camera2d>)>
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

#[derive(Component)]
struct Cweampuf;


#[derive(Component)]
struct CameraUpDownMovalbe {
    look_up_down_invoke_threshold: f32,
    look_up_down_duration: f32,
    camera_offset: f32,
}