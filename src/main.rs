mod movement;
mod app_states;
mod main_menu;

use app_states::AppState;
use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_rapier2d::{plugin::{NoUserData, RapierPhysicsPlugin}, prelude::{ActiveEvents, Collider, ExternalForce, Friction, GravityScale, LockedAxes, RigidBody, Velocity}};
use main_menu::{button_interactions_handler, button_visuals_handler, spawn_main_menu_buttons};
use movement::*;

const FLOOR_STARTING_POSITION: Vec3 = Vec3::new(0.0, 0.0, 1.0);
const LEFT_WALL_STARTING_POSITION: Vec3 = Vec3::new(-600.0, -200.0, 1.0);
const RIGHT_WALL_STARTING_POSITION: Vec3 = Vec3::new(600.0, -200.0, 1.0);

const CWEAMPUF_COLOR: Color = Color::srgb(1.0, 0.5, 0.5);
// We set the z-value of the ball to 1 so it renders on top in the case of overlapping sprites.
const CWEAMPUF_STARTING_POSITION: Vec3 = Vec3::new(0.0, 150.0, 1.0);
pub const CWEAMPUF_DIAMETER: f32 = 30.;
const CAMERA_TRANSFORM: Vec3 = Vec3::new(0.0, 3.0, 0.0);
const CAMERA_DECAY_RATE: f32 = 10.;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
       .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(125.0));

    app.init_state::<AppState>();

    app.add_systems(OnEnter(AppState::MainMenu), spawn_main_menu_buttons)
        .add_systems(Update, (
            button_visuals_handler,
            button_interactions_handler
        ).run_if(in_state(AppState::MainMenu)))
        .add_systems(OnExit(AppState::MainMenu), clean_resources)
        .add_systems(OnEnter(AppState::InGame), (
            setup_cweampuf, 
            setup_floor
        ))
        .add_systems(Update, (
            cweampuf_move,
            cweampuf_jump,
            cweampuf_dash,
            cweampuf_camera_adjustment
        ).run_if(in_state(AppState::InGame)))
        
        .add_systems(FixedUpdate, (
            dash_reset,
            jump_reset,
            velocity_limiter,
            stunlock_reset,
        ).run_if(in_state(AppState::InGame)))
        .run();
}

fn setup_cweampuf(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    //asset_server: Res<AssetServer>,
) {
    let mut projection = OrthographicProjection::default_2d();
    projection.scaling_mode = ScalingMode::AutoMin { min_width: 1920., min_height: 1080. };

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
        Jumper { jump_impulse: 800., is_jump_available: true, is_jumping: false, is_next_jump_doublejump: false, is_double_jump_available: true},
        Dasher { dash_impulse: 650., dash_cooldown: 1., time_passed_since_dash: 0. },
        LockedAxes::ROTATION_LOCKED,
        Movable { facing_right: true, hugging_wall: false, is_stunlocked: false, stun_duration: 0.2, time_passed_since_stun: 0. },
        ExternalForce::default(),
    ));
    
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

fn setup_floor (
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    //asset_server: Res<AssetServer>,
) {
    let floor_width = 1400.;
    let floor_height = 100.;

    commands
        .spawn(RigidBody::Fixed)
        .insert((
            Mesh2d(meshes.add(Rectangle::new(floor_width, floor_height))),
            MeshMaterial2d(materials.add(CWEAMPUF_COLOR)),
            Transform::from_translation(FLOOR_STARTING_POSITION)
        ))
        .insert(Collider::cuboid(floor_width / 2.0, floor_height / 2.0))
        .insert(Friction::coefficient(0.7))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(FloorCollider {entity_index: 0});

    let left_wall_width = 100.;
    let left_wall_height = 2000.;

    commands
        .spawn(RigidBody::Fixed)
        .insert((
            Mesh2d(meshes.add(Rectangle::new(left_wall_width, left_wall_height))),
            MeshMaterial2d(materials.add(CWEAMPUF_COLOR)),
            Transform::from_translation(LEFT_WALL_STARTING_POSITION)
        ))
        .insert(Collider::cuboid(left_wall_width / 2.0, left_wall_height / 2.0))
        .insert(Friction::coefficient(0.7))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(FloorCollider {entity_index: 0});

    commands
        .spawn(RigidBody::Fixed)
        .insert((
            Mesh2d(meshes.add(Rectangle::new(left_wall_width, left_wall_height))),
            MeshMaterial2d(materials.add(CWEAMPUF_COLOR)),
            Transform::from_translation(RIGHT_WALL_STARTING_POSITION)
        ))
        .insert(Collider::cuboid(left_wall_width / 2.0, left_wall_height / 2.0))
        .insert(Friction::coefficient(0.7))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(FloorCollider {entity_index: 0});
}

fn cweampuf_camera_adjustment(
    keyboard_input: Res<ButtonInput<KeyCode>>, 
    cweampuf: Single<&Transform, (With<Cweampuf>, Without<Camera2d>)>,
    mut camera: Single<(&mut Transform, &mut CameraUpDownMovalbe), With<Camera2d>>,
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

    camera_transform.translation.smooth_nudge(&(cweampuf.translation + offset + CAMERA_TRANSFORM), CAMERA_DECAY_RATE, time.delta_secs());
}

fn clean_resources(
    mut commands: Commands, 
    query: Query<Entity, (With<Node>, Without<Camera2d>)>,
    camera: Single<Entity, With<Camera2d>>
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    commands.entity(*camera).despawn();
}

#[derive(Component)]
struct Cweampuf;

#[derive(Component)]
struct FloorCollider {
    entity_index: u32
}

#[derive(Component)]
struct CameraUpDownMovalbe {
    look_up_down_invoke_threshold: f32,
    look_up_down_duration: f32,
    camera_offset: f32,
}