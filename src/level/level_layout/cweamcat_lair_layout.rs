use bevy::math::{Vec2, Vec3};

use crate::{level::{level_layout::{cweamcat_house_layout::CweamcatHouseInfo, starting_room_layout::StartingRoomInfo}, progression::Progression, Level}, npc::{conversation_entry::{ConversationEntry, ConversationPosition, Emotion}, NPC}};

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
    
    fn get_npcs(&self, cweampuff: &crate::Cweampuf) -> Vec<NPC> {
        let mut og_cweampuff = NPC { floor_info: FloorInfo { position: Vec3::new(750.0, -100.0, 0.0), size: Vec2::new(200.0, 100.0) }, is_active: false, current_conversation_index: 0,
                                      conversation: vec![], after_conversation_func: (|_cweampuff| { })   
        };

        match cweampuff.progression {
            Progression::None => {
                og_cweampuff.conversation = vec![
                    ConversationEntry { position: ConversationPosition::Left, npc_name: "cweampuff".to_string(), text: "Hello?..".to_string(), emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: "og cweampuff".to_string(), text: "Oh! Goodness!".to_string(), emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: "og cweampuff".to_string(), text: "You startled me!".to_string(), emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: "cweampuff".to_string(), text: "Sorry..".to_string(), emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: "og cweampuff".to_string(), text: "We don't usually get a lot of visitors here...".to_string(), emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: "og cweampuff".to_string(), text: "Hello! Welcome!".to_string(), emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: "og cweampuff".to_string(), text: "How can this old Cweampuff help you?".to_string(), emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: "cweampuff".to_string(), text: "I think I'm lost...\nI came in these lands looking for the hidden gem.".to_string(), emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: "cweampuff".to_string(), text: "But I fell down, and don't know where I am!".to_string(), emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: "og cweampuff".to_string(), text: "Ah, so you've heard the legend.".to_string(), emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: "og cweampuff".to_string(), text: "O-ho-ho!\nYou're just like me back in the days.".to_string(), emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: "og cweampuff".to_string(), text: "I too, used to travel these lands looking for my hidden gem.".to_string(), emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: "cweampuff".to_string(), text: "Did you find it?".to_string(), emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: "og cweampuff".to_string(), text: "Yes! And it seems you're about to meet yours!".to_string(), emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: "og cweampuff".to_string(), text: "Step inside the house; your hidden gem is waiting for you.".to_string(), emotion: Emotion::Regular },
                ];
            },
            Progression::MetMilk => {
                og_cweampuff.conversation = vec![
                    ConversationEntry { position: ConversationPosition::Left, npc_name: "cweampuff".to_string(), text: "Old Cweampuff! Old Cweampuff!".to_string(), emotion: Emotion::Surprised },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: "cweampuff".to_string(), text: "I've found her!".to_string(), emotion: Emotion::Surprised },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: "og cweampuff".to_string(), text: "O-ho-ho!".to_string(), emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: "og cweampuff".to_string(), text: "Isn't she the most precious thing in the world?".to_string(), emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: "cweampuff".to_string(), text: "She is!".to_string(), emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: "cweampuff".to_string(), text: "But she is sleeping. I guess, being this wonderful takes a lot of strength.".to_string(), emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: "og cweampuff".to_string(), text: "Oh... Well, you see...".to_string(), emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: "og cweampuff".to_string(), text: "She's been asleep for quite some time now...".to_string(), emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: "og cweampuff".to_string(), text: "And I don't know how to wake her up...".to_string(), emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: "cweampuff".to_string(), text: "What? Oh no...".to_string(), emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: "og cweampuff".to_string(), text: "I wish I could do something...".to_string(), emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: "og cweampuff".to_string(), text: "But I can't leave her alone.".to_string(), emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: "og cweampuff".to_string(), text: "Someone has to look after her.".to_string(), emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: "cweampuff".to_string(), text: "I want to help!".to_string(), emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: "cweampuff".to_string(), text: "But what can I do?".to_string(), emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: "og cweampuff".to_string(), text: "I wish I knew...".to_string(), emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: "og cweampuff".to_string(), text: "You'd probably need to go through Hell and back to wake her up.".to_string(), emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: "cweampuff".to_string(), text: "Through Hell and back...".to_string(), emotion: Emotion::Regular },
                ];
            }
        };

        vec![
            og_cweampuff
        ]
    }
}