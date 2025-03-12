use bevy::math::{Vec2, Vec3};

use crate::{level::Level, npc::NPC, CWEAMPUFF_Z_INDEX};

use super::{spaceship_3_layout::Spaceship3Info, DoorCollider, EntityInfo, FloorAssetType, FloorInfo, FloorModification, GravityInverter, LevelInfo, TransitionCollider};

#[derive(Clone, Copy)]
pub struct Spaceship4Info;

impl LevelInfo for Spaceship4Info {
    fn get_floor_info(&self, _cweampuff: &crate::Cweampuff) -> Box<[FloorInfo]> {
        Box::from([
            FloorInfo { position: Vec3::new(-1675.0, 0.0, 1.0), size: Vec2::new(300.0, 6000.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(-200.0, -2850.0, 1.0), size: Vec2::new(2650.0, 300.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(1500.0, 175.0, 1.0), size: Vec2::new(300.0, 6500.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(-175.0, 3325.0, 1.0), size: Vec2::new(3050.0, 300.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(-175.0, -2450.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(-1075.0, -2150.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(-500.0, -1850.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(0.0, -1150.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(500.0, -850.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(1000.0, -550.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(-1000.0, -50.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(-1300.0, 250.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(-400.0, 550.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(-400.0, 850.0, 1.0), size: Vec2::new(800.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(0.0, 1900.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(800.0, 2200.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(1100.0, 2925.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(500.0, 2875.0, 1.0), size: Vec2::new(150.0, 600.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(350.0, 2925.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(-200.0, 2375.0, 1.0), size: Vec2::new(150.0, 400.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(-900.0, 2875.0, 1.0), size: Vec2::new(150.0, 600.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
        ])
    }

    fn get_transitions_info(&self, _cweampuff: &crate::Cweampuff) -> Option<Box<[TransitionCollider]>> {
        Some(Box::from([
            TransitionCollider { exit_index: 0, safe_position: Vec3::new(1100.0, -2600.0, CWEAMPUFF_Z_INDEX), transition_to_level: Level::Spaceship3(Spaceship3Info), floor_info: EntityInfo { position: Vec3::new(1250.0, -2950.0, 2.0), size: Vec2::new(200.0, 100.0) }  }
        ]))
    }

    fn get_doors(&self, _cweampuff: &crate::Cweampuff) -> Option<Box<[DoorCollider]>> {
        None
    }

    fn get_npcs(&self, _cweampuff: &crate::Cweampuff) -> Option<Box<[NPC]>> {                
        None
    }

    fn get_floor_modifications(&self, _cweampuff: &crate::Cweampuff) -> Option<Box<[FloorModification]>> {
        Some(Box::from([
            FloorModification::GravityInverter(GravityInverter { floor_info: EntityInfo { position: Vec3 { x: -500., y: -1550., z: 0. }, size: Vec2 { x: 300., y: 300. } } }),
            FloorModification::GravityInverter(GravityInverter { floor_info: EntityInfo { position: Vec3 { x: 300., y: -150., z: 0. }, size: Vec2 { x: 400., y: 200. } } }),
            FloorModification::GravityInverter(GravityInverter { floor_info: EntityInfo { position: Vec3 { x: -300., y: -150., z: 0. }, size: Vec2 { x: 400., y: 200. } } }),
            FloorModification::GravityInverter(GravityInverter { floor_info: EntityInfo { position: Vec3 { x: 200., y: 1250., z: 0. }, size: Vec2 { x: 300., y: 300. } } }),
            FloorModification::GravityInverter(GravityInverter { floor_info: EntityInfo { position: Vec3 { x: -700., y: 1750., z: 0. }, size: Vec2 { x: 300., y: 300. } } }),
            FloorModification::GravityInverter(GravityInverter { floor_info: EntityInfo { position: Vec3 { x: 0., y: 2825., z: 0. }, size: Vec2 { x: 2850., y: 1000. } } }),
        ]))
    }
}