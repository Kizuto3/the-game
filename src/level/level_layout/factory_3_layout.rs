use bevy::math::{Vec2, Vec3};

use crate::{level::{progression::Progression, Level}, npc::{conversation_entry::{ConversationEntry, ConversationPosition, Emotion}, CWEAMPUFF, MINAWAN, NPC}, CWEAMPUFF_Z_INDEX};

use super::{factory_2_layout::Factory2Info, factory_4_layout::Factory4Info, BreakableWall, DoorCollider, EntityInfo, FloorAssetType, FloorInfo, FloorModification, LevelInfo, TimeTrial, TransitionCollider};

#[derive(Clone, Copy)]
pub struct Factory3Info;

impl LevelInfo for Factory3Info {
    fn get_floor_info(&self, _cweampuff: &crate::Cweampuff) -> Box<[FloorInfo]> {
        Box::from([
            FloorInfo { position: Vec3::new(-1850.0, 175.0, 1.0), size: Vec2::new(300.0, 5950.0), breakable_wall: None, floor_asset: FloorAssetType::Factory },
            FloorInfo { position: Vec3::new(-125.0, 3000.0, 1.0), size: Vec2::new(3150.0, 300.0), breakable_wall: None, floor_asset: FloorAssetType::Factory },
            FloorInfo { position: Vec3::new(0.0, -3150.0, 1.0), size: Vec2::new(4000.0, 300.0), breakable_wall: None, floor_asset: FloorAssetType::Factory },
            FloorInfo { position: Vec3::new(2150.0, 0.0, 1.0), size: Vec2::new(300.0, 6300.0), breakable_wall: None, floor_asset: FloorAssetType::Factory },
            FloorInfo { position: Vec3::new(500.0, 1000.0, 1.0), size: Vec2::new(3000.0, 300.0), breakable_wall: None, floor_asset: FloorAssetType::Factory },
            FloorInfo { position: Vec3::new(-1150.0, 1600.0, 1.0), size: Vec2::new(300.0, 1500.0), breakable_wall: None, floor_asset: FloorAssetType::Factory },
            FloorInfo { position: Vec3::new(-500.0, 2275.0, 1.0), size: Vec2::new(1000.0, 150.0), breakable_wall: None, floor_asset: FloorAssetType::Factory },
            FloorInfo { position: Vec3::new(-200.0, -1000.0, 1.0), size: Vec2::new(3000.0, 300.0), breakable_wall: None, floor_asset: FloorAssetType::Factory },
            FloorInfo { position: Vec3::new(1450.0, -400.0, 1.0), size: Vec2::new(300.0, 1500.0), breakable_wall: None, floor_asset: FloorAssetType::Factory },
            FloorInfo { position: Vec3::new(800.0, 275.0, 1.0), size: Vec2::new(1000.0, 150.0), breakable_wall: None, floor_asset: FloorAssetType::Factory },
            FloorInfo { position: Vec3::new(-1150.0, -2250.0, 1.0), size: Vec2::new(300.0, 1500.0), breakable_wall: None, floor_asset: FloorAssetType::Factory },
            FloorInfo { position: Vec3::new(-500.0, -1575.0, 1.0), size: Vec2::new(1000.0, 150.0), breakable_wall: None, floor_asset: FloorAssetType::Factory },
        ])
    }

    fn get_transitions_info(&self, _cweampuff: &crate::Cweampuff) -> Option<Box<[TransitionCollider]>> {
        Some(Box::from([
            TransitionCollider { exit_index: 0, safe_position: Vec3::new(1725.0, 2950.0, CWEAMPUFF_Z_INDEX), transition_to_level: Level::Factory2(Factory2Info), floor_info: EntityInfo { position: Vec3::new(1725.0, 3100.0, 2.0), size: Vec2::new(550.0, 100.0) }  },
            TransitionCollider { exit_index: 1, safe_position: Vec3::new(-1800.0, -2950.0, CWEAMPUFF_Z_INDEX), transition_to_level: Level::Factory4(Factory4Info), floor_info: EntityInfo { position: Vec3::new(-1950.0, -2900.0, 2.0), size: Vec2::new(100.0, 200.0) }  },
        ]))
    }

    fn get_doors(&self, _cweampuff: &crate::Cweampuff) -> Option<Box<[DoorCollider]>> {
        None
    }

