use bevy::math::{Vec2, Vec3};

use crate::{level::Level, npc::NPC, CWEAMPUFF_Z_INDEX};

use super::{factory_3_layout::Factory3Info, neuro_lair_layout::NeuroLairInfo, BreakableWall, DoorCollider, EntityInfo, FloorAssetType, FloorInfo, FloorModification, LevelInfo, TimeTrial, TransitionCollider};

#[derive(Clone, Copy)]
pub struct Factory4Info;

impl LevelInfo for Factory4Info {
    fn get_floor_info(&self, _cweampuff: &crate::Cweampuff) -> Box<[FloorInfo]> {
        Box::from([
            FloorInfo { position: Vec3::new(-1850.0, 250.0, 1.0), size: Vec2::new(300.0, 6500.0), breakable_wall: None, floor_asset: FloorAssetType::Factory },
            FloorInfo { position: Vec3::new(-500.0, 3850.0, 1.0), size: Vec2::new(3000.0, 300.0), breakable_wall: None, floor_asset: FloorAssetType::Factory },
            FloorInfo { position: Vec3::new(1150.0, 750.0, 1.0), size: Vec2::new(300.0, 6500.0), breakable_wall: None, floor_asset: FloorAssetType::Factory },
            FloorInfo { position: Vec3::new(-200.0, -2850.0, 1.0), size: Vec2::new(3000.0, 300.0), breakable_wall: None, floor_asset: FloorAssetType::Factory },
            FloorInfo { position: Vec3::new(-1250.0, 3450.0, 1.0), size: Vec2::new(900.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Factory },
        ])
    }

    fn get_transitions_info(&self, _cweampuff: &crate::Cweampuff) -> Option<Box<[TransitionCollider]>> {
        Some(Box::from([
            TransitionCollider { exit_index: 1, safe_position: Vec3::new(1100.0, -2650.0, CWEAMPUFF_Z_INDEX), transition_to_level: Level::Factory3(Factory3Info), floor_info: EntityInfo { position: Vec3::new(1250.0, -2600.0, 2.0), size: Vec2::new(100.0, 200.0) }  },
            TransitionCollider { exit_index: 0, safe_position: Vec3::new(-1800.0, 3550.0, CWEAMPUFF_Z_INDEX), transition_to_level: Level::NeuroLair(NeuroLairInfo), floor_info: EntityInfo { position: Vec3::new(-1950.0, 3600.0, 2.0), size: Vec2::new(100.0, 200.0) }  },
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
            FloorModification::TimeTrial(
                TimeTrial { lever_info: EntityInfo { position: Vec3::new(0.0, -2600.0, 0.0), size: Vec2::new(100.0, 200.0) }, seconds_to_complete: 10, id: 1, is_active: false,
                floor_infos: TIME_TRIAL_1
            }),
            FloorModification::TimeTrial(
                TimeTrial { lever_info: EntityInfo { position: Vec3::new(0.0, -400.0, 0.0), size: Vec2::new(100.0, 200.0) }, seconds_to_complete: 8, id: 2, is_active: false,
                floor_infos: TIME_TRIAL_2
            }),
            FloorModification::TimeTrial(
                TimeTrial { lever_info: EntityInfo { position: Vec3::new(-200.0, 1550.0, 0.0), size: Vec2::new(100.0, 200.0) }, seconds_to_complete: 10, id: 3, is_active: false,
                floor_infos: TIME_TRIAL_3
            }),
        ]))
    }

    fn get_bgm(&self) -> Option<&'static str> {
        Some("factory")
    }

    fn get_background(&self) -> FloorAssetType {
        FloorAssetType::Factory
    }
}

