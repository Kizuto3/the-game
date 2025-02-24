use bevy::math::{Vec2, Vec3};

use crate::{level::{level_layout::{cweamcat_house_layout::CweamcatHouseInfo, hell_1_layout::Hell1Info, starting_room_layout::StartingRoomInfo}, progression::Progression, Level}, npc::{conversation_entry::{ConversationEntry, ConversationPosition, Emotion}, CWEAMPUFF, NPC, OG_CWEAMPUFF}};

use super::{DoorCollider, FloorInfo, FloorModification, LevelInfo, TransitionCollider};

#[derive(Clone, Copy)]
pub struct CweamcatLairInfo;

impl LevelInfo for CweamcatLairInfo {
    fn get_floor_info(&self, cweampuff: &crate::Cweampuff) -> Vec<FloorInfo> {
        let mut floors = vec![
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
        ];

        if cweampuff.progression < Progression::HasCherish {
            floors.push(FloorInfo { position: Vec3::new(150.0, -350.0, 2.0), size: Vec2::new(300.0, 300.0) });
        }

        return floors;
    }

    fn get_transitions_info(&self, _cweampuff: &crate::Cweampuff) -> Vec<TransitionCollider> {
        vec![
            TransitionCollider { exit_index: 0, safe_position: Vec3::new(-350.0, 870.0, 1.0), transition_to_level: Level::StartingRoom(StartingRoomInfo), floor_info: FloorInfo { position: Vec3::new(-500.0, 950.0, 2.0), size: Vec2::new(100.0, 200.0) }  },
            TransitionCollider { exit_index: 1, safe_position: Vec3::new(2200.0, -50.0, 1.0), transition_to_level: Level::Hell1(Hell1Info), floor_info: FloorInfo { position: Vec3::new(2450.0, -350.0, 2.0), size: Vec2::new(300.0, 100.0) }  },
        ]
    }

    fn get_doors(&self, _cweampuff: &crate::Cweampuff) -> Vec<DoorCollider> {
        vec![
            DoorCollider { floor_info: FloorInfo { position: Vec3 { x: 1350., y: -50., z: 0.0 }, size: Vec2 { x: 100., y: 200. } },
                transition_to_level: Level::CweamcatHouse(CweamcatHouseInfo), safe_position: Vec3 { x: -750.0, y: -375.0, z: 1.0 }, is_active: false }
        ]
    }
    
    fn get_npcs(&self, cweampuff: &crate::Cweampuff) -> Vec<NPC> {
        let mut og_cweampuff = NPC { floor_info: FloorInfo { position: Vec3::new(750.0, -100.0, 0.0), size: Vec2::new(200.0, 100.0) }, is_active: false, current_conversation_index: 0,
                                      conversation: vec![], after_conversation_func: (|_cweampuff| { })   
        };

        match cweampuff.progression {
            Progression::None => {
                og_cweampuff.conversation = vec![
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Hello?..", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "Oh! Goodness!", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "You startled me!", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Sorry..", emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "We don't usually get a lot of visitors here...", emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "Hello! Welcome!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "How can this old Cweampuff help you?", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "I think I'm lost...\nI came in these lands looking for the hidden gem.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "But I fell down, and don't know where I am!", emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "Ah, so you've heard the legend.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "O-ho-ho!\nYou're just like me back in the days.", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "I too, used to travel these lands looking for my hidden gem.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Did you find it?", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "I did! It seems you're about to meet yours too!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "Step inside the house; your hidden gem is waiting for you.", emotion: Emotion::Regular },
                ];
            },
            Progression::MetMilk => {
                og_cweampuff.conversation = vec![
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Old Cweampuff! Old Cweampuff!", emotion: Emotion::Surprised },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "I've found her!", emotion: Emotion::Surprised },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "O-ho-ho!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "Isn't she the most precious thing in the world?", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "She is!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "The house looks incredible too!", emotion: Emotion::Surprised },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "It feels like...", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Home!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "O-ho-ho!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "Yes, indeed. She built it all on her own!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "On her own?!", emotion: Emotion::Surprised },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "It must've taken a lot of her strength.", emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "I can't wait to talk to her when she wakes up!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "Oh... Well, you see...", emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "She's been asleep for quite some time now...", emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "And I don't know how to wake her up...", emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "What? Oh no...", emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "I wish I could do something...", emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "But I can't leave her alone.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "Someone has to look after her.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "I want to help!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "But what can I do?", emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "I wish I knew...", emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "You'd probably need to go through Hell and back to wake her up.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Through Hell and back...", emotion: Emotion::Regular },
                ];
            },
            Progression::HasCherish => {
                // TODO
            }
        };

        vec![
            og_cweampuff
        ]
    }

    fn get_floor_modifications(&self, _cweampuff: &crate::Cweampuff) -> Vec<FloorModification> {
        vec![]
    }
}