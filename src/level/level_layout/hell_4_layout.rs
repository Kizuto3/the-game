use bevy::math::{Vec2, Vec3};

use crate::{level::{level_layout::JumpPad, Level}, npc::NPC};

use super::{cerber_lair_layout::CerberLairInfo, hell_3_layout::Hell3Info, DoorCollider, EntityInfo, FloorInfo, FloorModification, LevelInfo, TransitionCollider};

#[derive(Clone, Copy)]
pub struct Hell4Info;

impl LevelInfo for Hell4Info {
    fn get_floor_info(&self, _cweampuff: &crate::Cweampuff) -> Box<[FloorInfo]> {
        Box::from([
            FloorInfo { position: Vec3::new(-1500.0, -200.0, 1.0), size: Vec2::new(300.0, 5800.0), breakable_wall: None },
            FloorInfo { position: Vec3::new(1500.0, 200.0, 1.0), size: Vec2::new(300.0, 5800.0), breakable_wall: None },
            FloorInfo { position: Vec3::new(0.0, -3050.0, 1.0), size: Vec2::new(3300.0, 300.0), breakable_wall: None },
            FloorInfo { position: Vec3::new(0.0, 3050.0, 1.0), size: Vec2::new(3300.0, 300.0), breakable_wall: None },
            FloorInfo { position: Vec3::new(0.0, -2650.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None },
            FloorInfo { position: Vec3::new(400.0, -2350.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None },
            FloorInfo { position: Vec3::new(400.0, -2050.0, 1.0), size: Vec2::new(600.0, 100.0), breakable_wall: None },
            FloorInfo { position: Vec3::new(-400.0, -1750.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None },
            FloorInfo { position: Vec3::new(0.0, -1450.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None },
            FloorInfo { position: Vec3::new(1050.0, -1550.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None },
            FloorInfo { position: Vec3::new(1275.0, -1250.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None },
            FloorInfo { position: Vec3::new(700.0, -950.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None },
            FloorInfo { position: Vec3::new(700.0, -650.0, 1.0), size: Vec2::new(600.0, 100.0), breakable_wall: None },
            FloorInfo { position: Vec3::new(-300.0, -450.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None },
            FloorInfo { position: Vec3::new(-1000.0, -150.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None },
            FloorInfo { position: Vec3::new(-1000.0, 150.0, 1.0), size: Vec2::new(700.0, 100.0), breakable_wall: None },
            FloorInfo { position: Vec3::new(-500.0, 400.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None },
            FloorInfo { position: Vec3::new(-150.0, 750.0, 1.0), size: Vec2::new(150.0, 600.0), breakable_wall: None },
            FloorInfo { position: Vec3::new(200.0, 400.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None },
            FloorInfo { position: Vec3::new(700.0, 275.0, 1.0), size: Vec2::new(200.0, 100.0), breakable_wall: None },
            FloorInfo { position: Vec3::new(200.0, 1050.0, 1.0), size: Vec2::new(200.0, 100.0), breakable_wall: None },
            FloorInfo { position: Vec3::new(600.0, 1650.0, 1.0), size: Vec2::new(200.0, 100.0), breakable_wall: None },
            FloorInfo { position: Vec3::new(200.0, 2550.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None },
            FloorInfo { position: Vec3::new(-150.0, 2750.0, 1.0), size: Vec2::new(150.0, 300.0), breakable_wall: None },
            FloorInfo { position: Vec3::new(-500.0, 2550.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None },
        ])
    }

    fn get_transitions_info(&self, _cweampuff: &crate::Cweampuff) -> Option<Box<[TransitionCollider]>> {
        Some(Box::from([
            TransitionCollider { exit_index: 0, safe_position: Vec3::new(1550.0, -2850.0, 1.0), transition_to_level: Level::Hell3(Hell3Info), floor_info: EntityInfo { position: Vec3::new(1700.0, -2800.0, 2.0), size: Vec2::new(100.0, 200.0) }  },
            TransitionCollider { exit_index: 1, safe_position: Vec3::new(-1450.0, 2750.0, 1.0), transition_to_level: Level::CerberLair(CerberLairInfo), floor_info: EntityInfo { position: Vec3::new(-1600.0, 2800.0, 2.0), size: Vec2::new(100.0, 200.0) }  }
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
            FloorModification::JumpPad(JumpPad { floor_info: EntityInfo { position: Vec3::new(700.0, 400.0, 0.0), size: Vec2::new(200.0, 200.0) } }),
            FloorModification::JumpPad(JumpPad { floor_info: EntityInfo { position: Vec3::new(200.0, 1150.0, 0.0), size: Vec2::new(200.0, 200.0) } }),
            FloorModification::JumpPad(JumpPad { floor_info: EntityInfo { position: Vec3::new(600.0, 1800.0, 0.0), size: Vec2::new(200.0, 200.0) } }),
        ]))
    }
}