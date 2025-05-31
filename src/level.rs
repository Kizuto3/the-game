use bevy::{
    audio::{PlaybackMode, Volume},
    ecs::observer::TriggerTargets,
    prelude::*,
};
use bevy_rapier2d::prelude::*;
use level_bgm::LevelBGM;
use std::{collections::HashMap, sync::LazyLock};
use transition_states::TransitionState;

use crate::animations::AnimationConfig;
use crate::level::level_layout::{
    aquwa_lair_layout::AquwaLairInfo, cerber_lair_layout::CerberLairInfo,
    cweamcat_house_layout::CweamcatHouseInfo, cweamcat_lair_layout::CweamcatLairInfo,
    factory_1_layout::Factory1Info, factory_2_layout::Factory2Info, factory_3_layout::Factory3Info,
    factory_4_layout::Factory4Info, factory_hidden_level_layout::FactoryHiddenLevelInfo,
    factory_transition_layout::FactoryTransitionInfo, hell_1_layout::Hell1Info,
    hell_2_layout::Hell2Info, hell_3_layout::Hell3Info, hell_4_layout::Hell4Info,
    neuro_lair_layout::NeuroLairInfo, spaceship_1_layout::Spaceship1Info,
    spaceship_2_layout::Spaceship2Info, spaceship_3_layout::Spaceship3Info,
    spaceship_4_layout::Spaceship4Info, starting_room_layout::StartingRoomInfo, DoorCollider,
    DoorType, FloorAssetType, FloorCollider, FloorInfo, FloorModification, LevelInfo,
    TransitionCollider,
};
use crate::npc::{MILK, MILK_ASLEEP};
use crate::CWEAMPUFF_GRAVITY_SCALE;
use crate::{
    camera::get_adjusted_camera_position, interactable::Interactable, npc::NPC, Cweampuff,
};

pub mod cheats;
pub mod door;
pub mod floor_modification;
pub mod level_bgm;
pub mod level_layout;
pub mod progression;
pub mod transition_states;

const TRANSITION_COLOR: Color = Color::srgb(0.5, 1.0, 0.5);
const GRAVITY_INVERTER_COLOR: Color = Color::srgba(0.1, 0.2, 0.2, 0.5);

pub static LEVELS: LazyLock<LevelMap> = LazyLock::new(|| {
    let mut levels: HashMap<Level, Box<dyn LevelInfo>> = HashMap::with_capacity(20);    //Increment this as needed to avoid over allocating/reallocations
    levels.insert(Level::StartingRoom, Box::new(StartingRoomInfo));
    levels.insert(Level::CweamcatLair, Box::new(CweamcatLairInfo));
    levels.insert(Level::CweamcatHouse, Box::new(CweamcatHouseInfo));
    levels.insert(Level::Hell1, Box::new(Hell1Info));
    levels.insert(Level::Hell2, Box::new(Hell2Info));
    levels.insert(Level::Hell3, Box::new(Hell3Info));
    levels.insert(Level::Hell4, Box::new(Hell4Info));
    levels.insert(Level::CerberLair, Box::new(CerberLairInfo));
    levels.insert(Level::Spaceship1, Box::new(Spaceship1Info));
    levels.insert(Level::Spaceship2, Box::new(Spaceship2Info));
    levels.insert(Level::Spaceship3, Box::new(Spaceship3Info));
    levels.insert(Level::Spaceship4, Box::new(Spaceship4Info));
    levels.insert(Level::AquwaLair, Box::new(AquwaLairInfo));
    levels.insert(Level::FactoryTransition, Box::new(FactoryTransitionInfo));
    levels.insert(Level::Factory1, Box::new(Factory1Info));
    levels.insert(Level::Factory2, Box::new(Factory2Info));
    levels.insert(Level::Factory3, Box::new(Factory3Info));
    levels.insert(Level::Factory4, Box::new(Factory4Info));
    levels.insert(Level::FactoryHiddenLevel, Box::new(FactoryHiddenLevelInfo));
    levels.insert(Level::NeuroLair, Box::new(NeuroLairInfo));
    LevelMap { levels }
});

pub struct LevelMap {
    levels: HashMap<Level, Box<dyn LevelInfo>>,
}

impl LevelMap {
    pub fn get_level_info(&self, level_name: &Level) -> &dyn LevelInfo {
        self.levels
            .get(level_name)
            .unwrap_or_else(|| panic!("Attempted to load level info for None level: {level_name:#?}"))
            .as_ref()
    }
}

