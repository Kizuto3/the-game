use bevy::math::{Vec2, Vec3};

use crate::{level::{level_layout::{cweamcat_house_layout::CweamcatHouseInfo, starting_room_layout::StartingRoomInfo}, Level}, npc::{conversation_entry::{ConversationEntry, ConversationPosition, Emotion}, NPC}};

use super::{DoorCollider, FloorInfo, LevelInfo, TransitionCollider};

#[derive(Clone, Copy)]
pub struct CweamcatLairInfo;

impl LevelInfo for CweamcatLairInfo {
    fn get_floor_info(&self, _cweampuff: &crate::Cweampuf) -> Vec<FloorInfo> {
        vec![
            FloorInfo { position: Vec3::new(-450.0, 350.0, 1.0), size: Vec2::new(300.0, 1000.0) },
            FloorInfo { position: Vec3::new(-450.0, 1250.0, 1.0), size: Vec2::new(300.0, 400.0) },
            FloorInfo { position: Vec3::new(-300.0, 1600.0, 1.0), size: Vec2::new(600.0, 300.0) },
            FloorInfo { position: Vec3::new(1300.0, 1600.0, 1.0), size: Vec2::new(2000.0, 300.0) },
            FloorInfo { position: Vec3::new(-300.0, -350.0, 1.0), size: Vec2::new(600.0, 400.0) },
            FloorInfo { position: Vec3::new(1300.0, -350.0, 1.0), size: Vec2::new(2000.0, 400.0) },
            FloorInfo { position: Vec3::new(2900.0, -350.0, 1.0), size: Vec2::new(600.0, 400.0) },
            FloorInfo { position: Vec3::new(2900.0, 1600.0, 1.0), size: Vec2::new(600.0, 300.0) },
            FloorInfo { position: Vec3::new(3350.0, -150.0, 1.0), size: Vec2::new(300.0, 500.0) },
            FloorInfo { position: Vec3::new(3350.0, 1200.0, 1.0), size: Vec2::new(300.0, 1500.0) },
            FloorInfo { position: Vec3::new(2800.0, 350.0, 1.0), size: Vec2::new(150.0, 100.0) },
            FloorInfo { position: Vec3::new(2500.0, 650.0, 1.0), size: Vec2::new(150.0, 100.0) },
            FloorInfo { position: Vec3::new(2800.0, 950.0, 1.0), size: Vec2::new(150.0, 100.0) },
            FloorInfo { position: Vec3::new(2500.0, 1250.0, 1.0), size: Vec2::new(150.0, 100.0) },
        ]
    }

    fn get_transitions_info(&self, _cweampuff: &crate::Cweampuf) -> Vec<TransitionCollider> {
        vec![
            TransitionCollider { exit_index: 0, safe_position: Vec3::new(-350.0, 870.0, 1.0), transition_to_level: Level::StartingRoom(StartingRoomInfo), floor_info: FloorInfo { position: Vec3::new(-500.0, 950.0, 2.0), size: Vec2::new(100.0, 200.0) }  }
        ]
    }

    fn get_doors(&self, _cweampuff: &crate::Cweampuf) -> Vec<DoorCollider> {
        vec![
            DoorCollider { floor_info: FloorInfo { position: Vec3 { x: 1350., y: -50., z: 0.0 }, size: Vec2 { x: 100., y: 200. } },
                transition_to_level: Level::CweamcatHouse(CweamcatHouseInfo), safe_position: Vec3 { x: -750.0, y: -375.0, z: 1.0 }, is_active: false }
        ]
    }
    
    fn get_npcs(&self, _cweampuff: &crate::Cweampuf) -> Vec<NPC> {
        vec![
            NPC { floor_info: FloorInfo { position: Vec3::new(750.0, -100.0, 0.0), size: Vec2::new(200.0, 100.0) }, is_active: false, current_conversation_index: 0,
                conversation: vec![
                    ConversationEntry { position: ConversationPosition::Left, npc_name: "Cweampuff".to_string(), text: "Hello?..".to_string(), emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: "OG Cweampuff".to_string(), text: "Oh! Goodness!".to_string(), emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: "OG Cweampuff".to_string(), text: "You startled me!".to_string(), emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: "Cweampuff".to_string(), text: "Sorry..".to_string(), emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: "OG Cweampuff".to_string(), text: "We don't usually get a lot of visitors here...".to_string(), emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: "OG Cweampuff".to_string(), text: "Hello! And welcome!".to_string(), emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: "OG Cweampuff".to_string(), text: "How can this old cweampuff help you?".to_string(), emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: "Cweampuff".to_string(), text: "I think I got lost...\nI came in these lands looking for the hidden gem.".to_string(), emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: "Cweampuff".to_string(), text: "But I fell down and don't know where I am!".to_string(), emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: "OG Cweampuff".to_string(), text: "Ah, so you've heard the legend.".to_string(), emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: "OG Cweampuff".to_string(), text: "O-ho-ho!\nYou are just like me back in the days.".to_string(), emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: "OG Cweampuff".to_string(), text: "I too used to travel these lands looking for my hidden gem.".to_string(), emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: "Cweampuff".to_string(), text: "Did you find it?".to_string(), emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: "OG Cweampuff".to_string(), text: "Yes! And it seems you are about to meet yours!".to_string(), emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: "OG Cweampuff".to_string(), text: "Step inside the house.\nYour hidden gem is waiting for you.".to_string(), emotion: Emotion::Regular },
                ]   
            }
        ]
    }
}