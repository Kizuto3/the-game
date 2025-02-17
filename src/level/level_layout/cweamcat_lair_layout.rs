use bevy::math::{Vec2, Vec3};

use crate::{level::Level, npc::NPC};

use super::{FloorInfo, LevelInfo, TransitionCollider};

#[derive(Clone, Copy)]
pub struct CweamcatLairInfo;

impl LevelInfo for CweamcatLairInfo {
    fn get_floor_info(&self, _cweampuff: &crate::Cweampuf) -> Vec<FloorInfo> {
        vec![
            FloorInfo { position: Vec3::new(-450.0, 550.0, 1.0), size: Vec2::new(100.0, 1400.0) },
            FloorInfo { position: Vec3::new(500.0, -400.0, 1.0), size: Vec2::new(2000.0, 500.0) },
            FloorInfo { position: Vec3::new(2000.0, -200.0, 1.0), size: Vec2::new(1000.0, 900.0) },
            FloorInfo { position: Vec3::new(2650.0, 4000.0, 1.0), size: Vec2::new(300.0, 1600.0) },
        ]
    }

    fn get_transitions_info(&self, _cweampuff: &crate::Cweampuf) -> Vec<TransitionCollider> {
        vec![
            TransitionCollider { exit_index: 0, safe_position: Vec3::new(0.0, 850.0, 1.0), transition_to_level: Level::CweamcatLair(CweamcatLairInfo), floor_info: FloorInfo { position: Vec3::new(2700.0, 2700.0, 1.0), size: Vec2::new(100.0, 200.0) }  }
        ]
    }

    fn get_npcs(&self, _cweampuff: &crate::Cweampuf) -> Vec<NPC> {
        vec![]
    }
}