use bevy::math::{Vec2, Vec3};

use crate::{level::{level_layout::{hell_1_layout::Hell1Info, hell_3_layout::Hell3Info}, Level}, npc::NPC, CWEAMPUFF_Z_INDEX};

use super::{DoorCollider, EntityInfo, FloorAssetType, FloorInfo, FloorModification, LevelInfo, TransitionCollider};

#[derive(Clone, Copy)]
pub struct Hell2Info;

impl LevelInfo for Hell2Info {
    fn get_floor_info(&self, _cweampuff: &crate::Cweampuff) -> Box<[FloorInfo]> {
        Box::from([
            FloorInfo { position: Vec3::new(-150.0, 1600.0, 1.0), size: Vec2::new(300.0, 300.0), breakable_wall: None, floor_asset: FloorAssetType::Hell },
            FloorInfo { position: Vec3::new(2100.0, 1600.0, 1.0), size: Vec2::new(3600.0, 300.0), breakable_wall: None, floor_asset: FloorAssetType::Hell },
            FloorInfo { position: Vec3::new(4050.0, 700.0, 1.0), size: Vec2::new(300.0, 2100.0), breakable_wall: None, floor_asset: FloorAssetType::Hell },
            FloorInfo { position: Vec3::new(-450.0, 700.0, 1.0), size: Vec2::new(300.0, 2100.0), breakable_wall: None, floor_asset: FloorAssetType::Hell },
            FloorInfo { position: Vec3::new(1400.0, -200.0, 1.0), size: Vec2::new(3400.0, 300.0), breakable_wall: None, floor_asset: FloorAssetType::Hell },
            FloorInfo { position: Vec3::new(3250.0, 350.0, 1.0), size: Vec2::new(300.0, 1500.0), breakable_wall: None, floor_asset: FloorAssetType::Hell },
            FloorInfo { position: Vec3::new(900.0, 275.0, 1.0), size: Vec2::new(500.0, 650.0), breakable_wall: None, floor_asset: FloorAssetType::Hell },
            FloorInfo { position: Vec3::new(200.0, 200.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Hell },
            FloorInfo { position: Vec3::new(-225.0, 500.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Hell },
            FloorInfo { position: Vec3::new(300.0, 850.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Hell },
            FloorInfo { position: Vec3::new(1500.0, 400.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Hell },
            FloorInfo { position: Vec3::new(1800.0, 100.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Hell },
            FloorInfo { position: Vec3::new(700.0, 1100.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Hell },
            FloorInfo { position: Vec3::new(1050.0, 1300.0, 1.0), size: Vec2::new(150.0, 300.0), breakable_wall: None, floor_asset: FloorAssetType::Hell },
            FloorInfo { position: Vec3::new(1400.0, 1100.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Hell },
            FloorInfo { position: Vec3::new(1750.0, 1300.0, 1.0), size: Vec2::new(150.0, 300.0), breakable_wall: None, floor_asset: FloorAssetType::Hell },
            FloorInfo { position: Vec3::new(2100.0, 1100.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Hell },
            FloorInfo { position: Vec3::new(2450.0, 1300.0, 1.0), size: Vec2::new(150.0, 300.0), breakable_wall: None, floor_asset: FloorAssetType::Hell },
            FloorInfo { position: Vec3::new(2800.0, 1100.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Hell },
        ])
    }

    fn get_transitions_info(&self, _cweampuff: &crate::Cweampuff) -> Option<Box<[TransitionCollider]>> {
        Some(Box::from([
            TransitionCollider { exit_index: 0, safe_position: Vec3::new(150.0, 1500.0, CWEAMPUFF_Z_INDEX), transition_to_level: Level::Hell1(Hell1Info), floor_info: EntityInfo { position: Vec3::new(150.0, 1700.0, 2.0), size: Vec2::new(300.0, 200.0) }  },
            TransitionCollider { exit_index: 1, safe_position: Vec3::new(150.0, 1500.0, CWEAMPUFF_Z_INDEX), transition_to_level: Level::Hell3(Hell3Info), floor_info: EntityInfo { position: Vec3::new(3650.0, -350.0, 2.0), size: Vec2::new(500.0, 200.0) }  }
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
        Some("hell")
    }

    fn get_background(&self) -> FloorAssetType {
        FloorAssetType::Hell
    }
}