use bevy::math::{Vec2, Vec3};

use crate::{level::{level_layout::{hell_2_layout::Hell2Info, JumpPad}, Level}, npc::NPC};

use super::{DoorCollider, FloorInfo, FloorModification, LevelInfo, TransitionCollider};

#[derive(Clone, Copy)]
pub struct Hell3Info;

impl LevelInfo for Hell3Info {
    fn get_floor_info(&self, _cweampuff: &crate::Cweampuff) -> Box<[FloorInfo]> {
        Box::from([
            FloorInfo { position: Vec3::new(-3000.0, 450.0, 1.0), size: Vec2::new(300.0, 2800.0) },
            FloorInfo { position: Vec3::new(3000.0, 0.0, 1.0), size: Vec2::new(300.0, 3000.0) },
            FloorInfo { position: Vec3::new(-300.0, 1350.0, 1.0), size: Vec2::new(5100.0, 300.0) },
            FloorInfo { position: Vec3::new(-50.0, -1350.0, 1.0), size: Vec2::new(6200.0, 300.0) },
            FloorInfo { position: Vec3::new(2600.0, -350.0, 1.0), size: Vec2::new(500.0, 1700.0) },
            FloorInfo { position: Vec3::new(1550.0, -850.0, 1.0), size: Vec2::new(1600.0, 700.0) },
            FloorInfo { position: Vec3::new(1550.0, 700.0, 1.0), size: Vec2::new(150.0, 100.0) },
            FloorInfo { position: Vec3::new(850.0, -250.0, 1.0), size: Vec2::new(200.0, 500.0) },
            FloorInfo { position: Vec3::new(650.0, -100.0, 1.0), size: Vec2::new(200.0, 200.0) },
            FloorInfo { position: Vec3::new(400.0, 275.0, 1.0), size: Vec2::new(300.0, 950.0) },
            FloorInfo { position: Vec3::new(200.0, 250.0, 1.0), size: Vec2::new(100.0, 100.0) },
            FloorInfo { position: Vec3::new(200.0, 500.0, 1.0), size: Vec2::new(100.0, 100.0) },
            FloorInfo { position: Vec3::new(-290.0, -100.0, 1.0), size: Vec2::new(1080.0, 200.0) },
            FloorInfo { position: Vec3::new(-980.0, 100.0, 1.0), size: Vec2::new(300.0, 600.0) },
            FloorInfo { position: Vec3::new(-2700.0, 100.0, 1.0), size: Vec2::new(300.0, 600.0) },
            FloorInfo { position: Vec3::new(-2500.0, -800.0, 1.0), size: Vec2::new(200.0, 800.0) },
            FloorInfo { position: Vec3::new(-1900.0, -450.0, 1.0), size: Vec2::new(200.0, 1100.0) },
            FloorInfo { position: Vec3::new(-2275.0, -50.0, 1.0), size: Vec2::new(550.0, 300.0) },
            FloorInfo { position: Vec3::new(-1300.0, -950.0, 1.0), size: Vec2::new(1000.0, 100.0) },
            FloorInfo { position: Vec3::new(-700.0, -700.0, 1.0), size: Vec2::new(200.0, 600.0) },
        ])
    }

    fn get_transitions_info(&self, _cweampuff: &crate::Cweampuff) -> Option<Box<[TransitionCollider]>> {
        Some(Box::from([
            TransitionCollider { exit_index: 1, safe_position: Vec3::new(2400.0, 1350.0, 1.0), transition_to_level: Level::Hell2(Hell2Info), floor_info: FloorInfo { position: Vec3::new(2400.0, 1500.0, 2.0), size: Vec2::new(300.0, 200.0) }  }
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
            FloorModification::JumpPad(JumpPad { floor_info: FloorInfo { position: Vec3::new(1300.0, -400.0, 0.0), size: Vec2::new(200.0, 200.0) } }),
            FloorModification::JumpPad(JumpPad { floor_info: FloorInfo { position: Vec3::new(750.0, 100.0, 0.0), size: Vec2::new(200.0, 200.0) } }),
            FloorModification::JumpPad(JumpPad { floor_info: FloorInfo { position: Vec3::new(-2200.0, -1100.0, 0.0), size: Vec2::new(200.0, 200.0) } }),
            FloorModification::JumpPad(JumpPad { floor_info: FloorInfo { position: Vec3::new(-1600.0, -800.0, 0.0), size: Vec2::new(200.0, 200.0) } }),
        ]))
    }
}