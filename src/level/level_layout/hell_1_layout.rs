use bevy::math::{Vec2, Vec3};

use crate::{level::{level_layout::{cweamcat_lair_layout::CweamcatLairInfo, hell_2_layout::Hell2Info}, progression::Progression, Level}, npc::{conversation_entry::{ConversationEntry, ConversationPosition, Emotion}, CWEAMPUFF, MINAWAN, NPC}, CWEAMPUFF_Z_INDEX};

use super::{DoorCollider, EntityInfo, FloorAssetType, FloorInfo, FloorModification, LevelInfo, TransitionCollider};

#[derive(Clone, Copy)]
pub struct Hell1Info;

impl LevelInfo for Hell1Info {
    fn get_floor_info(&self, _cweampuff: &crate::Cweampuff) -> Box<[FloorInfo]> {
        Box::from([
            FloorInfo { position: Vec3::new(-300.0, 1600.0, 1.0), size: Vec2::new(600.0, 300.0), breakable_wall: None, floor_asset: FloorAssetType::Hell },
            FloorInfo { position: Vec3::new(1600.0, 1600.0, 1.0), size: Vec2::new(2600.0, 300.0), breakable_wall: None, floor_asset: FloorAssetType::Hell },
            FloorInfo { position: Vec3::new(3050.0, 700.0, 1.0), size: Vec2::new(300.0, 2100.0), breakable_wall: None, floor_asset: FloorAssetType::Hell },
            FloorInfo { position: Vec3::new(-450.0, 700.0, 1.0), size: Vec2::new(300.0, 2100.0), breakable_wall: None, floor_asset: FloorAssetType::Hell },
            FloorInfo { position: Vec3::new(1050.0, -200.0, 1.0), size: Vec2::new(2700.0, 300.0), breakable_wall: None, floor_asset: FloorAssetType::Hell },
            FloorInfo { position: Vec3::new(2550.0, 350.0, 1.0), size: Vec2::new(300.0, 1500.0), breakable_wall: None, floor_asset: FloorAssetType::Hell },
            FloorInfo { position: Vec3::new(1800.0, 200.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Hell },
            FloorInfo { position: Vec3::new(1400.0, 500.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Hell },
            FloorInfo { position: Vec3::new(1000.0, 400.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Hell },
            FloorInfo { position: Vec3::new(400.0, 650.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Hell },
            FloorInfo { position: Vec3::new(1200.0, 850.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Hell},
            FloorInfo { position: Vec3::new(2000.0, 1050.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Hell },
        ])
    }

    fn get_transitions_info(&self, _cweampuff: &crate::Cweampuff) -> Option<Box<[TransitionCollider]>> {
        Some(Box::from([
            TransitionCollider { exit_index: 0, safe_position: Vec3::new(150.0, 1500.0, CWEAMPUFF_Z_INDEX), transition_to_level: Level::Hell2(Hell2Info), floor_info: EntityInfo { position: Vec3::new(2800.0, -300.0, 2.0), size: Vec2::new(200.0, 200.0) }  },
            TransitionCollider { exit_index: 1, safe_position: Vec3::new(150.0, 1500.0, CWEAMPUFF_Z_INDEX), transition_to_level: Level::CweamcatLair(CweamcatLairInfo), floor_info: EntityInfo { position: Vec3::new(150.0, 1700.0, 2.0), size: Vec2::new(300.0, 200.0) }  }
        ]))
    }

    fn get_doors(&self, _cweampuff: &crate::Cweampuff) -> Option<Box<[DoorCollider]>> {
        None
    }

    fn get_npcs(&self, cweampuff: &crate::Cweampuff) -> Option<Box<[NPC]>> {
        if cweampuff.progression <= Progression::MetMilk {
            return Some(Box::from([
                NPC {
                    floor_info: EntityInfo { position: Vec3::new(2000.0, 0.0, 0.0), size: Vec2::new(200.0, 100.0) }, is_active: false, current_conversation_index: 0,
                    after_conversation_func: |cweampuff, _commands, _breakable_walls, _cutscene| { cweampuff.has_double_jump = true; },
                    conversation: &[
                        ConversationEntry { position: ConversationPosition::Right, npc_name: MINAWAN, text: "Wan! Wan!", emotion: Emotion::Regular },
                        ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Wan wan?..", emotion: Emotion::Regular },
                        ConversationEntry { position: ConversationPosition::Right, npc_name: MINAWAN, text: "WAN! WAN!", emotion: Emotion::Happy },
                        ConversationEntry { position: ConversationPosition::Right, npc_name: MINAWAN, text: "What brings wan here?", emotion: Emotion::Regular },
                        ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "My hidden gem..", emotion: Emotion::Regular },
                        ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "I've just found her but-", emotion: Emotion::Sad },
                        ConversationEntry { position: ConversationPosition::Right, npc_name: MINAWAN, text: "You've found your hidden gem?!", emotion: Emotion::Surprised },
                        ConversationEntry { position: ConversationPosition::Right, npc_name: MINAWAN, text: "What's she like?", emotion: Emotion::Regular },
                        ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Oh! I've never met someone like that before!", emotion: Emotion::Happy },
                        ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Pure and radiant... Just being near her feels me with joy!", emotion: Emotion::Happy },
                        ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "But she's sleeping now...", emotion: Emotion::Sad },
                        ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "I'm looking for a way to wake her up.", emotion: Emotion::Sad },
                        ConversationEntry { position: ConversationPosition::Right, npc_name: MINAWAN, text: "Oh no...", emotion: Emotion::Sad },
                        ConversationEntry { position: ConversationPosition::Right, npc_name: MINAWAN, text: "So the curse might be real...", emotion: Emotion::Sad },
                        ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "The cur-", emotion: Emotion::Regular },
                        ConversationEntry { position: ConversationPosition::Right, npc_name: MINAWAN, text: "Oh- um...", emotion: Emotion::Sad },
                        ConversationEntry { position: ConversationPosition::Right, npc_name: MINAWAN, text: "You should go see other minawan.", emotion: Emotion::Regular },
                        ConversationEntry { position: ConversationPosition::Right, npc_name: MINAWAN, text: "We used to have a hidden gem of our own.", emotion: Emotion::Regular },
                        ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Used to?", emotion: Emotion::Regular },
                        ConversationEntry { position: ConversationPosition::Right, npc_name: MINAWAN, text: "She is a rising star now!", emotion: Emotion::Happy },
                        ConversationEntry { position: ConversationPosition::Right, npc_name: MINAWAN, text: "Oh, what an adorable hellhound she is!", emotion: Emotion::Happy },
                        ConversationEntry { position: ConversationPosition::Right, npc_name: MINAWAN, text: "I particularly admire her chocolate horns.", emotion: Emotion::Surprised },
                        ConversationEntry { position: ConversationPosition::Right, npc_name: MINAWAN, text: "I am currently on a mission: to spread word of her cuteness.", emotion: Emotion::Regular },
                        ConversationEntry { position: ConversationPosition::Right, npc_name: MINAWAN, text: "So I can't guide you to our domain. You'll have to find a way yourself.", emotion: Emotion::Regular },
                        ConversationEntry { position: ConversationPosition::Right, npc_name: MINAWAN, text: "I'm sure other minawan can help you with your hidden gem!", emotion: Emotion::Happy },
                        ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Thank you! I will! Wan wan!", emotion: Emotion::Happy },
                        ConversationEntry { position: ConversationPosition::Right, npc_name: MINAWAN, text: "WAN! WAN!", emotion: Emotion::Happy },
                        ConversationEntry { position: ConversationPosition::Right, npc_name: MINAWAN, text: "*** Seeing how Minawan adores his rising star fills you with excitement! You want jump even when you are in the air ***", emotion: Emotion::Happy },
                    ]
                }
            ]))
        }
        
        None
    }
    
    fn get_floor_modifications(&self, _cweampuff: &crate::Cweampuff) -> Option<Box<[FloorModification]>> {
        None
    }
}