#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
pub enum Level {
    StartingRoom,
    CweamcatLair,
    CweamcatHouse,
    Hell1,
    Hell2,
    Hell3,
    Hell4,
    CerberLair,
    Spaceship1,
    Spaceship2,
    Spaceship3,
    Spaceship4,
    AquwaLair,
    FactoryTransition,
    Factory1,
    Factory2,
    Factory3,
    Factory4,
    FactoryHiddenLevel,
    NeuroLair,
}

#[derive(Component)]
pub struct BackgroundComponent;

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
    pub bgm: Option<&'static str>,
    pub background: FloorAssetType,
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
        commands.entity(entity).despawn();
    }
    for entity in floor_query.iter() {
        commands.entity(entity).despawn();
    }
    for entity in interactable_query.iter() {
        commands.entity(entity).despawn();
    }
    for entity in background_query.iter() {
        commands.entity(entity).despawn();
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
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    for level_layout in level_layout_query.iter() {
        match level_layout.bgm {
            Some(bgm) => {
                if bgm_query.is_empty() {
                    let audio_handle = asset_server.load(format!("ost/{}.mp3", bgm));

                    let mut playback_settings = PlaybackSettings::default().with_volume(Volume::Linear(0.0));
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
                        commands.entity(current_bgm_entity).despawn();
    
                        let mut playback_settings = PlaybackSettings::default().with_volume(Volume::Linear(0.0));
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
                    commands.entity(current_bgm_entity).despawn();
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
                        border: BorderRect { left: 0., right: 0., top: 40., bottom: 0. },
                        center_scale_mode: SliceScaleMode::Tile { stretch_value: 1. },
                        sides_scale_mode: SliceScaleMode::Tile { stretch_value: 1. },
                        max_corner_scale: 1.0
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

        let background_image_handle = match level_layout.background {
            FloorAssetType::Forest => asset_server.load("forest.png"),
            FloorAssetType::Hell => asset_server.load("hell.png"),
            FloorAssetType::CweamcatHouse => asset_server.load("house.png"),
            FloorAssetType::Factory => asset_server.load("factory.png"),
            FloorAssetType::Spaceship => asset_server.load("spaceship.png"),
        };

        let width = (max_x - min_x).abs();
        let height = (max_y - min_y).abs();
        let x = (max_x + min_x) / 2.;
        let y = (max_y + min_y) / 2.;

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

                let (sprite_size, anchor) = if npc.name == MILK || npc.name == MILK_ASLEEP {
                    (Vec2::new(200., 200.), Vec2::new(0., 0.))
                } else {
                    (Vec2::new(60., 45.), Vec2::new(0., 0.65))
                };

                commands
                    .spawn(npc.clone())
                    .insert((
                        Sprite {
                            image: image_handle,
                            anchor: bevy::sprite::Anchor::Custom(anchor),
                            custom_size: Some(sprite_size),
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
                let mut door_commands = commands.spawn((
                    *door,
                    Transform::from_translation(door.floor_info.position),
                    Collider::cuboid(door.floor_info.size.x / 2.0, door.floor_info.size.y / 2.0),
                    Sensor,
                    ActiveEvents::COLLISION_EVENTS,
                    Interactable
                ));

                if let DoorType::Teleport = door.door_type {
                    let texture = asset_server.load("floor_modifications/Teleporter.png");

                    door_commands.insert(
                        Sprite {
                            image: texture,
                            anchor: bevy::sprite::Anchor::Center,
                            custom_size: Some(Vec2::new(door.floor_info.size.x, door.floor_info.size.y)),
                            ..default()
                        });
                }
            }
        }

        if let Some(modifications) = &level_layout.floor_modifications {
            for modification in modifications {
                match modification {
                    FloorModification::JumpPad(jump_pad) => {
                        let texture = asset_server.load("floor_modifications/Wind_animation.png");

                        // the sprite sheet has 3 sprites arranged in a row, and they are all 200px x 200px
                        let layout = TextureAtlasLayout::from_grid(UVec2::splat(200), 3, 1, None, None);
                        let texture_atlas_layout = texture_atlas_layouts.add(layout);

                        let animation_config = AnimationConfig::new(0, 2, 15);

                        commands
                            .spawn(*jump_pad)
                            .insert((
                                Sprite {
                                    image: texture,
                                    texture_atlas: Some(TextureAtlas { layout: texture_atlas_layout, index: animation_config.first_sprite_index }),
                                    ..default()
                                },
                                Transform::from_translation(jump_pad.floor_info.position),
                                animation_config
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
                        let texture = asset_server.load("floor_modifications/Lever1.png");

                        commands
                            .spawn(*time_trial)
                            .insert((
                                Sprite {
                                    image: texture,
                                    anchor: bevy::sprite::Anchor::Center,
                                    custom_size: Some(Vec2::new(time_trial.lever_info.size.x, time_trial.lever_info.size.y)),
                                    ..default()
                                },
                                Transform::from_translation(time_trial.lever_info.position),
                                Interactable
                            ))
                            .insert(Collider::cuboid(time_trial.lever_info.size.x / 2.0, time_trial.lever_info.size.y / 2.0))
                            .insert(Sensor)
                            .insert(ActiveEvents::COLLISION_EVENTS);
                    },
                    FloorModification::IllusoryWall(illusory_wall) => {
                        let illusory_wall_handle = match illusory_wall.floor_asset {
                            FloorAssetType::Forest => asset_server.load("tiles/Forest.png"),
                            FloorAssetType::CweamcatHouse => asset_server.load("tiles/CweamcatHouse.png"),
                            FloorAssetType::Hell => asset_server.load("tiles/Hell.png"),
                            FloorAssetType::Spaceship => asset_server.load("tiles/Spaceship.png"),
                            FloorAssetType::Factory => asset_server.load("tiles/Factory.png")
                        };

                        commands
                            .spawn((
                                *illusory_wall,
                                Transform::from_translation(illusory_wall.position),
                                Sensor,
                                Sprite {
                                    image: illusory_wall_handle,
                                    anchor: bevy::sprite::Anchor::Center,
                                    custom_size: Some(Vec2::new(illusory_wall.size.x, illusory_wall.size.y)),
                                    image_mode: SpriteImageMode::Sliced(TextureSlicer {
                                        border: BorderRect { left: 18., right: 15., top: 38., bottom: 11. },
                                        center_scale_mode: SliceScaleMode::Tile { stretch_value: 1. },
                                        sides_scale_mode: SliceScaleMode::Tile { stretch_value: 1. },
                                        max_corner_scale: 1.0
                                    }),
                                    ..default()
                                },
                            ));
                    },
                    FloorModification::Decoration(decoration) => {
                        let decoration_handle = asset_server.load(format!("decorations/{}.png", decoration.asset));

                        commands
                            .spawn((
                                *decoration,
                                Transform::from_translation(decoration.position),
                                Sensor,
                                Sprite {
                                    image: decoration_handle,
                                    anchor: bevy::sprite::Anchor::Center,
                                    custom_size: Some(Vec2::new(decoration.size.x, decoration.size.y)),
                                    image_mode: SpriteImageMode::Auto,
                                    ..default()
                                },
                            ));
                    },
                }
            }
        }

        let (cweampuff_transform, cweampuff_gravity) = &mut *cweampuff;

        cweampuff_gravity.0 = CWEAMPUFF_GRAVITY_SCALE;

        if let Some(position) = level_layout.transition_info.transition_to_position {
            cweampuff_transform.translation = position;

            let new_camera_position = get_adjusted_camera_position(cweampuff_transform, &level_layout_query, None);
            camera.translation = new_camera_position;
        }
        else if let Some(transition_collider) = level_layout.transition_layout.as_deref().unwrap_or_default().iter().find(|f| f.exit_index == level_layout.transition_info.transition_to_index) {
            cweampuff_transform.translation = transition_collider.safe_position;

            let new_camera_position = get_adjusted_camera_position(cweampuff_transform, &level_layout_query, None);
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
        if let CollisionEvent::Started(h1, h2, _) = contact_event {
            for (collider_entity, transition_collider) in transition_colliders.iter() {
                if h1.entities().any(|f| f == collider_entity || f == *cweampuff_entity) && 
                   h2.entities().any(|f| f == collider_entity || f == *cweampuff_entity) {
                    for layout_entity in current_level_layout.iter() {
                        commands.entity(layout_entity).despawn();
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
        commands.entity(layout_entity).despawn();
    }

    let transition_info = LevelTransitionInfo { transition_to_index: 0, transition_to_position: Some(position) };

    spawn_level(commands, level, cweampuff, transition_info);
    transition_state.set(TransitionState::Started);
}

fn spawn_level(commands: &mut Commands, level: Level, cweampuff: &Cweampuff, transition_info: LevelTransitionInfo) {
    let layout_info = LEVELS.get_level_info(&level);

    commands.spawn(LevelLayout {
        floor_layout: layout_info.get_floor_info(cweampuff),
        transition_layout: layout_info.get_transitions_info(cweampuff),
        npc_layout: layout_info.get_npcs(cweampuff),
        door_layout: layout_info.get_doors(cweampuff),
        floor_modifications: layout_info.get_floor_modifications(cweampuff),
        transition_info,
        bgm: layout_info.get_bgm(),
        background: layout_info.get_background()
    });
}