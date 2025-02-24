use bevy::math::{Vec2, Vec3};

use crate::{level::{level_layout::hell_1_layout::Hell1Info, Level}, npc::NPC};

use super::{DoorCollider, FloorInfo, LevelInfo, TransitionCollider};

#[derive(Clone, Copy)]
pub struct Hell2Info;

impl LevelInfo for Hell2Info {
    fn get_floor_info(&self, _cweampuff: &crate::Cweampuf) -> Vec<FloorInfo> {
        vec![
            FloorInfo { position: Vec3::new(-300.0, 1600.0, 1.0), size: Vec2::new(600.0, 300.0) },
            FloorInfo { position: Vec3::new(2100.0, 1600.0, 1.0), size: Vec2::new(3600.0, 300.0) },
            FloorInfo { position: Vec3::new(4050.0, 700.0, 1.0), size: Vec2::new(300.0, 2100.0) },
            FloorInfo { position: Vec3::new(-450.0, 700.0, 1.0), size: Vec2::new(300.0, 2100.0) },
            FloorInfo { position: Vec3::new(1550.0, -200.0, 1.0), size: Vec2::new(3700.0, 300.0) },
            FloorInfo { position: Vec3::new(3250.0, 350.0, 1.0), size: Vec2::new(300.0, 1500.0) },
            FloorInfo { position: Vec3::new(900.0, 275.0, 1.0), size: Vec2::new(500.0, 650.0) },
            FloorInfo { position: Vec3::new(200.0, 200.0, 1.0), size: Vec2::new(150.0, 100.0) },
            FloorInfo { position: Vec3::new(-250.0, 500.0, 1.0), size: Vec2::new(150.0, 100.0) },
            FloorInfo { position: Vec3::new(300.0, 850.0, 1.0), size: Vec2::new(150.0, 100.0) },
            FloorInfo { position: Vec3::new(1500.0, 400.0, 1.0), size: Vec2::new(150.0, 100.0) },
            FloorInfo { position: Vec3::new(1800.0, 100.0, 1.0), size: Vec2::new(150.0, 100.0) },
            FloorInfo { position: Vec3::new(700.0, 1100.0, 1.0), size: Vec2::new(150.0, 100.0) },
            FloorInfo { position: Vec3::new(1050.0, 1300.0, 1.0), size: Vec2::new(150.0, 300.0) },
            FloorInfo { position: Vec3::new(1400.0, 1100.0, 1.0), size: Vec2::new(150.0, 100.0) },
            FloorInfo { position: Vec3::new(1750.0, 1300.0, 1.0), size: Vec2::new(150.0, 300.0) },
            FloorInfo { position: Vec3::new(2100.0, 1100.0, 1.0), size: Vec2::new(150.0, 100.0) },
            FloorInfo { position: Vec3::new(2450.0, 1300.0, 1.0), size: Vec2::new(150.0, 300.0) },
            FloorInfo { position: Vec3::new(2800.0, 1100.0, 1.0), size: Vec2::new(150.0, 100.0) },
        ]
    }

    fn get_transitions_info(&self, _cweampuff: &crate::Cweampuf) -> Vec<TransitionCollider> {
        vec![
            TransitionCollider { exit_index: 0, safe_position: Vec3::new(150.0, 1500.0, 1.0), transition_to_level: Level::Hell1(Hell1Info), floor_info: FloorInfo { position: Vec3::new(150.0, 1700.0, 2.0), size: Vec2::new(300.0, 200.0) }  }
        ]
    }

    fn get_npcs(&self, _cweampuff: &crate::Cweampuf) -> Vec<NPC> {        
        vec![]
    }
    
    fn get_doors(&self, _cweampuff: &crate::Cweampuf) -> Vec<DoorCollider> {
        vec![]
    }
}