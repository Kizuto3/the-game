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
                ConversationEntry { position: ConversationPosition::Right, npc_name: OG_DRONE, text: "Welcome, Cweampuff! I'm glad you made your way here!", emotion: Emotion::Happy },
                ConversationEntry { position: ConversationPosition::Right, npc_name: OG_DRONE, text: "We all received that picture of your hidden gem and our AI overlord.", emotion: Emotion::Regular },
                ConversationEntry { position: ConversationPosition::Right, npc_name: OG_DRONE, text: "She's very adorable indeed!", emotion: Emotion::Happy },
                ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Really?", emotion: Emotion::Surprised },
                ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Then will you be able to help us cure her curse?", emotion: Emotion::Regular },
                ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "I don't want her to fall asleep again...", emotion: Emotion::Sad },
                ConversationEntry { position: ConversationPosition::Right, npc_name: OG_DRONE, text: "The curse of a hidden gem...", emotion: Emotion::Sad },
                ConversationEntry { position: ConversationPosition::Right, npc_name: OG_DRONE, text: "How much do you know about it?", emotion: Emotion::Regular },
                ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Not a whole lot...", emotion: Emotion::Sad },
                ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Every time I bring it up everyone just switches the topic...", emotion: Emotion::Sad },
                ConversationEntry { position: ConversationPosition::Right, npc_name: OG_DRONE, text: "Why do you think they do that?", emotion: Emotion::Regular },
                ConversationEntry { position: ConversationPosition::Right, npc_name: OG_DRONE, text: "Is it because they don't know about it?", emotion: Emotion::Regular },
                ConversationEntry { position: ConversationPosition::Right, npc_name: OG_DRONE, text: "Or is the curse something so bad they'd rather not talk about it?", emotion: Emotion::Regular },
                ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "I've heard that minawan's and the crew's rising stars were once afflicted with it...", emotion: Emotion::Sad },
                ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "The curse makes hidden gem fall asleep.", emotion: Emotion::Regular },
                ConversationEntry { position: ConversationPosition::Right, npc_name: OG_DRONE, text: "What do you think would happen if there's no one to wake a hidden gem up?", emotion: Emotion::Regular },
                ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "?..", emotion: Emotion::Sad },
                ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Oh no...", emotion: Emotion::Sad },
                ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "If a hidden gem's family isn't growing and there's no one to show how much they are loved...", emotion: Emotion::Sad },
                ConversationEntry { position: ConversationPosition::Right, npc_name: OG_DRONE, text: "Then the hidden gem will never wake up...", emotion: Emotion::Sad },
                ConversationEntry { position: ConversationPosition::Right, npc_name: OG_DRONE, text: "And when the last family member leaves, they become forgotten...", emotion: Emotion::Sad },
                ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "And thus, hidden gem disappears...", emotion: Emotion::Sad },
                ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Never to be found again...", emotion: Emotion::Sad },
                ConversationEntry { position: ConversationPosition::Right, npc_name: OG_DRONE, text: "Being hidden gem is a blessing and a curse.", emotion: Emotion::Regular },
                ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "So Miruku has to stop being a hidden gem to cure the curse?", emotion: Emotion::Regular },
                ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "But how?", emotion: Emotion::Regular },
                ConversationEntry { position: ConversationPosition::Right, npc_name: OG_DRONE, text: "You've traveled all over these lands. I'm sure you know the answer.", emotion: Emotion::Regular },
                ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "...", emotion: Emotion::Sad },
                ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "?..", emotion: Emotion::Regular },
                ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "!..", emotion: Emotion::Surprised },
                ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "She has to become a rising star!", emotion: Emotion::Surprised },
                ConversationEntry { position: ConversationPosition::Right, npc_name: OG_DRONE, text: "Exactly!", emotion: Emotion::Happy },
                ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "But how can we make her a rising star?", emotion: Emotion::Regular },
                ConversationEntry { position: ConversationPosition::Right, npc_name: OG_DRONE, text: "Isn't that what you've been doing this whole time?", emotion: Emotion::Regular },
                ConversationEntry { position: ConversationPosition::Right, npc_name: OG_DRONE, text: "Your family's grown quite a bit, hasn't it?", emotion: Emotion::Happy },
                ConversationEntry { position: ConversationPosition::Right, npc_name: OG_DRONE, text: "There's a lot more cweampuffs now.", emotion: Emotion::Regular },
                ConversationEntry { position: ConversationPosition::Right, npc_name: OG_DRONE, text: "Others too seem to love your hidden gem.", emotion: Emotion::Regular },
                ConversationEntry { position: ConversationPosition::Right, npc_name: OG_DRONE, text: "She just needs one final push from cweampuffs.", emotion: Emotion::Regular },
                ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "...", emotion: Emotion::Surprised },
                ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Thank you so much, Old Drone!", emotion: Emotion::Happy },
                ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "I need to run to my hidden gem now!", emotion: Emotion::Happy },
                ConversationEntry { position: ConversationPosition::Right, npc_name: OG_DRONE, text: "Go on, Cweampuff! I'll open the gates to your left!", emotion: Emotion::Happy },
                ConversationEntry { position: ConversationPosition::Right, npc_name: OG_DRONE, text: "Don't waste a second!", emotion: Emotion::Happy },
            ];
        }
        
        let clipper_drone = NPC { 
            floor_info: EntityInfo { position: Vec3::new(-1550.0, -650.0, 0.0), size: Vec2::new(200.0, 100.0) }, is_active: false, current_conversation_index: 0,
            after_conversation_func: |_cweampuff, _commands, _breakable_walls, _cutscene| { },
            conversation: &[
                ConversationEntry { position: ConversationPosition::Right, npc_name: CLIPPER_DRONE, text: "Your hidden gem will be quite an addition to my collection.", emotion: Emotion::Regular },
                ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Your collection?", emotion: Emotion::Regular },
                ConversationEntry { position: ConversationPosition::Right, npc_name: CLIPPER_DRONE, text: "I like to go around these lands and take pictures of interesting individuals.", emotion: Emotion::Regular },
                ConversationEntry { position: ConversationPosition::Right, npc_name: CLIPPER_DRONE, text: "Then I can show them to everyone.", emotion: Emotion::Happy },
                ConversationEntry { position: ConversationPosition::Right, npc_name: CLIPPER_DRONE, text: "A lot of us don't know about some hidden gems that live here.", emotion: Emotion::Regular },
                ConversationEntry { position: ConversationPosition::Right, npc_name: CLIPPER_DRONE, text: "This way, everyone can find them easily.", emotion: Emotion::Happy },
                ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "I would love for you to take pictures of my hidden gem!", emotion: Emotion::Happy },
                ConversationEntry { position: ConversationPosition::Right, npc_name: CLIPPER_DRONE, text: "I will!", emotion: Emotion::Happy },
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

    fn get_bgm(&self) -> Option<&'static str> {
        Some("factory")
    }
}