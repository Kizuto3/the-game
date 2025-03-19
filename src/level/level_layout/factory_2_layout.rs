use bevy::math::{Vec2, Vec3};

use crate::{level::Level, npc::NPC, CWEAMPUFF_Z_INDEX};

use super::{factory_1_layout::Factory1Info, DoorCollider, EntityInfo, FloorAssetType, FloorInfo, FloorModification, LevelInfo, TransitionCollider};

#[derive(Clone, Copy)]
pub struct Factory2Info;

impl LevelInfo for Factory2Info {
    fn get_floor_info(&self, _cweampuff: &crate::Cweampuff) -> Box<[FloorInfo]> {
        Box::from([
            FloorInfo { position: Vec3::new(-3500.0, 100.0, 1.0), size: Vec2::new(300.0, 2800.0), breakable_wall: None, floor_asset: FloorAssetType::Factory },
            FloorInfo { position: Vec3::new(-500.0, -1650.0, 1.0), size: Vec2::new(6000.0, 300.0), breakable_wall: None, floor_asset: FloorAssetType::Factory },
            FloorInfo { position: Vec3::new(0., 1300.0, 1.0), size: Vec2::new(6700.0, 300.0), breakable_wall: None, floor_asset: FloorAssetType::Factory },
            FloorInfo { position: Vec3::new(3500.0, -150.0, 1.0), size: Vec2::new(300.0, 3300.0), breakable_wall: None, floor_asset: FloorAssetType::Factory },
            FloorInfo { position: Vec3::new(2650.0, -500.0, 1.0), size: Vec2::new(300.0, 2600.0), breakable_wall: None, floor_asset: FloorAssetType::Factory },
            FloorInfo { position: Vec3::new(-1350.0, 100.0, 1.0), size: Vec2::new(4000.0, 300.0), breakable_wall: None, floor_asset: FloorAssetType::Factory },
            FloorInfo { position: Vec3::new(1575.0, -375.0, 1.0), size: Vec2::new(1850.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Factory },
            FloorInfo { position: Vec3::new(1800.0, 750.0, 1.0), size: Vec2::new(1400.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Factory },
            FloorInfo { position: Vec3::new(-2400.0, -450.0, 1.0), size: Vec2::new(150.0, 800.0), breakable_wall: None, floor_asset: FloorAssetType::Factory },
            FloorInfo { position: Vec3::new(-1700.0, -1300.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Factory },
            FloorInfo { position: Vec3::new(-2000.0, -1000.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Factory },
            FloorInfo { position: Vec3::new(-1450.0, -450.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Factory },
            FloorInfo { position: Vec3::new(-500.0, -800.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Factory },
            FloorInfo { position: Vec3::new(-500.0, -300.0, 1.0), size: Vec2::new(150.0, 500.0), breakable_wall: None, floor_asset: FloorAssetType::Factory },
            FloorInfo { position: Vec3::new(200.0, -300.0, 1.0), size: Vec2::new(150.0, 500.0), breakable_wall: None, floor_asset: FloorAssetType::Factory },
            FloorInfo { position: Vec3::new(-2600.0, 450.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Factory },
            FloorInfo { position: Vec3::new(-1900.0, 700.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Factory },
            FloorInfo { position: Vec3::new(-1500.0, 900.0, 1.0), size: Vec2::new(150.0, 500.0), breakable_wall: None, floor_asset: FloorAssetType::Factory },
            FloorInfo { position: Vec3::new(-1100.0, 700.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Factory },
            FloorInfo { position: Vec3::new(-300.0, 900.0, 1.0), size: Vec2::new(150.0, 500.0), breakable_wall: None, floor_asset: FloorAssetType::Factory },
            FloorInfo { position: Vec3::new(600.0, 900.0, 1.0), size: Vec2::new(150.0, 500.0), breakable_wall: None, floor_asset: FloorAssetType::Factory },
        ])
    }

    fn get_transitions_info(&self, _cweampuff: &crate::Cweampuff) -> Option<Box<[TransitionCollider]>> {
        Some(Box::from([
            TransitionCollider { exit_index: 1, safe_position: Vec3::new(-3450.0, -1450.0, CWEAMPUFF_Z_INDEX), transition_to_level: Level::Factory1(Factory1Info), floor_info: EntityInfo { position: Vec3::new(-3600.0, -1400.0, 2.0), size: Vec2::new(100.0, 200.0) }  },
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
}