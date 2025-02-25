use bevy::{ecs::observer::TriggerTargets, prelude::*};
use bevy_rapier2d::prelude::*;
use level_layout::cweamcat_house_layout::CweamcatHouseInfo;
use level_layout::hell_1_layout::Hell1Info;
use level_layout::hell_2_layout::Hell2Info;
use level_layout::hell_3_layout::Hell3Info;
use level_layout::{DoorCollider, FloorModification};
use level_layout::{cweamcat_lair_layout::CweamcatLairInfo, starting_room_layout::StartingRoomInfo, FloorCollider, FloorInfo, TransitionCollider};
use transition_states::TransitionState;

use crate::{camera::get_adjusted_camera_position, interactable::Interactable, npc::NPC, Cweampuff};
use crate::level::level_layout::LevelInfo;

pub mod transition_states;
pub mod level_layout;
pub mod door;
pub mod progression;
pub mod floor_modification;

const TRANSITION_COLOR: Color = Color::srgb(0.5, 1.0, 0.5);
const NPC_COLOR: Color = Color::srgb(0.5, 0.5, 1.0);
const DOOR_COLOR: Color = Color::srgb(0.9, 0.2, 0.9);
const FLOOR_COLOR: Color = Color::srgb(1.0, 0.5, 0.5);
const JUMP_PAD_COLOR: Color = Color::srgb(0.2, 0.9, 0.9);

#[derive(Clone, Copy)]
pub enum Level {
    StartingRoom(StartingRoomInfo),
    CweamcatLair(CweamcatLairInfo),
    CweamcatHouse(CweamcatHouseInfo),
    Hell1(Hell1Info),
    Hell2(Hell2Info),
    Hell3(Hell3Info)
}

pub struct LevelTransitionInfo {
    pub transition_to_index: u32,
    pub transition_to_position: Option<Vec3>
}

#[derive(Component)]
pub struct LevelLayout {
    pub floor_layout: Vec<FloorInfo>,
    pub transition_layout: Vec<TransitionCollider>,
    pub npc_layout: Vec<NPC>,
    pub door_layout: Vec<DoorCollider>,
    pub floor_modifications: Vec<FloorModification>,
    pub transition_info: LevelTransitionInfo,
}

