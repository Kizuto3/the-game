pub mod starting_room_layout;
pub mod cweamcat_lair_layout;
pub mod cweamcat_house_layout;
pub mod hell_1_layout;
pub mod hell_2_layout;
pub mod hell_3_layout;
pub mod hell_4_layout;
pub mod cerber_lair_layout;
pub mod spaceship_1_layout;
pub mod spaceship_2_layout;
pub mod spaceship_3_layout;
pub mod spaceship_4_layout;
pub mod aquwa_lair_layout;
pub mod factory_transition_layout;
pub mod factory_1_layout;
pub mod factory_2_layout;
pub mod factory_3_layout;
pub mod factory_4_layout;
pub mod neuro_lair_layout;
pub mod factory_hidden_level_layout;

use bevy::{ecs::component::Component, math::{Vec2, Vec3}};
use crate::{npc::NPC, Cweampuff};

use super::Level;

pub enum CollisionType {
    Floor,
    LeftWall,
    RightWall,
    Ceiling
}

#[derive(Clone, Copy, Default)]
pub enum FloorAssetType {
    #[default]
    Forest,
    CweamcatHouse,
    Hell,
    Spaceship,
    Factory
}

#[derive(Component, Default)]
pub struct FloorCollider {
    pub currently_touching_side: Option<CollisionType>
}

#[derive(Component, Default, Clone, Copy)]
pub struct BreakableWall {
    pub index: u32
}

#[derive(Component, Clone, Copy)]
pub struct TransitionCollider {
    pub exit_index: u32,
    pub floor_info: EntityInfo,
    pub safe_position: Vec3,
    pub transition_to_level: Level
}

#[derive(Clone, Copy, Default)]
pub enum DoorType {
    #[default]
    Door,
    Teleport,
    MilkHouse
}

#[derive(Component, Clone, Copy)]
pub struct DoorCollider {
    pub floor_info: EntityInfo,
    pub transition_to_level: Level,
    pub safe_position: Vec3,
    pub is_active: bool,
    pub door_type: DoorType,
}

#[derive(Clone, Copy, Default)]
pub struct FloorInfo {
    pub position: Vec3,
    pub size: Vec2,
    pub breakable_wall: Option<BreakableWall>,
    pub floor_asset: FloorAssetType
}

#[derive(Clone, Copy, Default)]
pub struct EntityInfo {
    pub position: Vec3,
    pub size: Vec2
}

#[derive(Component, Clone, Copy)]
pub struct JumpPad {
    pub floor_info: EntityInfo
}

#[derive(Component, Clone, Copy)]
pub struct GravityInverter {
    pub floor_info: EntityInfo
}

#[derive(Component, Clone, Copy)]
pub struct TimeTrial {
    pub lever_info: EntityInfo,
    pub floor_infos: &'static [FloorInfo],
    pub seconds_to_complete: u64,
    pub id: u32,
    pub is_active: bool
}

#[derive(Component, Clone, Copy)]
pub struct IllusoryWall {
    pub position: Vec3,
    pub size: Vec2,
    pub floor_asset: FloorAssetType
}

#[derive(Component, Clone, Copy)]
pub struct Decoration {
    pub position: Vec3,
    pub size: Vec2,
    pub asset: &'static str
}

pub enum FloorModification {
    JumpPad(JumpPad),
    GravityInverter(GravityInverter),
    TimeTrial(TimeTrial),
    IllusoryWall(IllusoryWall),
    Decoration(Decoration),
}

pub trait LevelInfo: Sync + Send {
    fn get_floor_info(&self, cweampuff: &Cweampuff) -> Box<[FloorInfo]>;
    fn get_transitions_info(&self, cweampuff: &Cweampuff) -> Option<Box<[TransitionCollider]>>;
    fn get_doors(&self, cweampuff: &Cweampuff) -> Option<Box<[DoorCollider]>>;
    fn get_npcs(&self, cweampuff: &Cweampuff) -> Option<Box<[NPC]>>;
    fn get_floor_modifications(&self, cweampuff: &Cweampuff) -> Option<Box<[FloorModification]>>;
    fn get_bgm(&self) -> Option<&'static str>;
    fn get_background(&self) -> FloorAssetType;
}