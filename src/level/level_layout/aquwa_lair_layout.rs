use bevy::math::{Vec2, Vec3};

use crate::{level::{progression::Progression, Level}, npc::{conversation_entry::{ConversationEntry, ConversationPosition, Emotion}, CWEAMPUFF, NPC, OBSERVER_CREW_MEMBER, OG_CREW_MEMBER}, CWEAMPUFF_Z_INDEX};
use super::{BreakableWall, DoorCollider, DoorType, EntityInfo, FloorAssetType, FloorInfo, FloorModification, LevelInfo, TransitionCollider};

#[derive(Clone, Copy)]
pub struct AquwaLairInfo;

impl LevelInfo for AquwaLairInfo {
    fn get_floor_info(&self, cweampuff: &crate::Cweampuff) -> Box<[FloorInfo]> {
        let mut floors = vec![
            FloorInfo { position: Vec3::new(-2000.0, 0.0, 1.0), size: Vec2::new(300.0, 2000.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(2000.0, 500.0, 1.0), size: Vec2::new(300.0, 2000.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(125.0, -850.0, 1.0), size: Vec2::new(3950.0, 300.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(0.0, 850.0, 1.0), size: Vec2::new(3700.0, 300.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(-1450.0, 0.0, 1.0), size: Vec2::new(800.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(-1000.0, -500.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
            FloorInfo { position: Vec3::new(-600.0, -200.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Spaceship },
        ];

        if cweampuff.progression < Progression::HasLetter {
            floors.push(
                FloorInfo { position: Vec3::new(-1550.0, -550.0, 2.0), size: Vec2::new(300.0, 300.0), breakable_wall: Some(BreakableWall { index: 0 }), floor_asset: FloorAssetType::Spaceship }
            );
        }

        floors.into_boxed_slice()
    }

    fn get_transitions_info(&self, _cweampuff: &crate::Cweampuff) -> Option<Box<[TransitionCollider]>> {
        Some(Box::from([
            TransitionCollider { exit_index: 1, safe_position: Vec3::new(1950.0, -700.0, CWEAMPUFF_Z_INDEX), transition_to_level: Level::Spaceship4, floor_info: EntityInfo { position: Vec3::new(2100.0, -700.0, 2.0), size: Vec2::new(200.0, 200.0) }  },
        ]))
    }

    fn get_doors(&self, _cweampuff: &crate::Cweampuff) -> Option<Box<[DoorCollider]>> {
        Some(Box::from([
            DoorCollider { floor_info: EntityInfo { position: Vec3 { x: -1550.0, y: -600.0, z: 0.0 }, size: Vec2 { x: 100., y: 200. } }, door_type: DoorType::Teleport,
                transition_to_level: Level::CweamcatLair, safe_position: Vec3 { x: 2450., y: 1550., z: CWEAMPUFF_Z_INDEX }, is_active: false }
        ]))
    }

    fn get_npcs(&self, cweampuff: &crate::Cweampuff) -> Option<Box<[NPC]>> {
        let mut og_crew_member = NPC { floor_info: EntityInfo { position: Vec3::new(0.0, -650.0, 0.0), size: Vec2::new(200.0, 100.0) }, is_active: false, current_conversation_index: 0,
                                            name: OG_CREW_MEMBER,
                                            conversation: &[], after_conversation_func: |_cweampuff, _commands, _breakable_walls, _cutscene| { }
        };

        if cweampuff.progression == Progression::MilkWokeUp {
            og_crew_member.after_conversation_func = |cweampuff, commands, breakable_walls, _cutscene| { 
                cweampuff.progression = Progression::HasLetter;

                for (entity, wall) in breakable_walls.iter() {
                    if wall.index == 0 {
                        commands.entity(entity).despawn();
                        break;
                    }
                }
            };
            og_crew_member.conversation = &[
                ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Hello!", emotion: Emotion::Happy },
                ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CREW_MEMBER, text: "Cweampuff! There you are!", emotion: Emotion::Happy },
                ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CREW_MEMBER, text: "The Crew told me about you.", emotion: Emotion::Regular },
                ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CREW_MEMBER, text: "Thank you for volunteering to find the intruder.", emotion: Emotion::Regular },
                ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Of course!", emotion: Emotion::Happy },
                ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "I've found them in the room next to this one.", emotion: Emotion::Regular },
                ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CREW_MEMBER, text: "You have?", emotion: Emotion::Surprised },
                ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CREW_MEMBER, text: "That green bastard...", emotion: Emotion::Regular },
                ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CREW_MEMBER, text: "Don't worry, we'll handle them. Our rising star will be safe.", emotion: Emotion::Regular },
                ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "With all of you around, she has nothing to worry about!", emotion: Emotion::Happy },
                ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CREW_MEMBER, text: "I hope so.", emotion: Emotion::Regular },
                ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CREW_MEMBER, text: "We treasure our rising star very deeply!", emotion: Emotion::Happy },
                ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CREW_MEMBER, text: "She always takes us to places far and wide on this spaceship.", emotion: Emotion::Regular },
                ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CREW_MEMBER, text: "We get to see all kinds of things.", emotion: Emotion::Regular },
                ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CREW_MEMBER, text: "But most of all, I like to see our Captain happy!", emotion: Emotion::Happy },
                ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "I know exactly what you mean!", emotion: Emotion::Happy },
                ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CREW_MEMBER, text: "Speaking of our Captain, we informed her about your request.", emotion: Emotion::Regular },
                ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CREW_MEMBER, text: "She asked us to give you this letter in a bottle.", emotion: Emotion::Regular },
                ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CREW_MEMBER, text: "'Once, this very thing was a beacon for the new members of the Crew.' - she said", emotion: Emotion::Regular },
                ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CREW_MEMBER, text: "'May it serve their hidden gem as well as it served me.'", emotion: Emotion::Regular },
                ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Oh!", emotion: Emotion::Surprised },
                ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "I hope this brings many new Cweampuffs to my hidden gem!", emotion: Emotion::Happy },
                ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Thank you so much Captain and the Crew!", emotion: Emotion::Happy },
                ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CREW_MEMBER, text: "You are more than welcome, Cweampuff!", emotion: Emotion::Happy },
                ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CREW_MEMBER, text: "Now then, hurry to your hidden gem. I'll activate the teleporter for you.", emotion: Emotion::Regular },
                ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "I will deliver this letter as soon as possible!", emotion: Emotion::Happy },
                ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "See you!", emotion: Emotion::Happy },
                ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CREW_MEMBER, text: "Do come again!", emotion: Emotion::Happy },
                ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CREW_MEMBER, text: "*** You received a letter in a bottle from the spaceship's Captain. ***", emotion: Emotion::Happy },
            ];
        }
        
        let observer_crew_member = NPC {
            name: OBSERVER_CREW_MEMBER,
            floor_info: EntityInfo { position: Vec3::new(-1550.0, 100.0, 0.0), size: Vec2::new(200.0, 100.0) }, is_active: false, current_conversation_index: 0,
            after_conversation_func: |_cweampuff, _commands, _breakable_walls, _cutscene| { },
            conversation: &[
                ConversationEntry { position: ConversationPosition::Right, npc_name: OBSERVER_CREW_MEMBER, text: "I love our Captain.", emotion: Emotion::Regular },
                ConversationEntry { position: ConversationPosition::Right, npc_name: OBSERVER_CREW_MEMBER, text: "She always finds interesting things all around the universe.", emotion: Emotion::Regular },
                ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "That sounds so cool!", emotion: Emotion::Happy },
                ConversationEntry { position: ConversationPosition::Right, npc_name: OBSERVER_CREW_MEMBER, text: "Although, recently, she's been very interested in someone's back.", emotion: Emotion::Regular },
                ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "What? Is that some sort of a metaphor?", emotion: Emotion::Regular },
                ConversationEntry { position: ConversationPosition::Right, npc_name: OBSERVER_CREW_MEMBER, text: "I wish it was, but I mean it quite literally.", emotion: Emotion::Regular },
                ConversationEntry { position: ConversationPosition::Right, npc_name: OBSERVER_CREW_MEMBER, text: "She just keeps bringing it up.", emotion: Emotion::Regular },
                ConversationEntry { position: ConversationPosition::Right, npc_name: OBSERVER_CREW_MEMBER, text: "Have you seen that big telescope? She's been constantly looking through it to get a glimpse of his back.", emotion: Emotion::Regular },
                ConversationEntry { position: ConversationPosition::Right, npc_name: OBSERVER_CREW_MEMBER, text: "We even came to this planet in search of that green guy.", emotion: Emotion::Regular },
                ConversationEntry { position: ConversationPosition::Right, npc_name: OBSERVER_CREW_MEMBER, text: "I'm still having fun though, so no reason to complain.", emotion: Emotion::Regular },
                ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Well, I hope she finds him!", emotion: Emotion::Happy },
            ]
        };

        Some(Box::from([
            og_crew_member,
            observer_crew_member
        ]))
    }
    
    fn get_floor_modifications(&self, _cweampuff: &crate::Cweampuff) -> Option<Box<[FloorModification]>> {
        None
    }

    fn get_bgm(&self) -> Option<&'static str> {
        Some("spaceship")
    }

    fn get_background(&self) -> FloorAssetType {
        FloorAssetType::Spaceship
    }
}