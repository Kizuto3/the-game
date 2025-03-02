pub mod starting_room_layout;
pub mod cweamcat_lair_layout;
pub mod cweamcat_house_layout;
pub mod hell_1_layout;
pub mod hell_2_layout;
pub mod hell_3_layout;
pub mod hell_4_layout;
pub mod cerber_lair_layout;

use bevy::{ecs::component::Component, math::{Vec2, Vec3}};

use crate::{npc::NPC, Cweampuff};

use super::Level;

pub enum CollisionType {
    Floor,
    Wall,
    Ceiling
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

#[derive(Component, Clone, Copy)]
pub struct DoorCollider {
    pub floor_info: EntityInfo,
    pub transition_to_level: Level,
    pub safe_position: Vec3,
    pub is_active: bool
}

#[derive(Clone, Copy, Default)]
pub struct FloorInfo {
    pub position: Vec3,
    pub size: Vec2,
    pub breakable_wall: Option<BreakableWall>
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

pub enum FloorModification {
    JumpPad(JumpPad)
}

pub trait LevelInfo {
    fn get_floor_info(&self, cweampuff: &Cweampuff) -> Box<[FloorInfo]>;
    fn get_transitions_info(&self, cweampuff: &Cweampuff) -> Option<Box<[TransitionCollider]>>;
    fn get_doors(&self, cweampuff: &Cweampuff) -> Option<Box<[DoorCollider]>>;
    fn get_npcs(&self, cweampuff: &Cweampuff) -> Option<Box<[NPC]>>;
    fn get_floor_modifications(&self, cweampuff: &Cweampuff) -> Option<Box<[FloorModification]>>;
}