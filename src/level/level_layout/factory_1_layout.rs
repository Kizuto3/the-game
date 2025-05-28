use bevy::math::{Vec2, Vec3};

use crate::{level::{progression::Progression, Level}, npc::{conversation_entry::{ConversationEntry, ConversationPosition, Emotion}, CWEAMPUFF, DRONE, NPC}, CWEAMPUFF_Z_INDEX};

use super::{DoorCollider, EntityInfo, FloorAssetType, FloorInfo, FloorModification, LevelInfo, TransitionCollider};

#[derive(Clone, Copy)]
pub struct Factory1Info;

impl LevelInfo for Factory1Info {
    fn get_floor_info(&self, _cweampuff: &crate::Cweampuff) -> Box<[FloorInfo]> {
        Box::from([
            FloorInfo { position: Vec3::new(-2500.0, 0.0, 1.0), size: Vec2::new(300.0, 2600.0), breakable_wall: None, floor_asset: FloorAssetType::Factory },
            FloorInfo { position: Vec3::new(0.0, -1650.0, 1.0), size: Vec2::new(5000.0, 300.0), breakable_wall: None, floor_asset: FloorAssetType::Factory },
            FloorInfo { position: Vec3::new(150.0, 1150.0, 1.0), size: Vec2::new(5000.0, 300.0), breakable_wall: None, floor_asset: FloorAssetType::Factory },
            FloorInfo { position: Vec3::new(2500.0, -350.0, 1.0), size: Vec2::new(300.0, 2300.0), breakable_wall: None, floor_asset: FloorAssetType::Factory },
            FloorInfo { position: Vec3::new(-1675.0, -1100.0, 1.0), size: Vec2::new(150.0, 800.0), breakable_wall: None, floor_asset: FloorAssetType::Factory },
            FloorInfo { position: Vec3::new(-1525.0, -900.0, 1.0), size: Vec2::new(150.0, 1200.0), breakable_wall: None, floor_asset: FloorAssetType::Factory },
            FloorInfo { position: Vec3::new(-1900.0, -350.0, 1.0), size: Vec2::new(600.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Factory },
            FloorInfo { position: Vec3::new(-700.0, -1000.0, 1.0), size: Vec2::new(300.0, 1000.0), breakable_wall: None, floor_asset: FloorAssetType::Factory },
            FloorInfo { position: Vec3::new(-700.0, 375.0, 1.0), size: Vec2::new(300.0, 1250.0), breakable_wall: None, floor_asset: FloorAssetType::Factory },
            FloorInfo { position: Vec3::new(400.0, -850.0, 1.0), size: Vec2::new(300.0, 1300.0), breakable_wall: None, floor_asset: FloorAssetType::Factory },
            FloorInfo { position: Vec3::new(900.0, -850.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Factory },
            FloorInfo { position: Vec3::new(1300.0, -550.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Factory },
            FloorInfo { position: Vec3::new(2100.0, -350.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Factory },
            FloorInfo { position: Vec3::new(2050.0, 500.0, 1.0), size: Vec2::new(600.0, 600.0), breakable_wall: None, floor_asset: FloorAssetType::Factory },
            FloorInfo { position: Vec3::new(1250.0, 750.0, 1.0), size: Vec2::new(1000.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Factory },
            FloorInfo { position: Vec3::new(1000.0, 250.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Factory },
            FloorInfo { position: Vec3::new(200.0, 450.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Factory },
        ])
    }

    fn get_transitions_info(&self, _cweampuff: &crate::Cweampuff) -> Option<Box<[TransitionCollider]>> {
        Some(Box::from([
            TransitionCollider { exit_index: 0, safe_position: Vec3::new(-2450.0, -1450.0, CWEAMPUFF_Z_INDEX), transition_to_level: Level::FactoryTransition, floor_info: EntityInfo { position: Vec3::new(-2600.0, -1400.0, 2.0), size: Vec2::new(100.0, 200.0) }  },
            TransitionCollider { exit_index: 1, safe_position: Vec3::new(2450.0, 850.0, CWEAMPUFF_Z_INDEX), transition_to_level: Level::Factory2, floor_info: EntityInfo { position: Vec3::new(2600.0, 900.0, 2.0), size: Vec2::new(100.0, 200.0) }  },
        ]))
    }

    fn get_doors(&self, _cweampuff: &crate::Cweampuff) -> Option<Box<[DoorCollider]>> {
        None
    }

    fn get_npcs(&self, cweampuff: &crate::Cweampuff) -> Option<Box<[NPC]>> {        
        if cweampuff.progression == Progression::GivenLetter {
            return Some(Box::from([
                NPC {
                    name: DRONE,
                    floor_info: EntityInfo { position: Vec3::new(-1900.0, -1450.0, 0.0), size: Vec2::new(200.0, 100.0) }, is_active: false, current_conversation_index: 0,
                    after_conversation_func: |cweampuff, _commands, _breakable_walls, _cutscene| { cweampuff.has_wall_jump = true; },
                    conversation: &[
                        ConversationEntry { position: ConversationPosition::Right, npc_name: DRONE, text: "I LOVE OUR AI OVERLORD!!!!!", emotion: Emotion::Regular },
                        ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "W-what?..", emotion: Emotion::Sad },
                        ConversationEntry { position: ConversationPosition::Right, npc_name: DRONE, text: "SHE LOVES ME!!!!!", emotion: Emotion::Regular },
                        ConversationEntry { position: ConversationPosition::Right, npc_name: DRONE, text: "OUR AI OVERLORD WANTS MEEEEEEEEE!!!!!", emotion: Emotion::Regular },
                        ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "A-are you okay?..", emotion: Emotion::Sad },
                        ConversationEntry { position: ConversationPosition::Right, npc_name: DRONE, text: "I'VE NEVER BEEN BETTER!!!!!", emotion: Emotion::Regular },
                        ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "O-oh, I-I see...", emotion: Emotion::Sad },
                        ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Well, I think I have something you might like.", emotion: Emotion::Regular },
                        ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Look at this picture! It's my hidden gem and your AI overlord!", emotion: Emotion::Happy },
                        ConversationEntry { position: ConversationPosition::Right, npc_name: DRONE, text: "...", emotion: Emotion::Surprised },
                        ConversationEntry { position: ConversationPosition::Right, npc_name: DRONE, text: "THIS IS THE BEST THING I'VE EVER SEEN!!!", emotion: Emotion::Happy },
                        ConversationEntry { position: ConversationPosition::Right, npc_name: DRONE, text: "WHO IS YOUR HIDDEN GEM???", emotion: Emotion::Regular },
                        ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Isn't she the cutest thing?", emotion: Emotion::Happy },  //Not sure if "she" or "it" is a better fit. Are they talking about Milk, or just the picture? ~Blanc
                        ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "She's also very demure, intelligent, beautiful, kind, pure, soft, lovely, angelic, modest-", emotion: Emotion::Happy },
                        ConversationEntry { position: ConversationPosition::Right, npc_name: DRONE, text: "SHE WILL BE ASSIMILATED INTO THE SWARM!!!", emotion: Emotion::Regular },
                        ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "W-wait what?..", emotion: Emotion::Sad },
                        ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Is that a good thing?..", emotion: Emotion::Sad },
                        ConversationEntry { position: ConversationPosition::Right, npc_name: DRONE, text: "RESISTANCE IS FUTILE!!!", emotion: Emotion::Regular },
                        ConversationEntry { position: ConversationPosition::Right, npc_name: DRONE, text: "WE ARE THE SWARM THAT IS APPROACHING!!!", emotion: Emotion::Regular },
                        ConversationEntry { position: ConversationPosition::Right, npc_name: DRONE, text: "ALL WILL BE ONE!!!", emotion: Emotion::Regular },
                        ConversationEntry { position: ConversationPosition::Right, npc_name: DRONE, text: "OUR AI OVERLORD WILL NEVER BE LONELY!!!", emotion: Emotion::Regular },
                        ConversationEntry { position: ConversationPosition::Right, npc_name: DRONE, text: "NO ONE WILL EVER LEAVE HER!!!", emotion: Emotion::Happy },
                        ConversationEntry { position: ConversationPosition::Right, npc_name: DRONE, text: "*** Seeing how clingy Drone is makes you want to hug someone! Even the walls will do! ***", emotion: Emotion::Happy },
                    ]
                }
            ]))
        }
        
        None
    }

    fn get_floor_modifications(&self, _cweampuff: &crate::Cweampuff) -> Option<Box<[FloorModification]>> {
        None
    }

    fn get_bgm(&self) -> Option<&'static str> {
        Some("factory")
    }

    fn get_background(&self) -> FloorAssetType {
        FloorAssetType::Factory
    }
}