use bevy::math::{Vec2, Vec3};

use crate::{level::Level, npc::{conversation_entry::{ConversationEntry, ConversationPosition, Emotion}, CWEAMPUFF, GRIM, NPC}, CWEAMPUFF_Z_INDEX};

use super::{factory_3_layout::Factory3Info, Decoration, DoorCollider, EntityInfo, FloorAssetType, FloorInfo, FloorModification, LevelInfo, TransitionCollider};

#[derive(Clone, Copy)]
pub struct FactoryHiddenLevelInfo;

impl LevelInfo for FactoryHiddenLevelInfo {
    fn get_floor_info(&self, _cweampuff: &crate::Cweampuff) -> Box<[FloorInfo]> {
        Box::from([
            FloorInfo { position: Vec3::new(-1200.0, 550.0, 1.0), size: Vec2::new(400.0, 1500.0), breakable_wall: None, floor_asset: FloorAssetType::Factory },
            FloorInfo { position: Vec3::new(0.0, 600.0, 1.0), size: Vec2::new(2000.0, 400.0), breakable_wall: None, floor_asset: FloorAssetType::Factory },
            FloorInfo { position: Vec3::new(1200.0, 0.0, 1.0), size: Vec2::new(400.0, 1500.0), breakable_wall: None, floor_asset: FloorAssetType::Factory },
            FloorInfo { position: Vec3::new(-200.0, -600.0, 1.0), size: Vec2::new(2400.0, 400.0), breakable_wall: None, floor_asset: FloorAssetType::Factory },
        ])
    }

    fn get_transitions_info(&self, _cweampuff: &crate::Cweampuff) -> Option<Box<[TransitionCollider]>> {
        Some(Box::from([
            TransitionCollider { exit_index: 2, safe_position: Vec3::new(-1100.0, -350.0, CWEAMPUFF_Z_INDEX), transition_to_level: Level::Factory3(Factory3Info), floor_info: EntityInfo { position: Vec3::new(-1300.0, -300.0, 2.0), size: Vec2::new(100.0, 200.0) }  },
        ]))
    }

    fn get_doors(&self, _cweampuff: &crate::Cweampuff) -> Option<Box<[DoorCollider]>> {
        None
    }

    fn get_npcs(&self, _cweampuff: &crate::Cweampuff) -> Option<Box<[NPC]>> {
        let grim = NPC { floor_info: EntityInfo { position: Vec3::new(750.0, -350.0, 0.0), size: Vec2::new(200.0, 100.0) }, is_active: false, current_conversation_index: 0,
                            name: GRIM,
                            conversation: &[
                                ConversationEntry { position: ConversationPosition::Right, npc_name: GRIM, text: "Hello Cweampuff, I am your father!", emotion: Emotion::Happy },
                                ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "W-what?!", emotion: Emotion::Sad },
                                ConversationEntry { position: ConversationPosition::Right, npc_name: GRIM, text: "Is that reference too old for you?", emotion: Emotion::Sad },
                                ConversationEntry { position: ConversationPosition::Right, npc_name: GRIM, text: "Anyways, welcome to the factory!", emotion: Emotion::Regular },
                                ConversationEntry { position: ConversationPosition::Right, npc_name: GRIM, text: "I appreciate you coming here for a bit, but you are close to your goal and Miruku needs you right now, so keep moving!", emotion: Emotion::Happy },
                            ], 
                            after_conversation_func: |_cweampuff, _commands, _breakable_walls, _cutscene| { },
        };

        Some(Box::from([
            grim
        ]))
    }
    
    fn get_floor_modifications(&self, _cweampuff: &crate::Cweampuff) -> Option<Box<[FloorModification]>> {
        Some(Box::from([
            FloorModification::Decoration(
                Decoration { position: Vec3::new(0.0, 0.0, 1.0), size: Vec2::new(888.0, 500.0), asset: "Picture1" }
            )
        ]))
    }

    fn get_bgm(&self) -> Option<&'static str> {
        Some("factory_vocals")
    }

    fn get_background(&self) -> FloorAssetType {
        FloorAssetType::Factory
    }
}