    fn get_npcs(&self, cweampuff: &crate::Cweampuff) -> Option<Box<[NPC]>> {                
        if cweampuff.progression <= Progression::GivenLetter {
            return Some(Box::from([
                NPC {
                    floor_info: EntityInfo { position: Vec3::new(1600.0, 1200.0, 0.0), size: Vec2::new(200.0, 100.0) }, is_active: false, current_conversation_index: 0,
                    after_conversation_func: |_cweampuff, _commands, _breakable_walls, _cutscene| { },
                    conversation: &[
                        ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Minawan! Wan, wan!", emotion: Emotion::Happy },
                        ConversationEntry { position: ConversationPosition::Right, npc_name: MINAWAN, text: "Wan! Wan!", emotion: Emotion::Happy },
                        ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Did you come here to fulfill your mission?", emotion: Emotion::Regular },
                        ConversationEntry { position: ConversationPosition::Right, npc_name: MINAWAN, text: "Not really, no. I'm just visiting Swarm.", emotion: Emotion::Regular },
                        ConversationEntry { position: ConversationPosition::Right, npc_name: MINAWAN, text: "Minawan and Swarm are very good friends!", emotion: Emotion::Happy },
                        ConversationEntry { position: ConversationPosition::Right, npc_name: MINAWAN, text: "They can seem intimidating at first, but they're all really nice!", emotion: Emotion::Happy },
                        ConversationEntry { position: ConversationPosition::Right, npc_name: MINAWAN, text: "However, seems like this place changed a bit since I've been here last time.", emotion: Emotion::Regular },
                        ConversationEntry { position: ConversationPosition::Right, npc_name: MINAWAN, text: "Swarm installed these things that make platforms appear.", emotion: Emotion::Regular },
                        ConversationEntry { position: ConversationPosition::Right, npc_name: MINAWAN, text: "But they disappear too fast!", emotion: Emotion::Sad },
                        ConversationEntry { position: ConversationPosition::Right, npc_name: MINAWAN, text: "I can't jump through all of them fast enough!", emotion: Emotion::Sad },
                        ConversationEntry { position: ConversationPosition::Right, npc_name: MINAWAN, text: "When I was struggling on the spaceship I just copied what you did.", emotion: Emotion::Regular },
                        ConversationEntry { position: ConversationPosition::Right, npc_name: MINAWAN, text: "So how about you show me how it's done here too?", emotion: Emotion::Happy },
                        ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Okay! Watch and learn, Minawan! Wan, Wan!", emotion: Emotion::Happy },
                        ConversationEntry { position: ConversationPosition::Right, npc_name: MINAWAN, text: "Wan! Wan!", emotion: Emotion::Happy },
                    ]
                }
            ]))
        }
        
        None
    }

    fn get_floor_modifications(&self, _cweampuff: &crate::Cweampuff) -> Option<Box<[FloorModification]>> {
        Some(Box::from([
            FloorModification::TimeTrial(
                TimeTrial { lever_info: EntityInfo { position: Vec3::new(-600.0, 1250.0, 0.0), size: Vec2::new(100.0, 200.0) }, seconds_to_complete: 6, id: 1, is_active: false,
                floor_infos: TIME_TRIAL_1
            }),
            FloorModification::TimeTrial(
                TimeTrial { lever_info: EntityInfo { position: Vec3::new(900.0, -750.0, 0.0), size: Vec2::new(100.0, 200.0) }, seconds_to_complete: 7, id: 2, is_active: false,
                floor_infos: TIME_TRIAL_2
            }),
            FloorModification::TimeTrial(
                TimeTrial { lever_info: EntityInfo { position: Vec3::new(-600.0, -2900.0, 0.0), size: Vec2::new(100.0, 200.0) }, seconds_to_complete: 8, id: 3, is_active: false,
                floor_infos: TIME_TRIAL_3
            }),
        ]))
    }
}

static TIME_TRIAL_1: &[FloorInfo] = &[
    FloorInfo { position: Vec3::new(-100.0, 1450.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: Some(BreakableWall { index: 1 }), floor_asset: FloorAssetType::Factory },
    FloorInfo { position: Vec3::new(700.0, 1650.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: Some(BreakableWall { index: 1 }), floor_asset: FloorAssetType::Factory },
    FloorInfo { position: Vec3::new(1400.0, 2000.0, 1.0), size: Vec2::new(150.0, 400.0), breakable_wall: Some(BreakableWall { index: 1 }), floor_asset: FloorAssetType::Factory },
    FloorInfo { position: Vec3::new(900.0, 2500.0, 1.0), size: Vec2::new(150.0, 400.0), breakable_wall: Some(BreakableWall { index: 1 }), floor_asset: FloorAssetType::Factory },
];

static TIME_TRIAL_2: &[FloorInfo] = &[
    FloorInfo { position: Vec3::new(300.0, -650.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: Some(BreakableWall { index: 2 }), floor_asset: FloorAssetType::Factory },
    FloorInfo { position: Vec3::new(300.0, -350.0, 1.0), size: Vec2::new(800.0, 100.0), breakable_wall: Some(BreakableWall { index: 2 }), floor_asset: FloorAssetType::Factory },
    FloorInfo { position: Vec3::new(-900.0, -50.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: Some(BreakableWall { index: 2 }), floor_asset: FloorAssetType::Factory },
    FloorInfo { position: Vec3::new(-1400.0, 300.0, 1.0), size: Vec2::new(150.0, 400.0), breakable_wall: Some(BreakableWall { index: 2 }), floor_asset: FloorAssetType::Factory },
    FloorInfo { position: Vec3::new(-600.0, 400.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: Some(BreakableWall { index: 2 }), floor_asset: FloorAssetType::Factory },
];

static TIME_TRIAL_3: &[FloorInfo] = &[
    FloorInfo { position: Vec3::new(-300.0, -2600.0, 1.0), size: Vec2::new(150.0, 800.0), breakable_wall: Some(BreakableWall { index: 3 }), floor_asset: FloorAssetType::Factory },
    FloorInfo { position: Vec3::new(500.0, -2400.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: Some(BreakableWall { index: 3 }), floor_asset: FloorAssetType::Factory },
    FloorInfo { position: Vec3::new(1300.0, -2400.0, 1.0), size: Vec2::new(150.0, 500.0), breakable_wall: Some(BreakableWall { index: 3 }), floor_asset: FloorAssetType::Factory },
    FloorInfo { position: Vec3::new(1925.0, -1900.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: Some(BreakableWall { index: 3 }), floor_asset: FloorAssetType::Factory },
    FloorInfo { position: Vec3::new(1300.0, -1500.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: Some(BreakableWall { index: 3 }), floor_asset: FloorAssetType::Factory },
    FloorInfo { position: Vec3::new(600.0, -1500.0, 1.0), size: Vec2::new(150.0, 500.0), breakable_wall: Some(BreakableWall { index: 3 }), floor_asset: FloorAssetType::Factory },
];