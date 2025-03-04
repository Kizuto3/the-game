use bevy::math::{Vec2, Vec3};

use crate::{cutscene::{CutsceneEvent, CutsceneInfo}, level::{level_layout::cweamcat_lair_layout::CweamcatLairInfo, progression::Progression, Level}, npc::{conversation_entry::{ConversationEntry, ConversationPosition, Emotion}, CWEAMPUFF, MILK_ASLEEP, NPC}};

use super::{DoorCollider, EntityInfo, FloorInfo, FloorModification, LevelInfo, TransitionCollider};

#[derive(Clone, Copy)]
pub struct CweamcatHouseInfo;

impl LevelInfo for CweamcatHouseInfo {
    fn get_floor_info(&self, _cweampuff: &crate::Cweampuff) -> Box<[FloorInfo]> {
        Box::from([
            FloorInfo { position: Vec3::new(-1200.0, 0.0, 1.0), size: Vec2::new(400.0, 1500.0), breakable_wall: None },
            FloorInfo { position: Vec3::new(0.0, 600.0, 1.0), size: Vec2::new(2000.0, 400.0), breakable_wall: None},
            FloorInfo { position: Vec3::new(1200.0, 0.0, 1.0), size: Vec2::new(400.0, 1500.0), breakable_wall: None },
            FloorInfo { position: Vec3::new(0.0, -600.0, 1.0), size: Vec2::new(2000.0, 400.0), breakable_wall: None },
        ])
    }

    fn get_transitions_info(&self, _cweampuff: &crate::Cweampuff) -> Option<Box<[TransitionCollider]>> {
        None
    }

    fn get_doors(&self, _cweampuff: &crate::Cweampuff) -> Option<Box<[DoorCollider]>> {
        Some(Box::from([
            DoorCollider { floor_info: EntityInfo { position: Vec3 { x: -750., y: -300., z: 0.0 }, size: Vec2 { x: 100., y: 200. } },
                transition_to_level: Level::CweamcatLair(CweamcatLairInfo), safe_position: Vec3 { x: 1350., y: -125., z: 1.0 }, is_active: false }
        ]))
    }
    
    fn get_npcs(&self, cweampuff: &crate::Cweampuff) -> Option<Box<[NPC]>> {
        let mut milk = NPC { floor_info: EntityInfo { position: Vec3::new(750.0, -300.0, 0.0), size: Vec2::new(200.0, 200.0) }, is_active: false, current_conversation_index: 0,
                                  conversation: &[], after_conversation_func: |_cweampuff, _commands, _breakable_walls, _cutscene| {}
        };

        match cweampuff.progression {
            Progression::None => {
                milk.conversation = &[
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
                ];

                milk.after_conversation_func = |cweampuff, _commands, _breakable_walls, _cutscene| { 
                    if cweampuff.progression < Progression::MetMilk { 
                        cweampuff.progression = Progression::MetMilk;
                    }
                };
            },
            Progression::MetMilk => {
                milk.conversation = &[
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MILK_ASLEEP, text: "...", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "She is perfect...", emotion: Emotion::Surprised },
                ];
            },
            Progression::HasCherish => {
                milk.conversation = &[
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MILK_ASLEEP, text: "...", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "She is perfect...", emotion: Emotion::Surprised },
                ];

                milk.after_conversation_func = |cweampuff, _commands, _breakable_walls, cutscene| { 
                    if cweampuff.progression < Progression::MilkWokeUp { 
                        cweampuff.progression = Progression::MilkWokeUp;
                    }

                    cutscene.send(CutsceneEvent::Started(&[
                        CutsceneInfo { text: "Milk is still sleaping", background: "" },
                        CutsceneInfo { text: "Milk slightly opens her eye", background: "" },
                        CutsceneInfo { text: "Milk wakes up", background: "" },
                    ], Level::CweamcatHouse(CweamcatHouseInfo), "vine-boom.mp3"));
                };
            },
            Progression::MilkWokeUp => {
                // TODO
            }
        }

        Some(Box::from([milk]))
    }

    fn get_floor_modifications(&self, _cweampuff: &crate::Cweampuff) -> Option<Box<[FloorModification]>> {
        None
    }
}