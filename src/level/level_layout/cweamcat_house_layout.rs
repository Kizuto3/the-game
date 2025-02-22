use bevy::math::{Vec2, Vec3};

use crate::{level::{level_layout::cweamcat_lair_layout::CweamcatLairInfo, progression::Progression, Level}, npc::{conversation_entry::{ConversationEntry, ConversationPosition, Emotion}, NPC}};

use super::{DoorCollider, FloorInfo, LevelInfo, TransitionCollider};

#[derive(Clone, Copy)]
pub struct CweamcatHouseInfo;

impl LevelInfo for CweamcatHouseInfo {
    fn get_floor_info(&self, _cweampuff: &crate::Cweampuf) -> Vec<FloorInfo> {
        vec![
            FloorInfo { position: Vec3::new(-1200.0, 0.0, 1.0), size: Vec2::new(400.0, 1500.0) },
            FloorInfo { position: Vec3::new(0.0, 600.0, 1.0), size: Vec2::new(2000.0, 400.0) },
            FloorInfo { position: Vec3::new(1200.0, 0.0, 1.0), size: Vec2::new(400.0, 1500.0) },
            FloorInfo { position: Vec3::new(0.0, -600.0, 1.0), size: Vec2::new(2000.0, 400.0) },
        ]
    }

    fn get_transitions_info(&self, _cweampuff: &crate::Cweampuf) -> Vec<TransitionCollider> {
        vec![]
    }

    fn get_doors(&self, _cweampuff: &crate::Cweampuf) -> Vec<DoorCollider> {
        vec![
            DoorCollider { floor_info: FloorInfo { position: Vec3 { x: -750., y: -300., z: 0.0 }, size: Vec2 { x: 100., y: 200. } },
                transition_to_level: Level::CweamcatLair(CweamcatLairInfo), safe_position: Vec3 { x: 1350., y: -125., z: 1.0 }, is_active: false }
        ]
    }
    
    fn get_npcs(&self, _cweampuff: &crate::Cweampuf) -> Vec<NPC> {
        vec![
            NPC { floor_info: FloorInfo { position: Vec3::new(750.0, -300.0, 0.0), size: Vec2::new(200.0, 200.0) }, is_active: false, current_conversation_index: 0,
                conversation: vec![
                    ConversationEntry { position: ConversationPosition::Right, npc_name: "Milk_asleep".to_string(), text: "...".to_string(), emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: "Cweampuff".to_string(), text: "Wow...".to_string(), emotion: Emotion::Surprised },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: "Cweampuff".to_string(), text: "Is that?...".to_string(), emotion: Emotion::Surprised },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: "Cweampuff".to_string(), text: "What is this place?...".to_string(), emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: "Cweampuff".to_string(), text: "And this feeling...".to_string(), emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: "Cweampuff".to_string(), text: "I feel so... Warm...".to_string(), emotion: Emotion::Surprised },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: "Cweampuff".to_string(), text: "And happy!".to_string(), emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: "Cweampuff".to_string(), text: "Is this the power of the hidden gem?".to_string(), emotion: Emotion::Surprised },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: "Cweampuff".to_string(), text: "Excuse me...".to_string(), emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: "Cweampuff".to_string(), text: "Are you...".to_string(), emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: "Cweampuff".to_string(), text: "Are you my hidden gem?".to_string(), emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: "Milk_asleep".to_string(), text: "...".to_string(), emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: "Cweampuff".to_string(), text: "H-hello?".to_string(), emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: "Milk_asleep".to_string(), text: "...".to_string(), emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: "Cweampuff".to_string(), text: "Oh... She must be very tired.".to_string(), emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: "Cweampuff".to_string(), text: "Maybe Old Cweampuff knows something about this.".to_string(), emotion: Emotion::Sad },
                ],
                after_conversation_func: (|cweampuff| cweampuff.progression = Progression::MetMilk)
            }
        ]
    }
}