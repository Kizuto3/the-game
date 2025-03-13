use bevy::math::{Vec2, Vec3};

use crate::{cutscene::{CutsceneEvent, CutsceneInfo}, level::{level_layout::cweamcat_lair_layout::CweamcatLairInfo, progression::Progression, Level}, npc::{conversation_entry::{ConversationEntry, ConversationPosition, Emotion}, CWEAMPUFF, MILK, MILK_ASLEEP, NPC, OG_CWEAMPUFF}, CWEAMPUFF_Z_INDEX};

use super::{DoorCollider, EntityInfo, FloorAssetType, FloorInfo, FloorModification, LevelInfo, TransitionCollider};

#[derive(Clone, Copy)]
pub struct CweamcatHouseInfo;

impl LevelInfo for CweamcatHouseInfo {
    fn get_floor_info(&self, _cweampuff: &crate::Cweampuff) -> Box<[FloorInfo]> {
        Box::from([
            FloorInfo { position: Vec3::new(-1200.0, 0.0, 1.0), size: Vec2::new(400.0, 1500.0), breakable_wall: None, floor_asset: FloorAssetType::CweamcatHouse },
            FloorInfo { position: Vec3::new(0.0, 600.0, 1.0), size: Vec2::new(2000.0, 400.0), breakable_wall: None, floor_asset: FloorAssetType::CweamcatHouse },
            FloorInfo { position: Vec3::new(1200.0, 0.0, 1.0), size: Vec2::new(400.0, 1500.0), breakable_wall: None, floor_asset: FloorAssetType::CweamcatHouse },
            FloorInfo { position: Vec3::new(0.0, -600.0, 1.0), size: Vec2::new(2000.0, 400.0), breakable_wall: None, floor_asset: FloorAssetType::CweamcatHouse },
        ])
    }

    fn get_transitions_info(&self, _cweampuff: &crate::Cweampuff) -> Option<Box<[TransitionCollider]>> {
        None
    }

    fn get_doors(&self, _cweampuff: &crate::Cweampuff) -> Option<Box<[DoorCollider]>> {
        Some(Box::from([
            DoorCollider { floor_info: EntityInfo { position: Vec3 { x: -750., y: -300., z: 0.0 }, size: Vec2 { x: 100., y: 200. } },
                transition_to_level: Level::CweamcatLair(CweamcatLairInfo), safe_position: Vec3 { x: 1350., y: -125., z: CWEAMPUFF_Z_INDEX }, is_active: false }
        ]))
    }
    
