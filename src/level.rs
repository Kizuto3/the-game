use bevy::audio::{PlaybackMode, Volume};
use bevy::{ecs::observer::TriggerTargets, prelude::*};
use bevy_rapier2d::prelude::*;
use level_bgm::LevelBGM;
use level_layout::aquwa_lair_layout::AquwaLairInfo;
use level_layout::cerber_lair_layout::CerberLairInfo;
use level_layout::cweamcat_house_layout::CweamcatHouseInfo;
use level_layout::factory_1_layout::Factory1Info;
use level_layout::factory_2_layout::Factory2Info;
use level_layout::factory_3_layout::Factory3Info;
use level_layout::factory_4_layout::Factory4Info;
use level_layout::factory_transition_layout::FactoryTransitionInfo;
use level_layout::hell_1_layout::Hell1Info;
use level_layout::hell_2_layout::Hell2Info;
use level_layout::hell_3_layout::Hell3Info;
use level_layout::hell_4_layout::Hell4Info;
use level_layout::neuro_lair_layout::NeuroLairInfo;
use level_layout::spaceship_1_layout::Spaceship1Info;
use level_layout::spaceship_2_layout::Spaceship2Info;
use level_layout::spaceship_3_layout::Spaceship3Info;
use level_layout::spaceship_4_layout::Spaceship4Info;
use level_layout::{DoorCollider, FloorAssetType, FloorInfo, FloorModification};
use level_layout::{cweamcat_lair_layout::CweamcatLairInfo, starting_room_layout::StartingRoomInfo, FloorCollider, TransitionCollider};
use transition_states::TransitionState;

use crate::CWEAMPUFF_GRAVITY_SCALE;
use crate::{camera::get_adjusted_camera_position, interactable::Interactable, npc::NPC, Cweampuff};
use crate::level::level_layout::LevelInfo;

pub mod transition_states;
pub mod level_layout;
pub mod door;
pub mod progression;
pub mod floor_modification;
pub mod level_bgm;

const TRANSITION_COLOR: Color = Color::srgb(0.5, 1.0, 0.5);
const DOOR_COLOR: Color = Color::srgb(0.9, 0.2, 0.9);
const JUMP_PAD_COLOR: Color = Color::srgb(0.2, 0.9, 0.9);
const GRAVITY_INVERTER_COLOR: Color = Color::srgb(0.1, 0.2, 0.2);
const TIME_TRIAL_COLOR: Color = Color::srgb(0.9, 0.2, 0.2);

#[derive(Component)]
pub struct BackgroundComponent;

#[derive(Clone, Copy)]
pub enum Level {
    StartingRoom(StartingRoomInfo),
    CweamcatLair(CweamcatLairInfo),
    CweamcatHouse(CweamcatHouseInfo),
    Hell1(Hell1Info),
    Hell2(Hell2Info),
    Hell3(Hell3Info),
    Hell4(Hell4Info),
    CerberLair(CerberLairInfo),
    Spaceship1(Spaceship1Info),
    Spaceship2(Spaceship2Info),
    Spaceship3(Spaceship3Info),
    Spaceship4(Spaceship4Info),
    AquwaLair(AquwaLairInfo),
    FactoryTransition(FactoryTransitionInfo),
    Factory1(Factory1Info),
    Factory2(Factory2Info),
    Factory3(Factory3Info),
    Factory4(Factory4Info),
    NeuroLair(NeuroLairInfo)
}

pub struct LevelTransitionInfo {
    pub transition_to_index: u32,
    pub transition_to_position: Option<Vec3>
}

#[derive(Component)]
pub struct LevelLayout {
    pub floor_layout: Box<[FloorInfo]>,
    pub transition_layout: Option<Box<[TransitionCollider]>>,
    pub npc_layout: Option<Box<[NPC]>>,
    pub door_layout: Option<Box<[DoorCollider]>>,
    pub floor_modifications: Option<Box<[FloorModification]>>,
    pub transition_info: LevelTransitionInfo,
    pub bgm: Option<&'static str>
}

