use bevy::math::{Vec2, Vec3};

use crate::{level::{level_layout::cweamcat_lair_layout::CweamcatLairInfo, progression::Progression, Level}, npc::{conversation_entry::{ConversationEntry, ConversationPosition, Emotion}, CWEAMPUFF, MILK_ASLEEP, NPC}};

use super::{DoorCollider, FloorInfo, FloorModification, LevelInfo, TransitionCollider};

#[derive(Clone, Copy)]
pub struct CweamcatHouseInfo;

impl LevelInfo for CweamcatHouseInfo {
    fn get_floor_info(&self, _cweampuff: &crate::Cweampuff) -> Vec<FloorInfo> {
        vec![
            FloorInfo { position: Vec3::new(-1200.0, 0.0, 1.0), size: Vec2::new(400.0, 1500.0) },
            FloorInfo { position: Vec3::new(0.0, 600.0, 1.0), size: Vec2::new(2000.0, 400.0) },
            FloorInfo { position: Vec3::new(1200.0, 0.0, 1.0), size: Vec2::new(400.0, 1500.0) },
            FloorInfo { position: Vec3::new(0.0, -600.0, 1.0), size: Vec2::new(2000.0, 400.0) },
        ]
    }

    fn get_transitions_info(&self, _cweampuff: &crate::Cweampuff) -> Vec<TransitionCollider> {
        vec![]
    }

    fn get_doors(&self, _cweampuff: &crate::Cweampuff) -> Vec<DoorCollider> {
        vec![
            DoorCollider { floor_info: FloorInfo { position: Vec3 { x: -750., y: -300., z: 0.0 }, size: Vec2 { x: 100., y: 200. } },
                transition_to_level: Level::CweamcatLair(CweamcatLairInfo), safe_position: Vec3 { x: 1350., y: -125., z: 1.0 }, is_active: false }
        ]
    }
    
    fn get_npcs(&self, _cweampuff: &crate::Cweampuff) -> Vec<NPC> {
        vec![
            NPC { floor_info: FloorInfo { position: Vec3::new(750.0, -300.0, 0.0), size: Vec2::new(200.0, 200.0) }, is_active: false, current_conversation_index: 0,
                conversation: vec![
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MILK_ASLEEP, text: "...", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Wow...", emotion: Emotion::Surprised },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Is that?...", emotion: Emotion::Surprised },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "What is this place?...", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "And this feeling...", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "I feel so... Warm...", emotion: Emotion::Surprised },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "And happy!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Is this the power of the hidden gem?", emotion: Emotion::Surprised },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Excuse me...", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Are you...", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Are you my hidden gem?", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MILK_ASLEEP, text: "...", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "H-hello?", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MILK_ASLEEP, text: "...", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Oh... She must be very tired.", emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Maybe Old Cweampuff knows something about this.", emotion: Emotion::Regular },
                ],
                after_conversation_func: (|cweampuff| { 
                    if cweampuff.progression < Progression::MetMilk { 
                        cweampuff.progression = Progression::MetMilk;
                    }
                })
            }
        ]
    }

    fn get_floor_modifications(&self, _cweampuff: &crate::Cweampuff) -> Vec<FloorModification> {
        vec![]
    }
}