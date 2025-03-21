use bevy::math::{Vec2, Vec3};

use crate::{level::{progression::Progression, Level}, npc::{conversation_entry::{ConversationEntry, ConversationPosition, Emotion}, CLIPPER_DRONE, CWEAMPUFF, NPC, OG_DRONE}, CWEAMPUFF_Z_INDEX};

use super::{factory_4_layout::Factory4Info, factory_transition_layout::FactoryTransitionInfo, BreakableWall, DoorCollider, EntityInfo, FloorAssetType, FloorInfo, FloorModification, LevelInfo, TransitionCollider};

#[derive(Clone, Copy)]
pub struct NeuroLairInfo;

impl LevelInfo for NeuroLairInfo {
    fn get_floor_info(&self, cweampuff: &crate::Cweampuff) -> Box<[FloorInfo]> {
        let mut floors = vec![
            FloorInfo { position: Vec3::new(-2000.0, -450.0, 1.0), size: Vec2::new(300.0, 1300.0), breakable_wall: None, floor_asset: FloorAssetType::Factory },
            FloorInfo { position: Vec3::new(2000.0, 500.0, 1.0), size: Vec2::new(300.0, 2000.0), breakable_wall: None, floor_asset: FloorAssetType::Factory },
            FloorInfo { position: Vec3::new(125.0, -850.0, 1.0), size: Vec2::new(3950.0, 300.0), breakable_wall: None, floor_asset: FloorAssetType::Factory },
            FloorInfo { position: Vec3::new(-100.0, 850.0, 1.0), size: Vec2::new(3900.0, 300.0), breakable_wall: None, floor_asset: FloorAssetType::Factory },
            FloorInfo { position: Vec3::new(-1450.0, 0.0, 1.0), size: Vec2::new(800.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Factory },
            FloorInfo { position: Vec3::new(-1000.0, -500.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Factory },
            FloorInfo { position: Vec3::new(-600.0, -200.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Factory },
        ];

        if cweampuff.progression < Progression::RisingStar {
            floors.push(
                FloorInfo { position: Vec3::new(-2000.0, 450.0, 2.0), size: Vec2::new(300.0, 500.0), breakable_wall: Some(BreakableWall { index: 0 }), floor_asset: FloorAssetType::Factory }
            );
        }

        floors.into_boxed_slice()
    }

    fn get_transitions_info(&self, _cweampuff: &crate::Cweampuff) -> Option<Box<[TransitionCollider]>> {
        Some(Box::from([
            TransitionCollider { exit_index: 0, safe_position: Vec3::new(1950.0, -700.0, CWEAMPUFF_Z_INDEX), transition_to_level: Level::Factory4(Factory4Info), floor_info: EntityInfo { position: Vec3::new(2100.0, -700.0, 2.0), size: Vec2::new(200.0, 200.0) }  },
            TransitionCollider { exit_index: 1, safe_position: Vec3::new(-1900.0, 250.0, CWEAMPUFF_Z_INDEX), transition_to_level: Level::FactoryTransition(FactoryTransitionInfo), floor_info: EntityInfo { position: Vec3::new(-2100.0, 450.0, 0.0), size: Vec2::new(100.0, 500.0) }  },
        ]))
    }

    fn get_doors(&self, _cweampuff: &crate::Cweampuff) -> Option<Box<[DoorCollider]>> {
        None
    }

    fn get_npcs(&self, cweampuff: &crate::Cweampuff) -> Option<Box<[NPC]>> {
        let mut og_drone = NPC { floor_info: EntityInfo { position: Vec3::new(0.0, -650.0, 0.0), size: Vec2::new(200.0, 100.0) }, is_active: false, current_conversation_index: 0,
                                      conversation: &[], after_conversation_func: |_cweampuff, _commands, _breakable_walls, _cutscene| { }
        };

        if cweampuff.progression == Progression::GivenLetter {
            og_drone.after_conversation_func = |cweampuff, commands, breakable_walls, _cutscene| { 
                cweampuff.progression = Progression::RisingStar;

                for (entity, wall) in breakable_walls.iter() {
                    if wall.index == 0 {
                        commands.entity(entity).despawn();
                        break;
                    }
                }
            };
            og_drone.conversation = &[
                ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Hello!", emotion: Emotion::Happy },
                ConversationEntry { position: ConversationPosition::Right, npc_name: OG_DRONE, text: "Cweampuff! There you are!", emotion: Emotion::Happy },
            ];
        }
        
        let clipper_drone = NPC { 
            floor_info: EntityInfo { position: Vec3::new(-1550.0, -650.0, 0.0), size: Vec2::new(200.0, 100.0) }, is_active: false, current_conversation_index: 0,
            after_conversation_func: |_cweampuff, _commands, _breakable_walls, _cutscene| { },
            conversation: &[
                ConversationEntry { position: ConversationPosition::Right, npc_name: CLIPPER_DRONE, text: "I love our captain.", emotion: Emotion::Regular },
                ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "That sounds so cool!", emotion: Emotion::Happy },
            ]
        };

        Some(Box::from([
            og_drone,
            clipper_drone
        ]))
    }
    
    fn get_floor_modifications(&self, _cweampuff: &crate::Cweampuff) -> Option<Box<[FloorModification]>> {
        None
    }
}