pub fn despawn_current_level(
    mut commands: Commands,
    mut cweampuff: Query<&mut GravityScale, (With<Cweampuff>, Without<Camera2d>, Without<FloorCollider>)>,
    floor_query: Query<Entity, (With<FloorCollider>, Without<Camera2d>)>,
    transitions_query: Query<Entity, (With<Sensor>, Without<Camera2d>)>,
    interactable_query: Query<Entity, (With<Interactable>, Without<Camera2d>)>,
    background_query: Query<Entity, (With<BackgroundComponent>, Without<Camera2d>)>
) {
    for mut gravity in cweampuff.iter_mut() {
        gravity.0 = 0.;
    }
    for entity in transitions_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in floor_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in interactable_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in background_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn spawn_new_level(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut transition_state: ResMut<NextState<TransitionState>>,
    mut cweampuff: Single<(&mut Transform, &mut GravityScale), (With<Cweampuff>, Without<Camera2d>)>,
    mut camera: Single<&mut Transform, With<Camera2d>>,
    level_layout_query: Query<&LevelLayout, With<LevelLayout>>,
    bgm_query: Query<(Entity, &AudioPlayer), With<LevelBGM>>,
    asset_server: Res<AssetServer>,
) {
    for level_layout in level_layout_query.iter() {
        match level_layout.bgm {
            Some(bgm) => {
                if bgm_query.is_empty() {
                    let audio_handle = asset_server.load(format!("ost/{}.mp3", bgm));

                    let mut playback_settings = PlaybackSettings::default().with_volume(Volume::new(0.0));
                    playback_settings.mode = PlaybackMode::Loop;
                                        commands.spawn((
                        AudioPlayer::new(audio_handle),
                        LevelBGM,
                        playback_settings
                    ));
                }

                for (current_bgm_entity, current_bgm) in bgm_query.iter() {
                    let audio_handle = asset_server.load(format!("ost/{}.mp3", bgm));
                    
                    if current_bgm.0 != audio_handle {
                        commands.entity(current_bgm_entity).despawn_recursive();
    
                        let mut playback_settings = PlaybackSettings::default().with_volume(Volume::new(0.0));
                        playback_settings.mode = PlaybackMode::Loop;
                    
                        commands.spawn((
                            AudioPlayer::new(audio_handle),
                            LevelBGM,
                            playback_settings
                        ));
                    }
                }
            }
            None => {
                for (current_bgm_entity, _) in bgm_query.iter() {
                    commands.entity(current_bgm_entity).despawn_recursive();
                }
            }
        }

        let mut min_x = f32::MAX;
        let mut min_y = f32::MAX;
        let mut max_x = f32::MIN;
        let mut max_y = f32::MIN;

        for floor in &level_layout.floor_layout {
            if floor.position.x > max_x {
                max_x = floor.position.x;
            }
            if floor.position.y > max_y {
                max_y = floor.position.y;
            }
            if floor.position.x < min_x {
                min_x = floor.position.x;
            }
            if floor.position.y < min_y {
                min_y = floor.position.y;
            }

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
                        max_corner_scale: 1.0,
                        ..default()
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

        let background_image_handle = asset_server.load("forest.png");

        let width = (max_x - min_x).abs();
        let height = (max_y - min_y).abs();
        let x = (max_x + min_x) / 2.;
        let y = (max_y + min_y) / 2.;
        //TODO: Use TextureSlicer when background images are ready
        commands.spawn((
            Sprite {
                image: background_image_handle,
                anchor: bevy::sprite::Anchor::Center,
                custom_size: Some(Vec2::new(width.abs(), height.abs())),
                ..default()
            },
            BackgroundComponent,
            Transform::from_translation(Vec3::new(x, y, -10.0))
        ));

        if let Some(transitions) = &level_layout.transition_layout {
            for transition in transitions {
                commands
                    .spawn(*transition)
                    .insert((
                        Mesh2d(meshes.add(Rectangle::new(transition.floor_info.size.x, transition.floor_info.size.y))),
                        MeshMaterial2d(materials.add(TRANSITION_COLOR)),
                        Transform::from_translation(transition.floor_info.position)
                    ))
                    .insert(Collider::cuboid(transition.floor_info.size.x / 2.0, transition.floor_info.size.y / 2.0))
                    .insert(Sensor)
                    .insert(ActiveEvents::COLLISION_EVENTS);
            }
        }


        if let Some(npcs) = &level_layout.npc_layout {
            for npc in npcs {
                let image_handle = asset_server.load(format!("npcs/{}/Model.png", npc.name));

                commands
                    .spawn(npc.clone())
                    .insert((
                        Sprite {
                            image: image_handle,
                            anchor: bevy::sprite::Anchor::Custom(Vec2::new(0., 1.2)),
                            custom_size: Some(Vec2::new(40., 30.)),
                            ..default()
                        },
                        Transform::from_translation(npc.floor_info.position)
                    ))
                    .insert(Collider::cuboid(npc.floor_info.size.x / 2.0, npc.floor_info.size.y / 2.0))
                    .insert(Sensor)
                    .insert(ActiveEvents::COLLISION_EVENTS)
                    .insert(Interactable);
            }
        }


        if let Some(doors) = &level_layout.door_layout {
            for door in doors {
                commands
                    .spawn(*door)
                    .insert((
                        Mesh2d(meshes.add(Rectangle::new(door.floor_info.size.x, door.floor_info.size.y))),
                        MeshMaterial2d(materials.add(DOOR_COLOR)),
                        Transform::from_translation(door.floor_info.position)
                    ))
                    .insert(Collider::cuboid(door.floor_info.size.x / 2.0, door.floor_info.size.y / 2.0))
                    .insert(Sensor)
                    .insert(ActiveEvents::COLLISION_EVENTS)
                    .insert(Interactable);
            }
        }

        if let Some(modifications) = &level_layout.floor_modifications {
            for modification in modifications {
                match modification {
                    FloorModification::JumpPad(jump_pad) => {
                        commands
                            .spawn(*jump_pad)
                            .insert((
                                Mesh2d(meshes.add(Rectangle::new(jump_pad.floor_info.size.x, jump_pad.floor_info.size.y))),
                                MeshMaterial2d(materials.add(JUMP_PAD_COLOR)),
                                Transform::from_translation(jump_pad.floor_info.position)
                            ))
                            .insert(Collider::cuboid(jump_pad.floor_info.size.x / 2.0, jump_pad.floor_info.size.y / 2.0))
                            .insert(Sensor)
                            .insert(ActiveEvents::COLLISION_EVENTS);
                    },
                    FloorModification::GravityInverter(gravity_inverter) => {
                        commands
                            .spawn(*gravity_inverter)
                            .insert((
                                Mesh2d(meshes.add(Rectangle::new(gravity_inverter.floor_info.size.x, gravity_inverter.floor_info.size.y))),
                                MeshMaterial2d(materials.add(GRAVITY_INVERTER_COLOR)),
                                Transform::from_translation(gravity_inverter.floor_info.position)
                            ))
                            .insert(Collider::cuboid(gravity_inverter.floor_info.size.x / 2.0, gravity_inverter.floor_info.size.y / 2.0))
                            .insert(Sensor)
                            .insert(ActiveEvents::COLLISION_EVENTS);
                    },
                    FloorModification::TimeTrial(time_trial) => {
                        commands
                            .spawn(*time_trial)
                            .insert((
                                Mesh2d(meshes.add(Rectangle::new(time_trial.lever_info.size.x, time_trial.lever_info.size.y))),
                                MeshMaterial2d(materials.add(TIME_TRIAL_COLOR)),
                                Transform::from_translation(time_trial.lever_info.position),
                                Interactable
                            ))
                            .insert(Collider::cuboid(time_trial.lever_info.size.x / 2.0, time_trial.lever_info.size.y / 2.0))
                            .insert(Sensor)
                            .insert(ActiveEvents::COLLISION_EVENTS);
                    }
                }
            }
        }

        let (cweampuff_transform, cweampuff_gravity) = &mut *cweampuff;

        cweampuff_gravity.0 = CWEAMPUFF_GRAVITY_SCALE;

        if let Some(position) = level_layout.transition_info.transition_to_position {
            cweampuff_transform.translation = position;

            let new_camera_position = get_adjusted_camera_position(&cweampuff_transform, &level_layout_query, None);
            camera.translation = new_camera_position;
        }
        else if let Some(transition_collider) = level_layout.transition_layout.as_deref().unwrap_or_default().iter().find(|f| f.exit_index == level_layout.transition_info.transition_to_index) {
            cweampuff_transform.translation = transition_collider.safe_position;

            let new_camera_position = get_adjusted_camera_position(&cweampuff_transform, &level_layout_query, None);
            camera.translation = new_camera_position;
        }
    }

    transition_state.set(TransitionState::Finished);
}

pub fn level_transition_collision_reader(
    mut cweampuff: Single<(Entity, &Cweampuff, &mut Velocity, &mut GravityScale), With<Cweampuff>>,
    current_level_layout: Query<Entity, With<LevelLayout>>,
    transition_colliders: Query<(Entity, &TransitionCollider), With<TransitionCollider>>,
    mut contact_events: EventReader<CollisionEvent>,
    mut transition_state: ResMut<NextState<TransitionState>>,
    mut commands: Commands,
) {
    let (cweampuff_entity, cweampuff, cweampuff_velocity, cweampuff_gravity) = &mut *cweampuff;
    for contact_event in contact_events.read() {
        if let CollisionEvent::Started(h1, h2, _flags) = contact_event {
            for (collider_entity, transition_collider) in transition_colliders.iter() {
                if h1.entities().iter().any(|f| *f == collider_entity || *f == *cweampuff_entity) && 
                   h2.entities().iter().any(|f| *f == collider_entity || *f == *cweampuff_entity) {
                    for layout_entity in current_level_layout.iter() {
                        commands.entity(layout_entity).despawn_recursive();
                    }

                    let transition_info = LevelTransitionInfo { transition_to_index: transition_collider.exit_index, transition_to_position: None };

                    cweampuff_velocity.linvel = Vec2::new(0., 0.);
                    cweampuff_gravity.0 = 0.;
                    spawn_level(&mut commands, transition_collider.transition_to_level, cweampuff, transition_info);
                    transition_state.set(TransitionState::Started);

                    return;
                }
            }
        }
    }
}

pub fn manually_transition_to_level(
    current_level_layout: &Query<Entity, With<LevelLayout>>,
    transition_state: &mut ResMut<NextState<TransitionState>>,
    cweampuff: &Cweampuff,
    commands: &mut Commands,
    level: Level,
    position: Vec3
) {
    for layout_entity in current_level_layout.iter() {
        commands.entity(layout_entity).despawn_recursive();
    }

    let transition_info = LevelTransitionInfo { transition_to_index: 0, transition_to_position: Some(position) };

    spawn_level(commands, level, cweampuff, transition_info);
    transition_state.set(TransitionState::Started);
}

fn spawn_level(commands: &mut Commands, level: Level, cweampuff: &Cweampuff, transition_info: LevelTransitionInfo) {
    match level {
        Level::StartingRoom(layout_info) => {
            commands.spawn(LevelLayout {
                floor_layout: layout_info.get_floor_info(cweampuff),
                transition_layout: layout_info.get_transitions_info(cweampuff),
                npc_layout: layout_info.get_npcs(cweampuff),
                door_layout: layout_info.get_doors(cweampuff),
                floor_modifications: layout_info.get_floor_modifications(cweampuff),
                transition_info,
                bgm: layout_info.get_bgm()
            });
        },
        Level::CweamcatLair(layout_info) => {
            commands.spawn(LevelLayout {
                floor_layout: layout_info.get_floor_info(cweampuff),
                transition_layout: layout_info.get_transitions_info(cweampuff),
                npc_layout: layout_info.get_npcs(cweampuff),
                door_layout: layout_info.get_doors(cweampuff),
                floor_modifications: layout_info.get_floor_modifications(cweampuff),
                transition_info,
                bgm: layout_info.get_bgm()
            });
        },
        Level::CweamcatHouse(layout_info) => {
            commands.spawn(LevelLayout {
                floor_layout: layout_info.get_floor_info(cweampuff),
                transition_layout: layout_info.get_transitions_info(cweampuff),
                npc_layout: layout_info.get_npcs(cweampuff),
                door_layout: layout_info.get_doors(cweampuff),
                floor_modifications: layout_info.get_floor_modifications(cweampuff),
                transition_info,
                bgm: layout_info.get_bgm()
            });
        },
        Level::Hell1(layout_info) => {
            commands.spawn(LevelLayout {
                floor_layout: layout_info.get_floor_info(cweampuff),
                transition_layout: layout_info.get_transitions_info(cweampuff),
                npc_layout: layout_info.get_npcs(cweampuff),
                door_layout: layout_info.get_doors(cweampuff),
                floor_modifications: layout_info.get_floor_modifications(cweampuff),
                transition_info,
                bgm: layout_info.get_bgm()
            });
        },
        Level::Hell2(layout_info) => {
            commands.spawn(LevelLayout {
                floor_layout: layout_info.get_floor_info(cweampuff),
                transition_layout: layout_info.get_transitions_info(cweampuff),
                npc_layout: layout_info.get_npcs(cweampuff),
                door_layout: layout_info.get_doors(cweampuff),
                floor_modifications: layout_info.get_floor_modifications(cweampuff),
                transition_info,
                bgm: layout_info.get_bgm()
            });
        },
        Level::Hell3(layout_info) => {
            commands.spawn(LevelLayout {
                floor_layout: layout_info.get_floor_info(cweampuff),
                transition_layout: layout_info.get_transitions_info(cweampuff),
                npc_layout: layout_info.get_npcs(cweampuff),
                door_layout: layout_info.get_doors(cweampuff),
                floor_modifications: layout_info.get_floor_modifications(cweampuff),
                transition_info,
                bgm: layout_info.get_bgm()
            });
        },
        Level::Hell4(layout_info) => {
            commands.spawn(LevelLayout {
                floor_layout: layout_info.get_floor_info(cweampuff),
                transition_layout: layout_info.get_transitions_info(cweampuff),
                npc_layout: layout_info.get_npcs(cweampuff),
                door_layout: layout_info.get_doors(cweampuff),
                floor_modifications: layout_info.get_floor_modifications(cweampuff),
                transition_info,
                bgm: layout_info.get_bgm()
            });
        },
        Level::CerberLair(layout_info) => {
            commands.spawn(LevelLayout {
                floor_layout: layout_info.get_floor_info(cweampuff),
                transition_layout: layout_info.get_transitions_info(cweampuff),
                npc_layout: layout_info.get_npcs(cweampuff),
                door_layout: layout_info.get_doors(cweampuff),
                floor_modifications: layout_info.get_floor_modifications(cweampuff),
                transition_info,
                bgm: layout_info.get_bgm()
            });
        },
        Level::Spaceship1(layout_info) => {
            commands.spawn(LevelLayout {
                floor_layout: layout_info.get_floor_info(cweampuff),
                transition_layout: layout_info.get_transitions_info(cweampuff),
                npc_layout: layout_info.get_npcs(cweampuff),
                door_layout: layout_info.get_doors(cweampuff),
                floor_modifications: layout_info.get_floor_modifications(cweampuff),
                transition_info,
                bgm: layout_info.get_bgm()
            });
        },
        Level::Spaceship2(layout_info) => {
            commands.spawn(LevelLayout {
                floor_layout: layout_info.get_floor_info(cweampuff),
                transition_layout: layout_info.get_transitions_info(cweampuff),
                npc_layout: layout_info.get_npcs(cweampuff),
                door_layout: layout_info.get_doors(cweampuff),
                floor_modifications: layout_info.get_floor_modifications(cweampuff),
                transition_info,
                bgm: layout_info.get_bgm()
            });
        },
        Level::Spaceship3(layout_info) => {
            commands.spawn(LevelLayout {
                floor_layout: layout_info.get_floor_info(cweampuff),
                transition_layout: layout_info.get_transitions_info(cweampuff),
                npc_layout: layout_info.get_npcs(cweampuff),
                door_layout: layout_info.get_doors(cweampuff),
                floor_modifications: layout_info.get_floor_modifications(cweampuff),
                transition_info,
                bgm: layout_info.get_bgm()
            });
        },
        Level::Spaceship4(layout_info) => {
            commands.spawn(LevelLayout {
                floor_layout: layout_info.get_floor_info(cweampuff),
                transition_layout: layout_info.get_transitions_info(cweampuff),
                npc_layout: layout_info.get_npcs(cweampuff),
                door_layout: layout_info.get_doors(cweampuff),
                floor_modifications: layout_info.get_floor_modifications(cweampuff),
                transition_info,
                bgm: layout_info.get_bgm()
            });
        },
        Level::AquwaLair(layout_info) => {
            commands.spawn(LevelLayout {
                floor_layout: layout_info.get_floor_info(cweampuff),
                transition_layout: layout_info.get_transitions_info(cweampuff),
                npc_layout: layout_info.get_npcs(cweampuff),
                door_layout: layout_info.get_doors(cweampuff),
                floor_modifications: layout_info.get_floor_modifications(cweampuff),
                transition_info,
                bgm: layout_info.get_bgm()
            });
        },
        Level::FactoryTransition(layout_info) => {
            commands.spawn(LevelLayout {
                floor_layout: layout_info.get_floor_info(cweampuff),
                transition_layout: layout_info.get_transitions_info(cweampuff),
                npc_layout: layout_info.get_npcs(cweampuff),
                door_layout: layout_info.get_doors(cweampuff),
                floor_modifications: layout_info.get_floor_modifications(cweampuff),
                transition_info,
                bgm: layout_info.get_bgm()
            });
        },
        Level::Factory1(layout_info) => {
            commands.spawn(LevelLayout {
                floor_layout: layout_info.get_floor_info(cweampuff),
                transition_layout: layout_info.get_transitions_info(cweampuff),
                npc_layout: layout_info.get_npcs(cweampuff),
                door_layout: layout_info.get_doors(cweampuff),
                floor_modifications: layout_info.get_floor_modifications(cweampuff),
                transition_info,
                bgm: layout_info.get_bgm()
            });
        },
        Level::Factory2(layout_info) => {
            commands.spawn(LevelLayout {
                floor_layout: layout_info.get_floor_info(cweampuff),
                transition_layout: layout_info.get_transitions_info(cweampuff),
                npc_layout: layout_info.get_npcs(cweampuff),
                door_layout: layout_info.get_doors(cweampuff),
                floor_modifications: layout_info.get_floor_modifications(cweampuff),
                transition_info,
                bgm: layout_info.get_bgm()
            });
        },
        Level::Factory3(layout_info) => {
            commands.spawn(LevelLayout {
                floor_layout: layout_info.get_floor_info(cweampuff),
                transition_layout: layout_info.get_transitions_info(cweampuff),
                npc_layout: layout_info.get_npcs(cweampuff),
                door_layout: layout_info.get_doors(cweampuff),
                floor_modifications: layout_info.get_floor_modifications(cweampuff),
                transition_info,
                bgm: layout_info.get_bgm()
            });
        },
        Level::Factory4(layout_info) => {
            commands.spawn(LevelLayout {
                floor_layout: layout_info.get_floor_info(cweampuff),
                transition_layout: layout_info.get_transitions_info(cweampuff),
                npc_layout: layout_info.get_npcs(cweampuff),
                door_layout: layout_info.get_doors(cweampuff),
                floor_modifications: layout_info.get_floor_modifications(cweampuff),
                transition_info,
                bgm: layout_info.get_bgm()
            });
        },
        Level::NeuroLair(layout_info) => {
            commands.spawn(LevelLayout {
                floor_layout: layout_info.get_floor_info(cweampuff),
                transition_layout: layout_info.get_transitions_info(cweampuff),
                npc_layout: layout_info.get_npcs(cweampuff),
                door_layout: layout_info.get_doors(cweampuff),
                floor_modifications: layout_info.get_floor_modifications(cweampuff),
                transition_info,
                bgm: layout_info.get_bgm()
            });
        },
    }
}