static TIME_TRIAL_1: &[FloorInfo] = &[
    FloorInfo { position: Vec3::new(200.0, -2400.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: Some(BreakableWall { index: 1 }), floor_asset: FloorAssetType::Factory },
    FloorInfo { position: Vec3::new(-400.0, -2200.0, 1.0), size: Vec2::new(150.0, 300.0), breakable_wall: Some(BreakableWall { index: 1 }), floor_asset: FloorAssetType::Factory },
    FloorInfo { position: Vec3::new(-1100.0, -1950.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: Some(BreakableWall { index: 1 }), floor_asset: FloorAssetType::Factory },
    FloorInfo { position: Vec3::new(-1100.0, -1650.0, 1.0), size: Vec2::new(800.0, 100.0), breakable_wall: Some(BreakableWall { index: 1 }), floor_asset: FloorAssetType::Factory },
    FloorInfo { position: Vec3::new(-1550.0, -1150.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: Some(BreakableWall { index: 1 }), floor_asset: FloorAssetType::Factory },
    FloorInfo { position: Vec3::new(-700.0, -1000.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: Some(BreakableWall { index: 1 }), floor_asset: FloorAssetType::Factory },
    FloorInfo { position: Vec3::new(0.0, -700.0, 1.0), size: Vec2::new(150.0, 400.0), breakable_wall: Some(BreakableWall { index: 1 }), floor_asset: FloorAssetType::Factory },
];

static TIME_TRIAL_2: &[FloorInfo] = &[
    FloorInfo { position: Vec3::new(600.0, 0.0, 1.0), size: Vec2::new(150.0, 400.0), breakable_wall: Some(BreakableWall { index: 2 }), floor_asset: FloorAssetType::Factory },
    FloorInfo { position: Vec3::new(-600.0, 0.0, 1.0), size: Vec2::new(150.0, 400.0), breakable_wall: Some(BreakableWall { index: 2 }), floor_asset: FloorAssetType::Factory },
    FloorInfo { position: Vec3::new(850.0, 500.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: Some(BreakableWall { index: 2 }), floor_asset: FloorAssetType::Factory },
    FloorInfo { position: Vec3::new(-1250.0, 500.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: Some(BreakableWall { index: 2 }), floor_asset: FloorAssetType::Factory },
    FloorInfo { position: Vec3::new(600.0, 1000.0, 1.0), size: Vec2::new(150.0, 400.0), breakable_wall: Some(BreakableWall { index: 2 }), floor_asset: FloorAssetType::Factory },
    FloorInfo { position: Vec3::new(-600.0, 1000.0, 1.0), size: Vec2::new(150.0, 400.0), breakable_wall: Some(BreakableWall { index: 2 }), floor_asset: FloorAssetType::Factory },
    FloorInfo { position: Vec3::new(-200.0, 1400.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: Some(BreakableWall { index: 2 }), floor_asset: FloorAssetType::Factory },
];

static TIME_TRIAL_3: &[FloorInfo] = &[
    FloorInfo { position: Vec3::new(-600.0, 1700.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: Some(BreakableWall { index: 3 }), floor_asset: FloorAssetType::Factory },
    FloorInfo { position: Vec3::new(-600.0, 2000.0, 1.0), size: Vec2::new(800.0, 100.0), breakable_wall: Some(BreakableWall { index: 3 }), floor_asset: FloorAssetType::Factory },
    FloorInfo { position: Vec3::new(300.0, 2500.0, 1.0), size: Vec2::new(150.0, 400.0), breakable_wall: Some(BreakableWall { index: 3 }), floor_asset: FloorAssetType::Factory },
    FloorInfo { position: Vec3::new(600.0, 2800.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: Some(BreakableWall { index: 3 }), floor_asset: FloorAssetType::Factory },
    FloorInfo { position: Vec3::new(600.0, 3300.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: Some(BreakableWall { index: 3 }), floor_asset: FloorAssetType::Factory },
    FloorInfo { position: Vec3::new(0.0, 3500.0, 1.0), size: Vec2::new(150.0, 400.0), breakable_wall: Some(BreakableWall { index: 3 }), floor_asset: FloorAssetType::Factory },
    FloorInfo { position: Vec3::new(-600.0, 3300.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: Some(BreakableWall { index: 3 }), floor_asset: FloorAssetType::Factory },
];