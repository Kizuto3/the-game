use bevy::{prelude::*, render::camera::ScalingMode};

use crate::{level::LevelLayout, Cweampuff};

const CAMERA_TRANSFORM: Vec3 = Vec3::new(0.0, 3.0, 0.0);
const CAMERA_DECAY_RATE: f32 = 10.;

#[derive(Component)]
pub struct CameraUpDownMovable {
    look_up_down_invoke_threshold: f32,
    pub look_up_down_duration: f32,
    camera_offset: f32,
}

pub fn spawn_camera (
    mut commands: Commands,
) {
    let mut projection = OrthographicProjection::default_2d();
    projection.scaling_mode = ScalingMode::Fixed { width: 1920., height: 1080. };

    // Camera
    commands.spawn((
        Camera2d,
        CameraUpDownMovable { look_up_down_duration: 0., look_up_down_invoke_threshold: 0.3, camera_offset: 360. },
        Camera {
            ..default()
        },
        Projection::Orthographic(projection)
    ));
}

pub fn cweampuff_camera_adjustment(
    keyboard_input: Res<ButtonInput<KeyCode>>, 
    cweampuff: Single<&Transform, (With<Cweampuff>, Without<Camera2d>)>,
    mut camera: Single<(&mut Transform, &mut CameraUpDownMovable), With<Camera2d>>,
    level_layout_query: Query<&LevelLayout, With<LevelLayout>>,
    time: Res<Time>,
) {
    let (camera_transform, camera_movable) = &mut *camera;

    let time_passed = time.delta_secs();

    if keyboard_input.any_just_released([KeyCode::ArrowUp, KeyCode::KeyW, KeyCode::ArrowDown, KeyCode::KeyS]) {
        camera_movable.look_up_down_duration = 0.;
    }

    let mut direction = 1.;

    if keyboard_input.any_pressed([KeyCode::ArrowUp, KeyCode::KeyW, KeyCode::ArrowDown, KeyCode::KeyS]) {
        if camera_movable.look_up_down_duration < camera_movable.look_up_down_invoke_threshold {
            camera_movable.look_up_down_duration += time_passed;
        }

        if keyboard_input.any_pressed([KeyCode::ArrowDown, KeyCode::KeyS]) {
            direction = -1.;
        }
    }

    let mut offset = Vec3::new(0., 0., 0.);

    if camera_movable.look_up_down_duration >= camera_movable.look_up_down_invoke_threshold {
        offset.y = camera_movable.camera_offset * direction;
    }

    let new_camera_position = get_adjusted_camera_position(&cweampuff, &level_layout_query, Some(&offset));

    camera_transform.translation.smooth_nudge(&new_camera_position, CAMERA_DECAY_RATE, time_passed);
}

pub fn get_adjusted_camera_position(
    cweampuff_transform: &Transform,
    level_layout_query: &Query<&LevelLayout, With<LevelLayout>>,
    offset: Option<&Vec3>
) -> Vec3 {
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

    let mut new_camera_position = cweampuff_transform.translation + CAMERA_TRANSFORM;

    if let Some(offset) = offset {
        new_camera_position += offset;
    }

    new_camera_position.x = new_camera_position.x.clamp(min_x, max_x);
    new_camera_position.y = new_camera_position.y.clamp(min_y, max_y);

    new_camera_position
}