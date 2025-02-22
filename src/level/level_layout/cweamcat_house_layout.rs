use bevy::math::{Vec2, Vec3};

use crate::{level::{level_layout::cweamcat_lair_layout::CweamcatLairInfo, Level}, npc::{conversation_entry::{ConversationEntry, ConversationPosition, Emotion}, NPC}};

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
                    ConversationEntry { position: ConversationPosition::Left, npc_name: "Cweampuff".to_string(), text: "Hello?..".to_string(), emotion: Emotion::Regular },
                ]   
            }
        ]
    }
}