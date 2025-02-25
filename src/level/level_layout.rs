pub mod starting_room_layout;
pub mod cweamcat_lair_layout;
pub mod cweamcat_house_layout;
pub mod hell_1_layout;
pub mod hell_2_layout;
pub mod hell_3_layout;

use bevy::{ecs::component::Component, math::{Vec2, Vec3}};

use crate::{npc::NPC, Cweampuff};

use super::Level;

#[derive(Component)]
pub struct FloorCollider {
    pub entity_index: u32
}

#[derive(Component, Clone, Copy)]
pub struct TransitionCollider {
    pub exit_index: u32,
    pub floor_info: FloorInfo,
    pub safe_position: Vec3,
    pub transition_to_level: Level
}

#[derive(Component, Clone, Copy)]
pub struct DoorCollider {
    pub floor_info: FloorInfo,
    pub transition_to_level: Level,
    pub safe_position: Vec3,
    pub is_active: bool
}

#[derive(Clone, Copy)]
pub struct FloorInfo {
    pub position: Vec3,
    pub size: Vec2
}

#[derive(Component, Clone, Copy)]
pub struct JumpPad {
    pub floor_info: FloorInfo
}

pub enum FloorModification {
    JumpPad(JumpPad)
}

pub trait LevelInfo {
    fn get_floor_info(&self, cweampuff: &Cweampuff) -> Vec<FloorInfo>;
    fn get_transitions_info(&self, cweampuff: &Cweampuff) -> Vec<TransitionCollider>;
    fn get_doors(&self, cweampuff: &Cweampuff) -> Vec<DoorCollider>;
    fn get_npcs(&self, cweampuff: &Cweampuff) -> Vec<NPC>;
    fn get_floor_modifications(&self, cweampuff: &Cweampuff) -> Vec<FloorModification>;
}