    fn get_npcs(&self, cweampuff: &crate::Cweampuff) -> Option<Box<[NPC]>> {
        let mut npcs = vec![];

        let mut milk = NPC { floor_info: EntityInfo { position: Vec3::new(750.0, -300.0, 0.0), size: Vec2::new(200.0, 200.0) }, is_active: false, current_conversation_index: 0,
                                  conversation: &[], after_conversation_func: |_cweampuff, _commands, _breakable_walls, _cutscene| {}
        };

        let mut og_cweampuff = NPC { floor_info: EntityInfo { position: Vec3::new(350.0, -350.0, 0.0), size: Vec2::new(200.0, 100.0) }, is_active: false, current_conversation_index: 0,
                                          conversation: &[], after_conversation_func: |_cweampuff, _commands, _breakable_walls, _cutscene| { }
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
                og_cweampuff.conversation = &[
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "It's time...", emotion: Emotion::Sad },
                ];

                npcs.push(og_cweampuff);

                milk.conversation = &[
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Hello!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MILK_ASLEEP, text: "...", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "I really hope you can hear us, there is a lot we need to tell you.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: OG_CWEAMPUFF, text: "Miruku...", emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MILK_ASLEEP, text: "...", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "You know, I've come to these lands looking for something...", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Something to give me joy... Something to give me happiness...", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "At my 'home' I didn't feel quite right.", emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "A lot things didn't go my way.", emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "That's when I've heard of the legend of a hidden gem.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "So I went looking for something that could bring me solace.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Now I know, I was looking for you!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MILK_ASLEEP, text: "...", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: OG_CWEAMPUFF, text: "Miruku... I'm sorry.", emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: OG_CWEAMPUFF, text: "For some reason, I just assumed you would always be with us.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: OG_CWEAMPUFF, text: "I forgot to show you what you mean to us... What you mean to me...", emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: OG_CWEAMPUFF, text: "I'm sorry...", emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MILK_ASLEEP, text: "...", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Maybe it was fate that the bridge collapsed, maybe it was luck.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "But I've found you, and I couldn't be happier about it!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "I've met a lot of nice people while I was looking for a way to wake you up!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Even Minawan came here to see you!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MILK_ASLEEP, text: "...", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: OG_CWEAMPUFF, text: "Do you remember back in the days how we would sit here around you listening to your stories?", emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: OG_CWEAMPUFF, text: "Or how we would banter with you? Or play silly music to catch you off guard?", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: OG_CWEAMPUFF, text: "Oh-ho-ho, how fun it was!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: OG_CWEAMPUFF, text: "Do you think we can do that again?", emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MILK_ASLEEP, text: "...", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Perhaps, it may sound silly of me, but I need you...", emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "I've just found you, but I know it's true.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "I will do everything to prove it!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "I just need one thing from you...", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Please... Wake up...", emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MILK_ASLEEP, text: "...", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: OG_CWEAMPUFF, text: "Miruku...", emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: OG_CWEAMPUFF, text: "There is no one, not a single soul, that can replace you.", emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: OG_CWEAMPUFF, text: "I promise, I will always be there for you no matter what.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: OG_CWEAMPUFF, text: "We will make you happy!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "We will never leave you!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: OG_CWEAMPUFF, text: "We will support you!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "We will cherish you!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: OG_CWEAMPUFF, text: "Please, wake up!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Wake up!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MILK_ASLEEP, text: "...", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MILK_ASLEEP, text: "...", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MILK_ASLEEP, text: "?..", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MILK_ASLEEP, text: "!..", emotion: Emotion::Regular },
                ];

                milk.after_conversation_func = |cweampuff, _commands, _breakable_walls, cutscene| { 
                    if cweampuff.progression < Progression::MilkWokeUp { 
                        cweampuff.progression = Progression::MilkWokeUp;
                    }

                    cutscene.send(CutsceneEvent::Started(&[
                        CutsceneInfo { text: "Milk is still sleeping", background: "cutscenes/milk wakes up/1.png" },
                        CutsceneInfo { text: "Milk slightly opens her eye", background: "cutscenes/milk wakes up/2.png" },
                        CutsceneInfo { text: "Milk wakes up", background: "cutscenes/milk wakes up/3.png" },
                    ], Level::CweamcatHouse(CweamcatHouseInfo), "vine-boom.mp3"));
                };
            },
            Progression::MilkWokeUp => {
                og_cweampuff.conversation = &[
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "She woke up!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "Cweampuff, you did it!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "I'm so happy!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "But I couldn't do it without you.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "I'm sure your words meant a lot to her!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "Oh-ho-ho!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "This is the happiest day of my life!", emotion: Emotion::Happy },
                ];

                npcs.push(og_cweampuff);

                milk.conversation = &[
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "?..", emotion: Emotion::Surprised },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: OG_CWEAMPUFF, text: "?..", emotion: Emotion::Surprised },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MILK, text: "...", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MILK, text: "Good morning!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: OG_CWEAMPUFF, text: "Miruku! Thank goodness! You woke up!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Her voice...", emotion: Emotion::Surprised },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "She sounds like an angel!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: OG_CWEAMPUFF, text: "You were asleep for so long!", emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MILK, text: "I made you worried, didn't I?", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MILK, text: "I'm sorry, Old Cweampuff.", emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MILK, text: "I heard your voices while I was dreaming.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MILK, text: "Thank you! Both of you!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: OG_CWEAMPUFF, text: "I'm the one who should apologize...", emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MILK, text: "You have nothing to apologize for, Old Cweampuff.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MILK, text: "Without you I could've never become a hidden gem.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MILK, text: "You were always looking out for me.", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MILK, text: "Even when thing were tough, you stayed with me.", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MILK, text: "You helped to wake me up.", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MILK, text: "So thank you, Old Cweampuff!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: OG_CWEAMPUFF, text: "Miruku...", emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: OG_CWEAMPUFF, text: "Oh, Miruku!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "...", emotion: Emotion::Surprised },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MILK, text: "Thank you too, Cweampuff.", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MILK, text: "You've gone on quite a journey to wake me up.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "And I would do it again!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "I'm so happy!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MILK, text: "You guys are so cute!.", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MILK, text: "...", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MILK, text: "...", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: OG_CWEAMPUFF, text: "Miruku?", emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MILK, text: "Sorry, I feel a bit weak after such a long sleep.", emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MILK, text: "Don't worry about me, I just need time to recover.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: OG_CWEAMPUFF, text: "Oh, she is right. Cweampuff, let's go outside and give her some space.", emotion: Emotion::Regular },
                ];
            },
            Progression::HasLetter => {

            }
        }

        npcs.push(milk);

        Some(npcs.into_boxed_slice())
    }

    fn get_floor_modifications(&self, _cweampuff: &crate::Cweampuff) -> Option<Box<[FloorModification]>> {
        None
    }
}