pub fn despawn_current_level(
    mut commands: Commands,
    floor_query: Query<Entity, (With<FloorCollider>, Without<Camera2d>)>,
    transitions_query: Query<Entity, (With<Sensor>, Without<Camera2d>)>,
    interactable_query: Query<Entity, (With<Interactable>, Without<Camera2d>)>
) {
    for entity in transitions_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in floor_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in interactable_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn spawn_new_level(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut transition_state: ResMut<NextState<TransitionState>>,
    mut cweampuff: Single<&mut Transform, (With<Cweampuff>, Without<Camera2d>)>,
    mut camera: Single<&mut Transform, With<Camera2d>>,
    level_layout_query: Query<&LevelLayout, With<LevelLayout>>
) {
    for level_layout in level_layout_query.iter() {
        for floor in &level_layout.floor_layout {
            commands
            .spawn(RigidBody::Fixed)
            .insert((
                Mesh2d(meshes.add(Rectangle::new(floor.size.x, floor.size.y))),
                MeshMaterial2d(materials.add(FLOOR_COLOR)),
                Transform::from_translation(floor.position)
            ))
            .insert(Collider::cuboid(floor.size.x / 2.0, floor.size.y / 2.0))
            .insert(Friction::coefficient(0.7))
            .insert(ActiveEvents::COLLISION_EVENTS)
            .insert(FloorCollider {entity_index: 0});
        }

        for transition in &level_layout.transition_layout {
            commands
            .spawn(transition.clone())
            .insert((
                Mesh2d(meshes.add(Rectangle::new(transition.floor_info.size.x, transition.floor_info.size.y))),
                MeshMaterial2d(materials.add(TRANSITION_COLOR)),
                Transform::from_translation(transition.floor_info.position)
            ))
            .insert(Collider::cuboid(transition.floor_info.size.x / 2.0, transition.floor_info.size.y / 2.0))
            .insert(Sensor)
            .insert(ActiveEvents::COLLISION_EVENTS);
        }

        for npc in &level_layout.npc_layout {
            commands
            .spawn(npc.clone())
            .insert((
                Mesh2d(meshes.add(Rectangle::new(npc.floor_info.size.x, npc.floor_info.size.y))),
                MeshMaterial2d(materials.add(NPC_COLOR)),
                Transform::from_translation(npc.floor_info.position)
            ))
            .insert(Collider::cuboid(npc.floor_info.size.x / 2.0, npc.floor_info.size.y / 2.0))
            .insert(Sensor)
            .insert(ActiveEvents::COLLISION_EVENTS)
            .insert(Interactable);
        }

        for door in &level_layout.door_layout {
            commands
            .spawn(door.clone())
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

        for modification in &level_layout.floor_modifications {
            match modification {
                FloorModification::JumpPad(jump_pad) => {
                    commands
                        .spawn(jump_pad.clone())
                        .insert((
                            Mesh2d(meshes.add(Rectangle::new(jump_pad.floor_info.size.x, jump_pad.floor_info.size.y))),
                            MeshMaterial2d(materials.add(JUMP_PAD_COLOR)),
                            Transform::from_translation(jump_pad.floor_info.position)
                        ))
                        .insert(Collider::cuboid(jump_pad.floor_info.size.x / 2.0, jump_pad.floor_info.size.y / 2.0))
                        .insert(Sensor)
                        .insert(ActiveEvents::COLLISION_EVENTS);
                }
            }
        }

        if let Some(position) = level_layout.transition_info.transition_to_position {
            cweampuff.translation = position;

            let new_camera_position = get_adjusted_camera_position(&cweampuff, &level_layout_query, None);
            camera.translation = new_camera_position;
        }
        else if let Some(transition_collider) = level_layout.transition_layout.iter().find(|f| f.exit_index == level_layout.transition_info.transition_to_index) {
            cweampuff.translation = transition_collider.safe_position;

            let new_camera_position = get_adjusted_camera_position(&cweampuff, &level_layout_query, None);
            camera.translation = new_camera_position;
        }
    }

    transition_state.set(TransitionState::Finished);
}

pub fn level_transition_collision_reader(
    mut cweampuff: Single<(Entity, &Cweampuff, &mut Velocity), With<Cweampuff>>,
    current_level_layout: Query<Entity, With<LevelLayout>>,
    transition_colliders: Query<(Entity, &TransitionCollider), With<TransitionCollider>>,
    mut contact_events: EventReader<CollisionEvent>,
    mut transition_state: ResMut<NextState<TransitionState>>,
    mut commands: Commands,
) {
    let (cweampuff_entity, cweampuff, cweampuff_velocity) = &mut *cweampuff;
    for contact_event in contact_events.read() {
        if let CollisionEvent::Started(h1, h2, _flags) = contact_event {
            for (collider_entity, transition_collider) in transition_colliders.iter() {
                if h1.entities().iter().any(|f| *f == collider_entity || *f == *cweampuff_entity) && 
                   h2.entities().iter().any(|f| *f == collider_entity || *f == *cweampuff_entity) {
                    for layout_entity in current_level_layout.iter() {
                        commands.entity(layout_entity).despawn_recursive();
                    }

                    let transition_info = LevelTransitionInfo { transition_to_index: transition_collider.exit_index, transition_to_position: Option::None };

                    cweampuff_velocity.linvel = Vec2::new(0., 0.);
                    spawn_level(&mut commands, transition_collider.transition_to_level, &cweampuff, transition_info);
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
    mut commands: &mut Commands,
    level: Level,
    position: Vec3
) {
    for layout_entity in current_level_layout.iter() {
        commands.entity(layout_entity).despawn_recursive();
    }

    let transition_info = LevelTransitionInfo { transition_to_index: 0, transition_to_position: Some(position) };

    spawn_level(&mut commands, level, &cweampuff, transition_info);
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
                transition_info
            });
        },
        Level::CweamcatLair(layout_info) => {
            commands.spawn(LevelLayout {
                floor_layout: layout_info.get_floor_info(cweampuff),
                transition_layout: layout_info.get_transitions_info(cweampuff),
                npc_layout: layout_info.get_npcs(cweampuff),
                door_layout: layout_info.get_doors(cweampuff),
                floor_modifications: layout_info.get_floor_modifications(cweampuff),
                transition_info
            });
        }
        Level::CweamcatHouse(layout_info) => {
            commands.spawn(LevelLayout {
                floor_layout: layout_info.get_floor_info(cweampuff),
                transition_layout: layout_info.get_transitions_info(cweampuff),
                npc_layout: layout_info.get_npcs(cweampuff),
                door_layout: layout_info.get_doors(cweampuff),
                floor_modifications: layout_info.get_floor_modifications(cweampuff),
                transition_info
            });
        }
        Level::Hell1(layout_info) => {
            commands.spawn(LevelLayout {
                floor_layout: layout_info.get_floor_info(cweampuff),
                transition_layout: layout_info.get_transitions_info(cweampuff),
                npc_layout: layout_info.get_npcs(cweampuff),
                door_layout: layout_info.get_doors(cweampuff),
                floor_modifications: layout_info.get_floor_modifications(cweampuff),
                transition_info
            });
        }
        Level::Hell2(layout_info) => {
            commands.spawn(LevelLayout {
                floor_layout: layout_info.get_floor_info(cweampuff),
                transition_layout: layout_info.get_transitions_info(cweampuff),
                npc_layout: layout_info.get_npcs(cweampuff),
                door_layout: layout_info.get_doors(cweampuff),
                floor_modifications: layout_info.get_floor_modifications(cweampuff),
                transition_info
            });
        }
        Level::Hell3(layout_info) => {
            commands.spawn(LevelLayout {
                floor_layout: layout_info.get_floor_info(cweampuff),
                transition_layout: layout_info.get_transitions_info(cweampuff),
                npc_layout: layout_info.get_npcs(cweampuff),
                door_layout: layout_info.get_doors(cweampuff),
                floor_modifications: layout_info.get_floor_modifications(cweampuff),
                transition_info
            });
        }
    }
}