use bevy::math::{Vec2, Vec3};

use crate::{level::{level_layout::cweamcat_lair_layout::CweamcatLairInfo, progression::Progression, Level}, npc::{conversation_entry::{ConversationEntry, ConversationPosition, Emotion}, CREW_MEMBER_1, CWEAMPUFF, NPC}};

use super::{DoorCollider, EntityInfo, FloorAssetType, FloorInfo, FloorModification, LevelInfo, TransitionCollider};

#[derive(Clone, Copy)]
pub struct Spaceship1Info;

impl LevelInfo for Spaceship1Info {
    fn get_floor_info(&self, _cweampuff: &crate::Cweampuff) -> Box<[FloorInfo]> {
        Box::from([
            FloorInfo { position: Vec3::new(-150.0, 1900.0, 1.0), size: Vec2::new(12300.0, 300.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(-6150.0, -1200.0, 1.0), size: Vec2::new(300.0, 2000.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(-6150.0, 875.0, 1.0), size: Vec2::new(300.0, 1750.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(150.0, -2200.0, 1.0), size: Vec2::new(11700.0, 300.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(6150.0, 0.0, 1.0), size: Vec2::new(300.0, 6000.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(-5125.0, -350.0, 1.0), size: Vec2::new(1750.0, 300.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(-2000.0, -350.0, 1.0), size: Vec2::new(1100.0, 300.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(1000.0, -350.0, 1.0), size: Vec2::new(1650.0, 300.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(4000.0, -350.0, 1.0), size: Vec2::new(1200.0, 300.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(-4500.0, -1900.0, 1.0), size: Vec2::new(200.0, 300.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(-2950.0, -1800.0, 1.0), size: Vec2::new(300.0, 500.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(-2500.0, -1800.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(-3200.0, -1300.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(-1200.0, -1700.0, 1.0), size: Vec2::new(200.0, 700.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(-800.0, -1600.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(-400.0, -1800.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(100.0, -1250.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(-1000.0, -1000.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(100.0, -750.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(2000.0, -1450.0, 1.0), size: Vec2::new(300.0, 1200.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(3500.0, -750.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(5000.0, -850.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(2800.0, -1800.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(3200.0, -1550.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(4500.0, -1350.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(3500.0, -1050.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(5925.0, -550.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(5500.0, -350.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(4000.0, 50.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(5000.0, 250.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(5500.0, 550.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(4500.0, 750.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(3500.0, 850.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(3000.0, 1250.0, 1.0), size: Vec2::new(150.0, 1000.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(1725.0, 200.0, 1.0), size: Vec2::new(200.0, 800.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(-1550.0, 200.0, 1.0), size: Vec2::new(200.0, 800.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(1200.0, 0.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(1450.0, 300.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(1200.0, 800.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(600.0, 600.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(300.0, 300.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(-700.0, 300.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
        ])
    }

    fn get_transitions_info(&self, _cweampuff: &crate::Cweampuff) -> Option<Box<[TransitionCollider]>> {
        Some(Box::from([
            TransitionCollider { exit_index: 3, safe_position: Vec3::new(-5600.0, -1950.0, 1.0), transition_to_level: Level::CweamcatLair(CweamcatLairInfo), floor_info: EntityInfo { position: Vec3::new(-5850.0, -2300.0, 2.0), size: Vec2::new(300.0, 100.0) }  }
        ]))
    }

    fn get_doors(&self, _cweampuff: &crate::Cweampuff) -> Option<Box<[DoorCollider]>> {
        None
    }

    fn get_npcs(&self, cweampuff: &crate::Cweampuff) -> Option<Box<[NPC]>> {
        if cweampuff.progression <= Progression::MilkWokeUp {
            return Some(Box::from([
                NPC {
                    floor_info: EntityInfo { position: Vec3::new(-5000.0, -2000.0, 0.0), size: Vec2::new(200.0, 100.0) }, is_active: false, current_conversation_index: 0,
                    after_conversation_func: |cweampuff, _commands, _breakable_walls, _cutscene| { cweampuff.has_dash = true; },
                    conversation: &[
                        ConversationEntry { position: ConversationPosition::Right, npc_name: CREW_MEMBER_1, text: "Pumpkin to Otter, Pumpkin to Otter, do you copy, over?", emotion: Emotion::Regular },
                        ConversationEntry { position: ConversationPosition::Right, npc_name: CREW_MEMBER_1, text: "The last entrance has been shut, we are ready to dispatch a search, over.", emotion: Emotion::Regular },
                        ConversationEntry { position: ConversationPosition::Right, npc_name: CREW_MEMBER_1, text: "...", emotion: Emotion::Regular },
                        ConversationEntry { position: ConversationPosition::Right, npc_name: CREW_MEMBER_1, text: "Roger! Protect the captain at all cost, over and out!", emotion: Emotion::Regular },
                        ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "H-hello? What's going on?", emotion: Emotion::Sad },
                        ConversationEntry { position: ConversationPosition::Right, npc_name: CREW_MEMBER_1, text: "Oh, greetings.", emotion: Emotion::Regular },
                        ConversationEntry { position: ConversationPosition::Right, npc_name: CREW_MEMBER_1, text: "Sorry, we can't exactly welcome you properly right now.", emotion: Emotion::Sad },
                        ConversationEntry { position: ConversationPosition::Right, npc_name: CREW_MEMBER_1, text: "There is an intruder on our ship. Anyway, what brought you here?", emotion: Emotion::Regular },
                        ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "I-it's my hidden gem. She's just woken up from her sleep.", emotion: Emotion::Regular },
                        ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "I think it's some sort of a curse...", emotion: Emotion::Sad },
                        ConversationEntry { position: ConversationPosition::Right, npc_name: CREW_MEMBER_1, text: "Ah, the curse of a hidden gem.", emotion: Emotion::Regular },
                        ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "You know about it?", emotion: Emotion::Regular },
                        ConversationEntry { position: ConversationPosition::Right, npc_name: CREW_MEMBER_1, text: "I do. Our rising star has been afflicted once.", emotion: Emotion::Regular },
                        ConversationEntry { position: ConversationPosition::Right, npc_name: CREW_MEMBER_1, text: "Thankfully, there was enough of us, and we cured our captain!", emotion: Emotion::Happy },
                        ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "So the number does matter...", emotion: Emotion::Regular },
                        ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "I'm looking for an idea of how to get more cweampuffs to notice our hidden gem.", emotion: Emotion::Regular },
                        ConversationEntry { position: ConversationPosition::Right, npc_name: CREW_MEMBER_1, text: "I'm sure we can help you!", emotion: Emotion::Happy },
                        ConversationEntry { position: ConversationPosition::Right, npc_name: CREW_MEMBER_1, text: "However, we have to deal with the intruder first.", emotion: Emotion::Regular },
                        ConversationEntry { position: ConversationPosition::Right, npc_name: CREW_MEMBER_1, text: "I won't allow anything bad happening to our captain!", emotion: Emotion::Regular },
                        ConversationEntry { position: ConversationPosition::Right, npc_name: CREW_MEMBER_1, text: "I'll protect her smile!", emotion: Emotion::Happy },
                        ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "I can help you look for the intruder!", emotion: Emotion::Happy },
                        ConversationEntry { position: ConversationPosition::Right, npc_name: CREW_MEMBER_1, text: "Then let's rush to her help immediately!", emotion: Emotion::Happy },
                        ConversationEntry { position: ConversationPosition::Right, npc_name: CREW_MEMBER_1, text: "*** Seeing how the crew doesn't waste a second to help their captain makes you want to dash with them. ***", emotion: Emotion::Happy },
                        ConversationEntry { position: ConversationPosition::Right, npc_name: CREW_MEMBER_1, text: "*** X - dash ***", emotion: Emotion::Happy },
                    ]
                }
            ]))
        }
        
        None
    }

    fn get_floor_modifications(&self, _cweampuff: &crate::Cweampuff) -> Option<Box<[FloorModification]>> {
        None
    }
}