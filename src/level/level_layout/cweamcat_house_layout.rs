use bevy::math::{Vec2, Vec3};

use crate::{cutscene::{CutsceneEvent, CutsceneInfo, PostCutsceneAction}, level::{level_layout::cweamcat_lair_layout::CweamcatLairInfo, progression::Progression, Level}, npc::{conversation_entry::{ConversationEntry, ConversationPosition, Emotion}, COOL_CWEAMPUFF, CWEAMPUFF, CWEAMPUFFS, MASKED_CWEAMPUFF, MILK, MILK_ASLEEP, NPC, OG_CWEAMPUFF, RICH_CWEAMPUFF}, CWEAMPUFF_Z_INDEX};

use super::{DoorCollider, DoorType, EntityInfo, FloorAssetType, FloorInfo, FloorModification, LevelInfo, TransitionCollider};

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
            DoorCollider { floor_info: EntityInfo { position: Vec3 { x: -750., y: -300., z: 0.0 }, size: Vec2 { x: 100., y: 200. } }, door_type: DoorType::Door,
                transition_to_level: Level::CweamcatLair(CweamcatLairInfo), safe_position: Vec3 { x: 1350., y: -125., z: CWEAMPUFF_Z_INDEX }, is_active: false }
        ]))
    }
    
    fn get_npcs(&self, cweampuff: &crate::Cweampuff) -> Option<Box<[NPC]>> {
        let mut npcs = vec![];

        let mut milk = NPC { floor_info: EntityInfo { position: Vec3::new(750.0, -300.0, 0.0), size: Vec2::new(200.0, 200.0) }, is_active: false, current_conversation_index: 0,
                                  conversation: &[], after_conversation_func: |_cweampuff, _commands, _breakable_walls, _cutscene| {},
                                  name: MILK
        };

        let mut og_cweampuff = NPC { floor_info: EntityInfo { position: Vec3::new(450.0, -350.0, 0.0), size: Vec2::new(200.0, 100.0) }, is_active: false, current_conversation_index: 0,
                                          conversation: &[], after_conversation_func: |_cweampuff, _commands, _breakable_walls, _cutscene| { },
                                          name: OG_CWEAMPUFF
        };

        let mut cool_cweampuff = NPC { floor_info: EntityInfo { position: Vec3::new(-450.0, -350.0, 0.0), size: Vec2::new(200.0, 100.0) }, is_active: false, current_conversation_index: 0,
                                          conversation: &[], after_conversation_func: |_cweampuff, _commands, _breakable_walls, _cutscene| { },
                                          name: COOL_CWEAMPUFF
        };

        let mut masked_cweampuff = NPC { floor_info: EntityInfo { position: Vec3::new(150.0, -350.0, 0.0), size: Vec2::new(200.0, 100.0) }, is_active: false, current_conversation_index: 0,
                                            conversation: &[], after_conversation_func: |_cweampuff, _commands, _breakable_walls, _cutscene| { },
                                            name: MASKED_CWEAMPUFF
        };

        let mut rich_cweampuff = NPC { floor_info: EntityInfo { position: Vec3::new(-150.0, -350.0, 0.0), size: Vec2::new(200.0, 100.0) }, is_active: false, current_conversation_index: 0,
                                            conversation: &[], after_conversation_func: |_cweampuff, _commands, _breakable_walls, _cutscene| { },
                                            name: RICH_CWEAMPUFF
        };

        match cweampuff.progression {
            Progression::None => {
                milk.name = MILK_ASLEEP;
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
                milk.name = MILK_ASLEEP;

                milk.conversation = &[
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MILK_ASLEEP, text: "...", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "She is perfect...", emotion: Emotion::Surprised },
                ];
            },
            Progression::HasCherish => {
                milk.name = MILK_ASLEEP;

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
                    ], "vine-boom.mp3", PostCutsceneAction::TransitionTo(Level::CweamcatHouse(CweamcatHouseInfo))));
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
                og_cweampuff.conversation = &[
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "I wonder what's in that bottle...", emotion: Emotion::Regular },
                ];

                npcs.push(og_cweampuff);

                cool_cweampuff.conversation = &[
                    ConversationEntry { position: ConversationPosition::Right, npc_name: COOL_CWEAMPUFF, text: "Yo, sup, Cweampuff.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Hello, Cool Cweampuff!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Have you been hanging out with out hidden gem?", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: COOL_CWEAMPUFF, text: "Oh- um- y-yeah, you could say so.", emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: COOL_CWEAMPUFF, text: "*cough cough* I mean, I've just been looking out for her.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: COOL_CWEAMPUFF, text: "Lurking around, just to make sure she's OK.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Wow, you're so cool.", emotion: Emotion::Surprised },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "I bet you had a lot to talk about!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: COOL_CWEAMPUFF, text: "Yeah, she even said 'Hi' to me!", emotion: Emotion::Surprised },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: COOL_CWEAMPUFF, text: "But I was too scare-", emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: COOL_CWEAMPUFF, text: "*cough* *cough* I mean, I was too stunned by her beauty to say anything back.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "I totally know what you mean.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Still, with you around our hidden gem has nothing to worry about.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "I'm happy you are here.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: COOL_CWEAMPUFF, text: "You think so?", emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: COOL_CWEAMPUFF, text: "I've been a little under the weather lately...", emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "What? Why?", emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: COOL_CWEAMPUFF, text: "Don't worry about it.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: COOL_CWEAMPUFF, text: "I don't really want to talk about it in front of our hidden gem.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: COOL_CWEAMPUFF, text: "We're here to make her happy, aren't we?", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Well, you can always talk to me if you need to.", emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: COOL_CWEAMPUFF, text: "Thanks, Cweampuff. You've become a bit cooler since last we met.", emotion: Emotion::Regular },
                ];

                npcs.push(cool_cweampuff);

                milk.conversation = &[
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MILK, text: "Cweampuff! Hi! I missed you!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "...", emotion: Emotion::Surprised },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "I missed you too!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Are you feeling better now?", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MILK, text: "I am, thank you!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MILK, text: "*Yawn* Although, I'm still a bit sleepy.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MILK, text: "What've you been up to, my precious little Cweampuff?", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "...", emotion: Emotion::Surprised },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "I've looking for a way to bring more cweampuffs here!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "So our family can grow bigger!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MILK, text: "Oh, Cweampuff... You didn't have to.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MILK, text: "You don't have to do anything at all.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MILK, text: "Just being here with me is enough.", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "But I want to!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "The spaceship's captain asked me to deliver you this letter in a bottle.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MILK, text: "Captain?", emotion: Emotion::Surprised },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MILK, text: "SHE wanted ME to have it?", emotion: Emotion::Surprised },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "You know her?", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MILK, text: "Well, I've always admired her. I didn't think she'd known me!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MILK, text: "I'm so excited to read this letter!", emotion: Emotion::Surprised },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MILK, text: "'Dear Miruku...'", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MILK, text: "...", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MILK, text: "...", emotion: Emotion::Surprised },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MILK, text: "'I hope this helps you...'", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MILK, text: "...", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MILK, text: "'Do you...'", emotion: Emotion::Surprised },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MILK, text: "'... think we can be friends?'", emotion: Emotion::Surprised },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MILK, text: "Huh? There is a picture here as well...", emotion: Emotion::Regular },
                ];

                milk.after_conversation_func = |cweampuff, _commands, _breakable_walls, cutscene| { 
                    if cweampuff.progression < Progression::GivenLetter { 
                        cweampuff.progression = Progression::GivenLetter;
                    }

                    cutscene.send(CutsceneEvent::Started(&[
                        CutsceneInfo { text: "*** Milk reads letter ***", background: "cutscenes/letter/1.png" },
                        CutsceneInfo { text: "*** Looks at the picture ***", background: "cutscenes/letter/2.png" },
                        CutsceneInfo { text: "*** Ghost appears ***", background: "cutscenes/letter/3.png" },
                    ], "vine-boom.mp3", PostCutsceneAction::TransitionTo(Level::CweamcatHouse(CweamcatHouseInfo))));
                };
            },
            Progression::GivenLetter => {
                og_cweampuff.conversation = &[
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "Did my eyes deceive me?", emotion: Emotion::Surprised },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "You saw that too, didn't you?", emotion: Emotion::Surprised },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "I did!", emotion: Emotion::Surprised },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "Let's meet outside. We need to discuss what've just seen.", emotion: Emotion::Surprised },
                ];

                npcs.push(og_cweampuff);

                cool_cweampuff.conversation = &[
                    ConversationEntry { position: ConversationPosition::Right, npc_name: COOL_CWEAMPUFF, text: "I've never seen such perfection before...", emotion: Emotion::Surprised },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Exactly! It's like I've just opened my eyes for the first time.", emotion: Emotion::Surprised },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: COOL_CWEAMPUFF, text: "I have to go now. I need to draw this event in full details.", emotion: Emotion::Surprised },
                ];

                npcs.push(cool_cweampuff);

                milk.conversation = &[
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MILK, text: "Wow!", emotion: Emotion::Surprised },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "...", emotion: Emotion::Surprised },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MILK, text: "Just now I've felt so warm...", emotion: Emotion::Surprised },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MILK, text: "A robot girl, huh...", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MILK, text: "Cweampuff... Thank you so much!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MILK, text: "This letter means so much to me!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MILK, text: "I never could've imagined she wanted to be friends with me...", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MILK, text: "I love that lady so much!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Y-you're welcome...", emotion: Emotion::Surprised },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MILK, text: "Grrrr, cweampuffs are so nice to me!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MILK, text: "But please don't feel like you have to do something for me.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "...", emotion: Emotion::Surprised },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MILK, text: "Cweampuff?..", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "I can't even describe to you what you just made me feel...", emotion: Emotion::Surprised },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "I need to step outside and take a breather...", emotion: Emotion::Surprised },
                ];
            },
            Progression::RisingStar => {
                milk.conversation = &[
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MILK, text: "Cweampuffs! You're all here!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MILK, text: "The crew, minawan and even the Swarm are all here!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MILK, text: "I can't believe I've made friends with the crew's captain and met all of you!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "I'm so glad to see everyone noticing you!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MILK, text: "Everyone is so nice to me!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MILK, text: "But I...", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MILK, text: "I don't know why you're doing this.", emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MILK, text: "Old Cweampuff, you've always stayed with me...", emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: OG_CWEAMPUFF, text: "...", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MILK, text: "Cool Cweampuff, you're always there to listen to my silly stories...", emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: COOL_CWEAMPUFF, text: "...", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MILK, text: "Masked Cweampuff, you always surprise me with your presents...", emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MILK, text: "But I can't even thank you properly!..", emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: MASKED_CWEAMPUFF, text: "...", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MILK, text: "Rich Cweampuff, you spend way too much money on me...", emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: RICH_CWEAMPUFF, text: "...", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MILK, text: "Cweampuff, you've traveled all over these lands to help me...", emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "...", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MILK, text: "I don't know what I did to deserve this...", emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MILK, text: "I don't even know how to thank you properly...", emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MILK, text: "'Thank you' isn't enough when it's up against this mountain of love...", emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: RICH_CWEAMPUFF, text: "Your happiness is our biggest reward!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: MASKED_CWEAMPUFF, text: "We all just want to see you smile!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: COOL_CWEAMPUFF, text: "You accept me for who I am! It's me who should be thanking you!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: OG_CWEAMPUFF, text: "I know more than anyone else just how much you deserve all of this!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "You built a house for us with your own hands!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "You always smile when we come visit you!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "You gave us home!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MILK, text: "Stop...", emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "It's your hard work paying off!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: OG_CWEAMPUFF, text: "You being you is enough!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFFS, text: "WE LOVE YOU!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MILK, text: "Cweampuffs...", emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "But this is just the beginning, isn't it?", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "You've been waiting for this for a long time now.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "It's time for you to shine.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "...", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFFS, text: "Rise, our star!", emotion: Emotion::Happy },
                ];

                milk.after_conversation_func = |_cweampuff, _commands, _breakable_walls, cutscene| { 
                    cutscene.send(CutsceneEvent::Started(&[
                        CutsceneInfo { text: "", background: "cutscenes/rising star/1.png" },
                        CutsceneInfo { text: "", background: "cutscenes/rising star/2.png" },
                        CutsceneInfo { text: "", background: "cutscenes/rising star/3.png" },
                    ], "vine-boom.mp3", PostCutsceneAction::EndGame));
                };

                masked_cweampuff.conversation = &[
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MASKED_CWEAMPUFF, text: "Cute...", emotion: Emotion::Regular },
                ];

                npcs.push(masked_cweampuff);

                og_cweampuff.conversation = &[
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "Adorable...", emotion: Emotion::Regular },
                ];

                npcs.push(og_cweampuff);

                cool_cweampuff.conversation = &[
                    ConversationEntry { position: ConversationPosition::Right, npc_name: COOL_CWEAMPUFF, text: "Perfect...", emotion: Emotion::Regular },
                ];

                npcs.push(cool_cweampuff);

                rich_cweampuff.conversation = &[
                    ConversationEntry { position: ConversationPosition::Right, npc_name: RICH_CWEAMPUFF, text: "Angelic..", emotion: Emotion::Regular },
                ];

                npcs.push(rich_cweampuff);
            }
        }

        npcs.push(milk);

        Some(npcs.into_boxed_slice())
    }

    fn get_floor_modifications(&self, _cweampuff: &crate::Cweampuff) -> Option<Box<[FloorModification]>> {
        None
    }

    fn get_bgm(&self) -> Option<&'static str> {
        Some("forest")
    }
}