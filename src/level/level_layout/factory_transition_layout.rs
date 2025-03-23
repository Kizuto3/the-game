use bevy::math::{Vec2, Vec3};

use crate::{level::{progression::Progression, Level}, npc::NPC, CWEAMPUFF_Z_INDEX};

use super::{cweamcat_lair_layout::CweamcatLairInfo, factory_1_layout::Factory1Info, neuro_lair_layout::NeuroLairInfo, DoorCollider, EntityInfo, FloorAssetType, FloorInfo, FloorModification, LevelInfo, TransitionCollider};

#[derive(Clone, Copy)]
pub struct FactoryTransitionInfo;

impl LevelInfo for FactoryTransitionInfo {
    fn get_floor_info(&self, cweampuff: &crate::Cweampuff) -> Box<[FloorInfo]> {
        let mut floors = vec![
            FloorInfo { position: Vec3::new(-1000.0, 0.0, 1.0), size: Vec2::new(300.0, 1000.0), breakable_wall: None, floor_asset: FloorAssetType::Forest },
            FloorInfo { position: Vec3::new(-1000.0, -1100.0, 1.0), size: Vec2::new(300.0, 500.0), breakable_wall: None, floor_asset: FloorAssetType::Forest },
            FloorInfo { position: Vec3::new(350.0, 350.0, 1.0), size: Vec2::new(2400.0, 300.0), breakable_wall: None, floor_asset: FloorAssetType::Forest },
            FloorInfo { position: Vec3::new(550.0, -1200.0, 1.0), size: Vec2::new(2800.0, 300.0), breakable_wall: None, floor_asset: FloorAssetType::Forest },
            FloorInfo { position: Vec3::new(1150.0, -200.0, 1.0), size: Vec2::new(600.0, 500.0), breakable_wall: None, floor_asset: FloorAssetType::Forest },
            FloorInfo { position: Vec3::new(0.0, -650.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Forest },
            FloorInfo { position: Vec3::new(500.0, -350.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Forest },
            FloorInfo { position: Vec3::new(0.0, -50.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Forest },
        ];
        
        if cweampuff.progression <= Progression::GivenLetter {
            floors.push(
                FloorInfo { position: Vec3::new(1200.0, -750.0, 1.0), size: Vec2::new(300.0, 600.0), breakable_wall: None, floor_asset: FloorAssetType::Factory },
            );
        }

        floors.into_boxed_slice()
    }

    fn get_transitions_info(&self, cweampuff: &crate::Cweampuff) -> Option<Box<[TransitionCollider]>> {
        let mut transitions = vec![
            TransitionCollider { exit_index: 4, safe_position: Vec3::new(-950.0, -750.0, CWEAMPUFF_Z_INDEX), transition_to_level: Level::CweamcatLair(CweamcatLairInfo), floor_info: EntityInfo { position: Vec3::new(-1100.0, -675.0, 2.0), size: Vec2::new(100.0, 350.0) }  },
            TransitionCollider { exit_index: 0, safe_position: Vec3::new(1150.0, 100.0, CWEAMPUFF_Z_INDEX), transition_to_level: Level::Factory1(Factory1Info), floor_info: EntityInfo { position: Vec3::new(1250.0, 150.0, 2.0), size: Vec2::new(100.0, 200.0) }  },
        ];

        if cweampuff.progression > Progression::GivenLetter {
            transitions.push(
                TransitionCollider { exit_index: 1, safe_position: Vec3::new(1150.0, -900.0, CWEAMPUFF_Z_INDEX), transition_to_level: Level::NeuroLair(NeuroLairInfo), floor_info: EntityInfo { position: Vec3::new(1250.0, -750.0, 2.0), size: Vec2::new(100.0, 600.0) }  }
            );
        }

        Some(transitions.into_boxed_slice())
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