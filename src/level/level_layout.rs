pub mod starting_room_layout;
pub mod cweamcat_lair_layout;

use bevy::{ecs::component::Component, math::{Vec2, Vec3}};

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

#[derive(Clone, Copy)]
pub struct FloorInfo {
    pub position: Vec3,
    pub size: Vec2
}

