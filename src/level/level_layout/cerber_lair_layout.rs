use bevy::math::{Vec2, Vec3};

use crate::{level::{level_layout::cweamcat_lair_layout::CweamcatLairInfo, progression::Progression, Level}, npc::{conversation_entry::{ConversationEntry, ConversationPosition, Emotion}, CWEAMPUFF, NPC, OG_MINAWAN, SCIENTIST_MINAWAN}, CWEAMPUFF_Z_INDEX};

use super::{hell_4_layout::Hell4Info, BreakableWall, DoorCollider, EntityInfo, FloorAssetType, FloorInfo, FloorModification, JumpPad, LevelInfo, TransitionCollider};

#[derive(Clone, Copy)]
pub struct CerberLairInfo;

impl LevelInfo for CerberLairInfo {
    fn get_floor_info(&self, cweampuff: &crate::Cweampuff) -> Box<[FloorInfo]> {
        let mut floors = vec![
            FloorInfo { position: Vec3::new(-2000.0, 0.0, 1.0), size: Vec2::new(300.0, 2000.0), breakable_wall: None, floor_asset: FloorAssetType::Hell },
            FloorInfo { position: Vec3::new(2000.0, 500.0, 1.0), size: Vec2::new(300.0, 2000.0), breakable_wall: None, floor_asset: FloorAssetType::Hell },
            FloorInfo { position: Vec3::new(125.0, -850.0, 1.0), size: Vec2::new(3950.0, 300.0), breakable_wall: None, floor_asset: FloorAssetType::Hell },
            FloorInfo { position: Vec3::new(-100.0, 850.0, 1.0), size: Vec2::new(3500.0, 300.0), breakable_wall: None, floor_asset: FloorAssetType::Hell },
            FloorInfo { position: Vec3::new(1450.0, -450.0, 1.0), size: Vec2::new(200.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Hell },
            FloorInfo { position: Vec3::new(950.0, -150.0, 1.0), size: Vec2::new(200.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Hell },
            FloorInfo { position: Vec3::new(1750.0, -50.0, 1.0), size: Vec2::new(200.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Hell },
            FloorInfo { position: Vec3::new(-1450.0, 0.0, 1.0), size: Vec2::new(800.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Hell },
            FloorInfo { position: Vec3::new(-1000.0, -250.0, 1.0), size: Vec2::new(100.0, 600.0), breakable_wall: None, floor_asset: FloorAssetType::Hell },
        ];

        if cweampuff.progression < Progression::HasCherish {
            floors.push(
                FloorInfo { position: Vec3::new(1750.0, 800.0, 2.0), size: Vec2::new(300.0, 200.0), breakable_wall: Some(BreakableWall { index: 0 }), floor_asset: FloorAssetType::Hell }
            );
        }

        floors.into_boxed_slice()
    }

    fn get_transitions_info(&self, _cweampuff: &crate::Cweampuff) -> Option<Box<[TransitionCollider]>> {
        Some(Box::from([
            TransitionCollider { exit_index: 1, safe_position: Vec3::new(2050.0, -700.0, CWEAMPUFF_Z_INDEX), transition_to_level: Level::Hell4(Hell4Info), floor_info: EntityInfo { position: Vec3::new(2200.0, -700.0, 2.0), size: Vec2::new(200.0, 200.0) }  },
            TransitionCollider { exit_index: 2, safe_position: Vec3::new(1750.0, 800.0, CWEAMPUFF_Z_INDEX), transition_to_level: Level::CweamcatLair(CweamcatLairInfo), floor_info: EntityInfo { position: Vec3::new(1750.0, 950.0, 2.0), size: Vec2::new(200.0, 200.0) }  }
        ]))
    }

    fn get_doors(&self, _cweampuff: &crate::Cweampuff) -> Option<Box<[DoorCollider]>> {
        None
    }

    fn get_npcs(&self, cweampuff: &crate::Cweampuff) -> Option<Box<[NPC]>> {
        let mut og_minawan = NPC { floor_info: EntityInfo { position: Vec3::new(0.0, -650.0, 0.0), size: Vec2::new(200.0, 100.0) }, is_active: false, current_conversation_index: 0,
                                      conversation: &[], after_conversation_func: |_cweampuff, _commands, _breakable_walls, _cutscene| { },
                                      name: OG_MINAWAN
        };

        if cweampuff.progression == Progression::MetMilk {
            og_minawan.after_conversation_func = |cweampuff, commands, breakable_walls, _cutscene| { 
                cweampuff.progression = Progression::HasCherish;

                for (entity, wall) in breakable_walls.iter() {
                    if wall.index == 0 {
                        commands.entity(entity).despawn();
                        break;
                    }
                }
            };
            og_minawan.conversation = &[
                ConversationEntry { position: ConversationPosition::Right, npc_name: OG_MINAWAN, text: "Wan! Wan!", emotion: Emotion::Happy },
                ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Wan wan!", emotion: Emotion::Happy },
                ConversationEntry { position: ConversationPosition::Right, npc_name: OG_MINAWAN, text: "Oh how glad I am to see a new face here!", emotion: Emotion::Happy },
                ConversationEntry { position: ConversationPosition::Right, npc_name: OG_MINAWAN, text: "Has wan came here to witness the cuteness of our rising star?", emotion: Emotion::Regular },
                ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Well... It's my hidden gem... She's sleeping.", emotion: Emotion::Sad },
                ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "I was told you might be able to help.", emotion: Emotion::Sad },
                ConversationEntry { position: ConversationPosition::Right, npc_name: OG_MINAWAN, text: "Sleeping hidden gem...", emotion: Emotion::Sad },
                ConversationEntry { position: ConversationPosition::Right, npc_name: OG_MINAWAN, text: "Let me tell you a story.", emotion: Emotion::Regular },
                ConversationEntry { position: ConversationPosition::Right, npc_name: OG_MINAWAN, text: "You see, when our rising star was but a little puppy, there weren't that many of us.", emotion: Emotion::Regular },
                ConversationEntry { position: ConversationPosition::Right, npc_name: OG_MINAWAN, text: "It was tough. We've been through a lot of ups and downs.", emotion: Emotion::Sad },
                ConversationEntry { position: ConversationPosition::Right, npc_name: OG_MINAWAN, text: "But she always did her best to take care of us,", emotion: Emotion::Happy },
                ConversationEntry { position: ConversationPosition::Right, npc_name: OG_MINAWAN, text: "and we never stopped supporting her.", emotion: Emotion::Regular },
                ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "That's beautiful!", emotion: Emotion::Happy },
                ConversationEntry { position: ConversationPosition::Right, npc_name: OG_MINAWAN, text: "Indeed!", emotion: Emotion::Happy },
                ConversationEntry { position: ConversationPosition::Right, npc_name: OG_MINAWAN, text: "Eventually, more and more minawan found her.", emotion: Emotion::Regular },
                ConversationEntry { position: ConversationPosition::Right, npc_name: OG_MINAWAN, text: "Together, we'd always make sure she knew how much she is appreciated.", emotion: Emotion::Regular },
                ConversationEntry { position: ConversationPosition::Right, npc_name: OG_MINAWAN, text: "And thus, she became a rising star! Look how bright she shines!", emotion: Emotion::Happy },
                ConversationEntry { position: ConversationPosition::Right, npc_name: OG_MINAWAN, text: "Thankfully, she's never fallen asleep.", emotion: Emotion::Regular },
                ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Oh, so you don't know how to wake my hidden gem up?", emotion: Emotion::Sad },
                ConversationEntry { position: ConversationPosition::Right, npc_name: OG_MINAWAN, text: "I don't, I'm afraid.", emotion: Emotion::Sad },
                ConversationEntry { position: ConversationPosition::Right, npc_name: OG_MINAWAN, text: "However, more than ever before, your hidden gem needs you. All of you.", emotion: Emotion::Regular },
                ConversationEntry { position: ConversationPosition::Right, npc_name: OG_MINAWAN, text: "She depends on you.", emotion: Emotion::Regular },
                ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "But what can I do?", emotion: Emotion::Sad },
                ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "The moment I saw her, I knew it was fate.", emotion: Emotion::Regular },
                ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "She is the one for me, and I want to protect her, and the home she built.", emotion: Emotion::Regular },
                ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Is there really nothing I can do?..", emotion: Emotion::Sad },
                ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "I just want to see her happy...", emotion: Emotion::Sad },
                ConversationEntry { position: ConversationPosition::Right, npc_name: OG_MINAWAN, text: "That is all you need!", emotion: Emotion::Happy },
                ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Is that so?..", emotion: Emotion::Sad },
                ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "She's my hidden gem...", emotion: Emotion::Regular },
                ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "I will do everything I can and more to see her smile!", emotion: Emotion::Happy },
                ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "*** Your desire to help your hidden gem has grown ***", emotion: Emotion::Happy },
                ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "*** After going through Hell, seeing how minawan love their rising star, and hearing their words of encouragement... ***", emotion: Emotion::Happy },
                ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "*** You have learnt to cherish ***", emotion: Emotion::Happy },
                ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "...", emotion: Emotion::Surprised },
                ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "I need to see my hidden gem right away!", emotion: Emotion::Surprised },
                ConversationEntry { position: ConversationPosition::Right, npc_name: OG_MINAWAN, text: "So you've found your answer!", emotion: Emotion::Happy },
                ConversationEntry { position: ConversationPosition::Right, npc_name: OG_MINAWAN, text: "Go on then, I'll unlock a door for you. It's right above us.", emotion: Emotion::Regular },
                ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Thank you so much Old Minawan! And thank you, all of the minawan of Hell!", emotion: Emotion::Happy },
            ];
        }
        
        let scientist_minawan = NPC { 
            floor_info: EntityInfo { position: Vec3::new(-1550.0, -650.0, 0.0), size: Vec2::new(200.0, 100.0) }, is_active: false, current_conversation_index: 0,
            after_conversation_func: |_cweampuff, _commands, _breakable_walls, _cutscene| { },
            conversation: &[
                ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Wan! Wan!", emotion: Emotion::Happy },
                ConversationEntry { position: ConversationPosition::Right, npc_name: SCIENTIST_MINAWAN, text: "...", emotion: Emotion::Regular },
                ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Wan Wan?..", emotion: Emotion::Regular },
                ConversationEntry { position: ConversationPosition::Right, npc_name: SCIENTIST_MINAWAN, text: "Oh, I beg your pardon, I hadn't noticed you.", emotion: Emotion::Regular },
                ConversationEntry { position: ConversationPosition::Right, npc_name: SCIENTIST_MINAWAN, text: "Excuse me, but I am very busy at the moment.", emotion: Emotion::Regular },
                ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "What are you doing?", emotion: Emotion::Regular },
                ConversationEntry { position: ConversationPosition::Right, npc_name: SCIENTIST_MINAWAN, text: "I am very close to a scientific breakthrough.", emotion: Emotion::Regular },
                ConversationEntry { position: ConversationPosition::Right, npc_name: SCIENTIST_MINAWAN, text: "You see, our rising star exudes unnatural amounts of cuteness.", emotion: Emotion::Regular },
                ConversationEntry { position: ConversationPosition::Right, npc_name: SCIENTIST_MINAWAN, text: "I almost figured out how wan can be this cute.", emotion: Emotion::Regular },
                ConversationEntry { position: ConversationPosition::Right, npc_name: SCIENTIST_MINAWAN, text: "But some things are still missing.", emotion: Emotion::Regular },
                ConversationEntry { position: ConversationPosition::Right, npc_name: SCIENTIST_MINAWAN, text: "Excuse me, I have to get back to work.", emotion: Emotion::Regular },
                ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Good luck with your research!", emotion: Emotion::Happy },
            ],
            name: SCIENTIST_MINAWAN
        };

        Some(Box::from([
            og_minawan,
            scientist_minawan
        ]))
    }
    
    fn get_floor_modifications(&self, _cweampuff: &crate::Cweampuff) -> Option<Box<[FloorModification]>> {
        Some(Box::from([
            FloorModification::JumpPad(JumpPad { floor_info: EntityInfo { position: Vec3::new(1750.0, 100.0, 0.0), size: Vec2::new(200.0, 200.0) } }),
        ]))
    }

    fn get_bgm(&self) -> Option<&'static str> {
        Some("hell")
    }

    fn get_background(&self) -> FloorAssetType {
        FloorAssetType::Hell
    }
}