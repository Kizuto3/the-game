use bevy::math::{Vec2, Vec3};

use crate::{level::{progression::Progression, Level}, npc::{conversation_entry::{ConversationEntry, ConversationPosition, Emotion}, CWEAMPUFF, MINAWAN, NPC}, CWEAMPUFF_Z_INDEX};

use super::{spaceship_2_layout::Spaceship2Info, spaceship_4_layout::Spaceship4Info, DoorCollider, EntityInfo, FloorAssetType, FloorInfo, FloorModification, GravityInverter, LevelInfo, TransitionCollider};

#[derive(Clone, Copy)]
pub struct Spaceship3Info;

impl LevelInfo for Spaceship3Info {
    fn get_floor_info(&self, _cweampuff: &crate::Cweampuff) -> Box<[FloorInfo]> {
        Box::from([
            FloorInfo { position: Vec3::new(-5000.0, 0.0, 1.0), size: Vec2::new(300.0, 4000.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(-900.0, -1850.0, 1.0), size: Vec2::new(7500.0, 300.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(3000.0, 0.0, 1.0), size: Vec2::new(300.0, 4000.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(-1100.0, 1850.0, 1.0), size: Vec2::new(7500.0, 300.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(-3650.0, -1000.0, 1.0), size: Vec2::new(300.0, 1400.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(-3200.0, -1450.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(-2800.0, -1150.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(-3200.0, -850.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(-2800.0, -550.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(-1000.0, -1000.0, 1.0), size: Vec2::new(300.0, 1400.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(-600.0, 800.0, 1.0), size: Vec2::new(300.0, 1800.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(-600.0, -1450.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(-200.0, -1150.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(1500.0, -750.0, 1.0), size: Vec2::new(300.0, 1900.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(-4200.0, -1450.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(-4700.0, -1150.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(-3875.0, -850.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(-4100.0, -550.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(-4300.0, -100.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(-4200.0, 1450.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(-3700.0, 1150.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(-3700.0, 850.0, 1.0), size: Vec2::new(800.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(-2600.0, 550.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(-1900.0, 350.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(-1000.0, 350.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(700.0, -850.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(0.0, -650.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(600.0, -550.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(600.0, -250.0, 1.0), size: Vec2::new(800.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(0.0, -50.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(2300.0, 800.0, 1.0), size: Vec2::new(300.0, 1800.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(1000.0, 1450.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(500.0, 1150.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(1300.0, 850.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(2075.0, 650.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(1913.0, 350.0, 1.0), size: Vec2::new(474.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(1800.0, -1450.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(2600.0, -1150.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(2400.0, -850.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(2775.0, -550.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(2500.0, -350.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(2775.0, -50.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
        ])
    }

    fn get_transitions_info(&self, _cweampuff: &crate::Cweampuff) -> Option<Box<[TransitionCollider]>> {
        Some(Box::from([
            TransitionCollider { exit_index: 1, safe_position: Vec3::new(-4600.0, -1600.0, CWEAMPUFF_Z_INDEX), transition_to_level: Level::Spaceship2(Spaceship2Info), floor_info: EntityInfo { position: Vec3::new(-4750.0, -2000.0, 2.0), size: Vec2::new(200.0, 200.0) }  },
            TransitionCollider { exit_index: 0, safe_position: Vec3::new(2750.0, 1800.0, CWEAMPUFF_Z_INDEX), transition_to_level: Level::Spaceship4(Spaceship4Info), floor_info: EntityInfo { position: Vec3::new(2750.0, 1900.0, 2.0), size: Vec2::new(200.0, 100.0) }  }
        ]))
    }

    fn get_doors(&self, _cweampuff: &crate::Cweampuff) -> Option<Box<[DoorCollider]>> {
        None
    }

    fn get_npcs(&self, cweampuff: &crate::Cweampuff) -> Option<Box<[NPC]>> {        
        if cweampuff.progression <= Progression::MilkWokeUp {
            return Some(Box::from([
                NPC {
                    floor_info: EntityInfo { position: Vec3::new(-4000.0, -1650.0, 0.0), size: Vec2::new(200.0, 100.0) }, is_active: false, current_conversation_index: 0,
                    after_conversation_func: |_cweampuff, _commands, _breakable_walls, _cutscene| { },
                    conversation: &[
                        ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Minawan? You're here too?", emotion: Emotion::Regular },
                        ConversationEntry { position: ConversationPosition::Right, npc_name: MINAWAN, text: "Wan! Wan! My mission led me here!", emotion: Emotion::Happy },
                        ConversationEntry { position: ConversationPosition::Right, npc_name: MINAWAN, text: "I've notified a few crew members about my rising star's cuteness.", emotion: Emotion::Regular },
                        ConversationEntry { position: ConversationPosition::Right, npc_name: MINAWAN, text: "But I'm having some trouble moving forward.", emotion: Emotion::Regular },
                        ConversationEntry { position: ConversationPosition::Right, npc_name: MINAWAN, text: "Gravity seems all messed up in this room.", emotion: Emotion::Sad },
                        ConversationEntry { position: ConversationPosition::Right, npc_name: MINAWAN, text: "Up is down, down is up...", emotion: Emotion::Regular },
                        ConversationEntry { position: ConversationPosition::Right, npc_name: MINAWAN, text: "Makes my head hurt a little.", emotion: Emotion::Sad },
                        ConversationEntry { position: ConversationPosition::Right, npc_name: MINAWAN, text: "I'm going to rest here for a bit. Be careful, Cweampuff!", emotion: Emotion::Regular },
                        ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Okay! See you, Minawan! Wan, Wan!", emotion: Emotion::Happy },
                        ConversationEntry { position: ConversationPosition::Right, npc_name: MINAWAN, text: "Wan! Wan!", emotion: Emotion::Happy },
                    ]
                }
            ]))
        }
        
        None
    }

    fn get_floor_modifications(&self, _cweampuff: &crate::Cweampuff) -> Option<Box<[FloorModification]>> {
        Some(Box::from([
            FloorModification::GravityInverter(GravityInverter { floor_info: EntityInfo { position: Vec3 { x: -1100., y: 1000., z: 0. }, size: Vec2 { x: 7500., y: 1700. } } })
        ]))
    }

    fn get_bgm(&self) -> Option<&'static str> {
        Some("spaceship")
    }
}