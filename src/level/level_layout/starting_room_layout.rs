use bevy::math::{Vec2, Vec3};

use crate::{level::{level_layout::cweamcat_lair_layout::CweamcatLairInfo, Level}, npc::NPC, CWEAMPUFF_Z_INDEX};

use super::{DoorCollider, EntityInfo, FloorAssetType, FloorInfo, FloorModification, LevelInfo, TransitionCollider};

#[derive(Clone, Copy)]
pub struct StartingRoomInfo;

impl LevelInfo for StartingRoomInfo {
    fn get_floor_info(&self, _cweampuff: &crate::Cweampuff) -> Box<[FloorInfo]> {
        Box::from([
            FloorInfo { position: Vec3::new(-450.0, 550.0, 1.0), size: Vec2::new(100.0, 1400.0), breakable_wall: None, floor_asset: FloorAssetType::Forest },
            FloorInfo { position: Vec3::new(500.0, -400.0, 1.0), size: Vec2::new(2000.0, 500.0), breakable_wall: None, floor_asset: FloorAssetType::Forest },
            FloorInfo { position: Vec3::new(2000.0, -200.0, 1.0), size: Vec2::new(1000.0, 900.0), breakable_wall: None, floor_asset: FloorAssetType::Forest },
            FloorInfo { position: Vec3::new(2650.0, 0.0, 1.0), size: Vec2::new(300.0, 1600.0), breakable_wall: None, floor_asset: FloorAssetType::Forest },
            FloorInfo { position: Vec3::new(650.0, -40.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Forest },
            FloorInfo { position: Vec3::new(1050.0, 120.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Forest },
            FloorInfo { position: Vec3::new(1900.0, 500.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Forest },
            FloorInfo { position: Vec3::new(1700.0, 650.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Forest },
            FloorInfo { position: Vec3::new(2300.0, 365.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Forest },
            FloorInfo { position: Vec3::new(2300.0, 750.0, 1.0), size: Vec2::new(400.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Forest },
            FloorInfo { position: Vec3::new(2450.0, 1300.0, 1.0), size: Vec2::new(700.0, 500.0), breakable_wall: None, floor_asset: FloorAssetType::Forest },
        ])
    }

    fn get_transitions_info(&self, _cweampuff: &crate::Cweampuff) -> Option<Box<[TransitionCollider]>> {
        Some(Box::from([
            TransitionCollider { exit_index: 0, safe_position: Vec3::new(2600.0, 820.0, CWEAMPUFF_Z_INDEX), transition_to_level: Level::CweamcatLair(CweamcatLairInfo), floor_info: EntityInfo { position: Vec3::new(2700.0, 925.0, 2.0), size: Vec2::new(100.0, 250.0) }  }
        ]))
    }

    fn get_doors(&self, _cweampuff: &crate::Cweampuff) -> Option<Box<[DoorCollider]>> {
        None
    }

    fn get_npcs(&self, _cweampuff: &crate::Cweampuff) -> Option<Box<[NPC]>> {
        None
    }

    fn get_floor_modifications(&self, _cweampuff: &crate::Cweampuff) -> Option<Box<[FloorModification]>> {
        None
    }

    fn get_bgm(&self) -> Option<&'static str> {
        None
    }
}