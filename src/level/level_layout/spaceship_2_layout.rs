use bevy::math::{Vec2, Vec3};

use crate::{level::Level, npc::NPC, CWEAMPUFF_Z_INDEX};

use super::{spaceship_1_layout::Spaceship1Info, spaceship_3_layout::Spaceship3Info, DoorCollider, EntityInfo, FloorAssetType, FloorInfo, FloorModification, LevelInfo, TransitionCollider};

#[derive(Clone, Copy)]
pub struct Spaceship2Info;

impl LevelInfo for Spaceship2Info {
    fn get_floor_info(&self, _cweampuff: &crate::Cweampuff) -> Box<[FloorInfo]> {
        Box::from([
            FloorInfo { position: Vec3::new(-5000.0, 0.0, 1.0), size: Vec2::new(300.0, 2700.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(-3300.0, 1500.0, 1.0), size: Vec2::new(3700.0, 300.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(625.0, 1500.0, 1.0), size: Vec2::new(3750.0, 300.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(2500.0, 125.0, 1.0), size: Vec2::new(300.0, 2450.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(-2500.0, -1500.0, 1.0), size: Vec2::new(10000.0, 300.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(800.0, -150.0, 1.0), size: Vec2::new(300.0, 2400.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(1800.0, -1100.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(1800.0, -800.0, 1.0), size: Vec2::new(800.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(1100.0, -500.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(2100.0, -200.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(1400.0, 100.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(1800.0, 400.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(1800.0, 700.0, 1.0), size: Vec2::new(800.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(250.0, 700.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(-100.0, 400.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(-100.0, 0.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(250.0, -200.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(250.0, -500.0, 1.0), size: Vec2::new(800.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(-300.0, -1100.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(250.0, -800.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(-600.0, 700.0, 1.0), size: Vec2::new(300.0, 1300.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(-2000.0, 200.0, 1.0), size: Vec2::new(2500.0, 300.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(-1000.0, -400.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(-1900.0, -100.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(-2600.0, -400.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(-2900.0, -150.0, 1.0), size: Vec2::new(150.0, 400.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(-3600.0, -400.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(-4300.0, -100.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(-3800.0, 200.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(-1350.0, 1100.0, 1.0), size: Vec2::new(800.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(-1350.0, 800.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(-2150.0, 500.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
        ])
    }

    fn get_transitions_info(&self, _cweampuff: &crate::Cweampuff) -> Option<Box<[TransitionCollider]>> {
        Some(Box::from([
            TransitionCollider { exit_index: 0, safe_position: Vec3::new(2400.0, -1300.0, CWEAMPUFF_Z_INDEX), transition_to_level: Level::Spaceship1(Spaceship1Info), floor_info: EntityInfo { position: Vec3::new(2550.0, -1200.0, 2.0), size: Vec2::new(100.0, 300.0) }  },
            TransitionCollider { exit_index: 1, safe_position: Vec3::new(-1350.0, 1400.0, CWEAMPUFF_Z_INDEX), transition_to_level: Level::Spaceship3(Spaceship3Info), floor_info: EntityInfo { position: Vec3::new(-1350.0, 1600.0, 2.0), size: Vec2::new(200.0, 200.0) }  }
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
        Some("spaceship")
    }

    fn get_background(&self) -> FloorAssetType {
        FloorAssetType::Spaceship
    }
}