use bevy::math::{Vec2, Vec3};

use crate::{level::{level_layout::{cweamcat_house_layout::CweamcatHouseInfo, hell_1_layout::Hell1Info, starting_room_layout::StartingRoomInfo}, progression::Progression, Level}, npc::{conversation_entry::{ConversationEntry, ConversationPosition, Emotion}, COOL_CWEAMPUFF, CREW_MEMBER, CWEAMPUFF, DRONE, MASKED_CWEAMPUFF, MINAWAN, NPC, OG_CWEAMPUFF, RICH_CWEAMPUFF}, CWEAMPUFF_Z_INDEX};

use super::{cerber_lair_layout::CerberLairInfo, factory_transition_layout::FactoryTransitionInfo, spaceship_1_layout::Spaceship1Info, DoorCollider, DoorType, EntityInfo, FloorAssetType, FloorInfo, FloorModification, LevelInfo, TransitionCollider};

#[derive(Clone, Copy)]
pub struct CweamcatLairInfo;

impl LevelInfo for CweamcatLairInfo {
    fn get_floor_info(&self, cweampuff: &crate::Cweampuff) -> Box<[FloorInfo]> {
        let mut floors = vec![
            FloorInfo { position: Vec3::new(-450.0, 350.0, 1.0), size: Vec2::new(300.0, 1000.0), breakable_wall: None, floor_asset: FloorAssetType::Forest },
            FloorInfo { position: Vec3::new(-450.0, 1400.0, 1.0), size: Vec2::new(300.0, 700.0), breakable_wall: None, floor_asset: FloorAssetType::Forest },
            FloorInfo { position: Vec3::new(-300.0, 1600.0, 1.0), size: Vec2::new(600.0, 300.0), breakable_wall: None, floor_asset: FloorAssetType::Forest },
            FloorInfo { position: Vec3::new(850.0, 1600.0, 1.0), size: Vec2::new(2900.0, 300.0), breakable_wall: None, floor_asset: FloorAssetType::Forest },
            FloorInfo { position: Vec3::new(-300.0, -350.0, 1.0), size: Vec2::new(600.0, 400.0), breakable_wall: None, floor_asset: FloorAssetType::Forest },
            FloorInfo { position: Vec3::new(1300.0, -350.0, 1.0), size: Vec2::new(2000.0, 400.0), breakable_wall: None, floor_asset: FloorAssetType::Forest },
            FloorInfo { position: Vec3::new(2900.0, -350.0, 1.0), size: Vec2::new(600.0, 400.0), breakable_wall: None, floor_asset: FloorAssetType::Forest },
            FloorInfo { position: Vec3::new(2900.0, 1600.0, 1.0), size: Vec2::new(600.0, 300.0), breakable_wall: None, floor_asset: FloorAssetType::Forest },
            FloorInfo { position: Vec3::new(3350.0, -150.0, 1.0), size: Vec2::new(300.0, 500.0), breakable_wall: None, floor_asset: FloorAssetType::Forest },
            FloorInfo { position: Vec3::new(3350.0, 1200.0, 1.0), size: Vec2::new(300.0, 1500.0), breakable_wall: None, floor_asset: FloorAssetType::Forest },
            FloorInfo { position: Vec3::new(2800.0, 350.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Forest },
            FloorInfo { position: Vec3::new(2500.0, 650.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Forest },
            FloorInfo { position: Vec3::new(2800.0, 950.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Forest },
            FloorInfo { position: Vec3::new(2500.0, 1250.0, 1.0), size: Vec2::new(150.0, 100.0), breakable_wall: None, floor_asset: FloorAssetType::Forest },
        ];

        if cweampuff.progression < Progression::MetMilk {
            floors.push(FloorInfo { position: Vec3::new(2450.0, -350.0, 2.0), size: Vec2::new(300.0, 300.0), breakable_wall: None, floor_asset: FloorAssetType::Forest });
        }

        if cweampuff.progression < Progression::HasCherish {
            floors.push(FloorInfo { position: Vec3::new(150.0, -350.0, 2.0), size: Vec2::new(300.0, 300.0), breakable_wall: None, floor_asset: FloorAssetType::Forest });
        }

        if cweampuff.progression < Progression::MilkWokeUp {
            floors.push(FloorInfo { position: Vec3::new(2450.0, 1600.0, 2.0), size: Vec2::new(300.0, 300.0), breakable_wall: None, floor_asset: FloorAssetType::Forest });
        }

        floors.into_boxed_slice()
    }

    fn get_transitions_info(&self, cweampuff: &crate::Cweampuff) -> Option<Box<[TransitionCollider]>> {
        let mut transitions = vec![
            TransitionCollider { exit_index: 0, safe_position: Vec3::new(-350.0, 870.0, CWEAMPUFF_Z_INDEX), transition_to_level: Level::StartingRoom(StartingRoomInfo), floor_info: EntityInfo { position: Vec3::new(-500.0, 950.0, 2.0), size: Vec2::new(100.0, 200.0) }  },
            TransitionCollider { exit_index: 4, safe_position: Vec3::new(3250.0, 150.0, CWEAMPUFF_Z_INDEX), transition_to_level: Level::FactoryTransition(FactoryTransitionInfo), floor_info: EntityInfo { position: Vec3::new(3450.0, 275.0, 1.0), size: Vec2::new(100.0, 350.0) }  },
        ];

        if cweampuff.progression >= Progression::MetMilk {
            transitions.push(
                TransitionCollider { exit_index: 1, safe_position: Vec3::new(2200.0, -50.0, CWEAMPUFF_Z_INDEX), transition_to_level: Level::Hell1(Hell1Info), floor_info: EntityInfo { position: Vec3::new(2450.0, -400.0, 2.0), size: Vec2::new(300.0, 100.0) }  },
            );
        }

        if cweampuff.progression >= Progression::HasCherish {
            transitions.push(
                TransitionCollider { exit_index: 2, safe_position: Vec3::new(350.0, -50.0, CWEAMPUFF_Z_INDEX), transition_to_level: Level::CerberLair(CerberLairInfo), floor_info: EntityInfo { position: Vec3::new(150.0, -400.0, 2.0), size: Vec2::new(300.0, 100.0) }  },
            );
        }

        if cweampuff.progression >= Progression::MilkWokeUp {
            transitions.push(
                TransitionCollider { exit_index: 3, safe_position: Vec3::new(2450.0, 1550.0, CWEAMPUFF_Z_INDEX), transition_to_level: Level::Spaceship1(Spaceship1Info), floor_info: EntityInfo { position: Vec3::new(2450.0, 1700.0, 2.0), size: Vec2::new(300.0, 100.0) }  },
            );
        }

        Some(transitions.into_boxed_slice())
    }

    fn get_doors(&self, _cweampuff: &crate::Cweampuff) -> Option<Box<[DoorCollider]>> {
        Some(Box::from([
            DoorCollider { floor_info: EntityInfo { position: Vec3 { x: 1350., y: -50., z: 0.0 }, size: Vec2 { x: 100., y: 200. } }, door_type: DoorType::Door,
                transition_to_level: Level::CweamcatHouse(CweamcatHouseInfo), safe_position: Vec3 { x: -750.0, y: -375.0, z: CWEAMPUFF_Z_INDEX }, is_active: false }
        ]))
    }
    
    fn get_npcs(&self, cweampuff: &crate::Cweampuff) -> Option<Box<[NPC]>> {
        let mut npcs = vec![];
        
        let mut og_cweampuff = NPC { floor_info: EntityInfo { position: Vec3::new(750.0, -100.0, 0.0), size: Vec2::new(200.0, 100.0) }, is_active: false, current_conversation_index: 0,
                                      conversation: &[], after_conversation_func: |_cweampuff, _commands, _breakable_walls, _cutscene| { }
        };

        let mut minawan = NPC { floor_info: EntityInfo { position: Vec3::new(1750.0, -100.0, 0.0), size: Vec2::new(200.0, 100.0) }, is_active: false, current_conversation_index: 0,
                                        conversation: &[], after_conversation_func: |_cweampuff, _commands, _breakable_walls, _cutscene| { }
        };

        let mut cool_cweampuff = NPC { floor_info: EntityInfo { position: Vec3::new(2050.0, -100.0, 0.0), size: Vec2::new(200.0, 100.0) }, is_active: false, current_conversation_index: 0,
                                      conversation: &[], after_conversation_func: |_cweampuff, _commands, _breakable_walls, _cutscene| { }
        };

        let mut crew_member = NPC { floor_info: EntityInfo { position: Vec3::new(1750.0, -100.0, 0.0), size: Vec2::new(200.0, 100.0) }, is_active: false, current_conversation_index: 0,
                                        conversation: &[], after_conversation_func: |_cweampuff, _commands, _breakable_walls, _cutscene| { }
        };

        let mut masked_cweampuff = NPC { floor_info: EntityInfo { position: Vec3::new(2050.0, -100.0, 0.0), size: Vec2::new(200.0, 100.0) }, is_active: false, current_conversation_index: 0,
                                      conversation: &[], after_conversation_func: |_cweampuff, _commands, _breakable_walls, _cutscene| { }
        };

        let mut drone = NPC { floor_info: EntityInfo { position: Vec3::new(2800.0, 450.0, 0.0), size: Vec2::new(200.0, 100.0) }, is_active: false, current_conversation_index: 0,
                                    conversation: &[], after_conversation_func: |_cweampuff, _commands, _breakable_walls, _cutscene| { }
        };

        let mut rich_cweampuff = NPC { floor_info: EntityInfo { position: Vec3::new(1050.0, -100.0, 0.0), size: Vec2::new(200.0, 100.0) }, is_active: false, current_conversation_index: 0,
                                            conversation: &[], after_conversation_func: |_cweampuff, _commands, _breakable_walls, _cutscene| { }
        };

        match cweampuff.progression {
            Progression::None => {
                og_cweampuff.conversation = &[
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Hello?..", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "Oh! Goodness!", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "You startled me!", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Sorry..", emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "We don't usually get a lot of visitors here...", emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "Hello! Welcome!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "How can this old Cweampuff help you?", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "I think I'm lost...\nI came in these lands looking for the hidden gem.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "But I fell down, and don't know where I am!", emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "Ah, so you've heard the legend.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "O-ho-ho!\nYou're just like me back in the days.", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "I too, used to travel these lands looking for my hidden gem.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Did you find it?", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "I did! It seems you're about to meet yours too!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "Step inside the house; your hidden gem is waiting for you.", emotion: Emotion::Regular },
                ];
            },
            Progression::MetMilk => {
                og_cweampuff.conversation = &[
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Old Cweampuff! Old Cweampuff!", emotion: Emotion::Surprised },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "I've found her!", emotion: Emotion::Surprised },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "O-ho-ho!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "Isn't she the most precious thing in the world?", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "She is!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "The house looks incredible too!", emotion: Emotion::Surprised },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "It feels like...", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Home!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "O-ho-ho!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "Yes, indeed. She built it all on her own!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "On her own?!", emotion: Emotion::Surprised },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "It must've taken a lot of her strength.", emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "I can't wait to talk to her when she wakes up!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "Oh... Well, you see...", emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "She's been asleep for quite some time now...", emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "And I don't know how to wake her up...", emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "What? Oh no...", emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "I wish I could do something...", emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "But I can't leave her alone.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "Someone has to look after her.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "I want to help!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "But what can I do?", emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "I wish I knew...", emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "You'd probably need to go through Hell and back to wake her up.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Through Hell and back...", emotion: Emotion::Regular },
                ];
            },
            Progression::HasCherish => {
                og_cweampuff.conversation = &[
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Old Cweampuff! Old Cweampuff!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "Ah! Cweampuff!", emotion: Emotion::Surprised },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "I'm so glad you came back!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "You were away for quite some time. You made this old Cweampuff worried!", emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "I started to think I'd never see you again.", emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Sorry...", emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "But I brought great news!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "I met minawan while I was in Hell, and they told me their story.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "I think I know now how to wake our hidden gem up!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "What???", emotion: Emotion::Surprised },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "Oh, this old heart of mine can't take such wonderful news!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "You see, what I've learnt is that minawan and their rising star were always together.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "They supported each other through thick and thin.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "So I think...", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "As much as we need our hidden gem...", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "She needs us more!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "And we must show her just that!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "Is that so?..", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "...", emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Old Cweampuff?", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "You might be right.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "Before she fell asleep...", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "She hadn't been her usual self.", emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "She looked... Sad.", emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "Maybe she'd been thinking about...", emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "It's my fault.", emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "What? No!", emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "If only I showed how much she means to me more often.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "Maybe I could've prevented this...", emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "Back then, there were more of us.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "Cweampuffs around the world would come and go.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "I thought our family would grow bigger.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "But for each new cweampuff, old one would leave.", emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "It hurt to see, and in my own sadness I would forget to talk to her.", emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "I didn't even realize that it hurt her even more.", emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "Did she...", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "Did she think we didn't need her anymore?", emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "No! Old Cweampuff! That can't be!", emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "I'm sure you did your best!", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "My best wasn't good enough...", emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "Cweampuff, we need to go see her.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "There is something I need to tell her.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "This time I will make sure she'll never doubt me.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "I'll show her how much I love her!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "*** You have sparked a newfound resolve in Old Cweampuff ***", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "*** Old Cweampuff has learnt to cherish ***", emotion: Emotion::Happy },
                ];

                minawan.conversation = &[
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Minawan?! How come you're here?", emotion: Emotion::Surprised },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MINAWAN, text: "Wan! Wan!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MINAWAN, text: "After I heard your praises of your hidden gem I just had to come see her!", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MINAWAN, text: "I'm glad I did!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MINAWAN, text: "She truly is the most precious cat I've seen!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "That makes me so happy!", emotion: Emotion::Surprised },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Are you going to stay here for a bit?", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MINAWAN, text: "Wan is on a mission, so I'll have to go soon.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MINAWAN, text: "But be sure I'll come back!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "I'll be waiting for you!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MINAWAN, text: "Oh, I even found Cool Cweampuff while I was traveling.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MINAWAN, text: "Make sure to say 'Hi' to him!", emotion: Emotion::Happy },
                ];

                npcs.push(minawan);

                cool_cweampuff.conversation = &[
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Hello!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: COOL_CWEAMPUFF, text: "Sup.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: COOL_CWEAMPUFF, text: "Minawan told me there was a hidden gem here, so I wanted to check her out.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: COOL_CWEAMPUFF, text: "She seems pretty chill.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Isn't she such a cutie?", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: COOL_CWEAMPUFF, text: "Yeah, you could say so.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: COOL_CWEAMPUFF, text: "Her house had records of her journeys.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: COOL_CWEAMPUFF, text: "I've already read it about 57 times while I was hanging out here.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: COOL_CWEAMPUFF, text: "Probably gonna read them a few more dozens times.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Wow, you are so cool!", emotion: Emotion::Surprised },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: COOL_CWEAMPUFF, text: "Yeah, I guess.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: COOL_CWEAMPUFF, text: "Shame she is sleeping, though.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Oh! We are about to wake her up!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Do you want to come with us?", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: COOL_CWEAMPUFF, text: "Wait, what?", emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: COOL_CWEAMPUFF, text: "Oh- um- oh god-", emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: COOL_CWEAMPUFF, text: "-what if she thinks my glasses look stupid -did I fix my fur -oh god -oh no-", emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "What?", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: COOL_CWEAMPUFF, text: "*cough* *cough* I mean... I-I think I'll just lurk around for a bit more.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: COOL_CWEAMPUFF, text: "You guys go ahead, I'll be right behind you.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Okay!", emotion: Emotion::Happy },
                ];
            },
            Progression::MilkWokeUp => {
                og_cweampuff.conversation = &[
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "You did a great job, Cweampuff! Thank you!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "I'm glad I could help! We'll be looking out for her together now!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "Oh-ho-ho!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Speaking of it, when I told Minawan about our hidden gem they mentioned some kind of curse.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "A curse?", emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "I've heard about the curse that can inflict hidden gems, but I've always thought they were just rumors.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Maybe the curse is why she's fallen asleep?", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "And if so, what if...", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "What if there is a possibility she'll fall asleep again?..", emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "Oh no... We can't let that happen...", emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "But we've woken up her once, I'm sure we can do it again!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "I'm afraid it isn't that simple.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "If I recall correctly, it is said the curse grows stronger each day.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "We'll have to find a way to cure the curse completely.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "But how can we do it?", emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Maybe...", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Maybe if there were more of us, each one could've shown her their appreciation.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Do you think it could work?", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "Brilliant, Cweampuff!", emotion: Emotion::Surprised },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "If the three of us were enough to wake her up, then having more cweampuffs would definitely cure the curse!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "But how can we find more cweampuffs?", emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "We'd need something that would catch everyone's attention.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "What could it be?...", emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "Only an otherworldly idea could do such a thing.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Otherworldly...", emotion: Emotion::Regular },
                ];

                cool_cweampuff.conversation = &[
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Cool Cweampuff! She woke up!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: COOL_CWEAMPUFF, text: "What?", emotion: Emotion::Surprised },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "You should go say 'Hi' to her! She'll be happy to see you!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: COOL_CWEAMPUFF, text: "Oh- um-... I-I yeah I'll um-", emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: COOL_CWEAMPUFF, text: "*Cough* *cough* I mean, I already said 'Hi' to her once.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: COOL_CWEAMPUFF, text: "I don't really like repeating myself.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: COOL_CWEAMPUFF, text: "I kinda prefer just looking out for her from a distance.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: COOL_CWEAMPUFF, text: "Don't like to put myself out there, you know?", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Wow, you are so cool!", emotion: Emotion::Surprised },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "I wish I could be as cool as you one day!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: COOL_CWEAMPUFF, text: "I bet you can if you put your mind to it!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: COOL_CWEAMPUFF, text: "By the way, some sort of a spaceship landed just above us recently.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: COOL_CWEAMPUFF, text: "But don't worry, I'll protect you all if anything comes around.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: COOL_CWEAMPUFF, text: "For now, I think I'll just go read the records a few more times.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: COOL_CWEAMPUFF, text: "Okay! See you!", emotion: Emotion::Happy },
                ];
            },
            Progression::HasLetter => {
                og_cweampuff.conversation = &[
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Old Cweampuff! Old Cweampuff!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "Cweampuff! You're back!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "'Back', huh...", emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Anyways, the crew of the spaceship and their precious captain gave me something!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "They said it can help bring more cweampuffs!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "O-ho-ho, that's just wonderful!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Let's go give this letter in a bottle to our hidden gem!", emotion: Emotion::Happy },
                ];

                crew_member.conversation = &[
                    ConversationEntry { position: ConversationPosition::Right, npc_name: CREW_MEMBER, text: "Ah, Cweampuff! Hello!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Hello! Happy to see you here!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: CREW_MEMBER, text: "After you helped us it's the least I can do.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: CREW_MEMBER, text: "I could feel your burning passion to help your hidden gem.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: CREW_MEMBER, text: "So I had to come and see her for myself.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: CREW_MEMBER, text: "What a gem you've found!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "She's the best thing I've ever seen!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: CREW_MEMBER, text: "She even reminds me of my rising star a little.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: CREW_MEMBER, text: "I wonder if they would get along.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: CREW_MEMBER, text: "Oh, one more thing.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: CREW_MEMBER, text: "We've found Masked Cweampuff on our ship.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: CREW_MEMBER, text: "We mistook 'em for the intruder, but he turned out to be a really nice guy.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: CREW_MEMBER, text: "He even gave a little present to our captain!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: CREW_MEMBER, text: "Go say 'Hi' to him if you haven't already. I'll be heading out soon", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Alright! Was really nice seeing you!", emotion: Emotion::Happy },
                ];

                npcs.push(crew_member);

                masked_cweampuff.conversation = &[
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Hello?..", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MASKED_CWEAMPUFF, text: "Hello, Cweampuff.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Cool masks!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MASKED_CWEAMPUFF, text: "Oh, thank you! I like them too!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MASKED_CWEAMPUFF, text: "Although, sometimes they get me in trouble.", emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MASKED_CWEAMPUFF, text: "Like on that ship just recently...", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Really? Why do you wear them?", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MASKED_CWEAMPUFF, text: "Well, the answer is a bit complicated...", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MASKED_CWEAMPUFF, text: "Simply put it, they allow me to be anyone.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MASKED_CWEAMPUFF, text: "I like making others happy!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MASKED_CWEAMPUFF, text: "No one knows who really is behind this mask.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MASKED_CWEAMPUFF, text: "When I give a present to someone, they don't know who gave it.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MASKED_CWEAMPUFF, text: "It could've been you, or Old Cweampuff, or someone else.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MASKED_CWEAMPUFF, text: "That way, everyone gets a little bit a appreciation, and not just me.", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "That's so kind of you!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MASKED_CWEAMPUFF, text: "Thank you!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MASKED_CWEAMPUFF, text: "I'm going to go prepare something for our hidden gem now. Hope we get to talk again!", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "See you!", emotion: Emotion::Happy },
                ];
            },
            Progression::GivenLetter => {
                og_cweampuff.conversation = &[
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "What was that?", emotion: Emotion::Surprised },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "That might have been exactly what we were looking for...", emotion: Emotion::Surprised },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "It was THEIR overlord...", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Who are 'they'?", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "Everyone in these lands know them...", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "A force so strong a mere name of their overlord can summon them...", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "They are known as...", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "The Swarm.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "With their help there's going to be enough of us to completely stop the curse.", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "We just need to let them know about what just happened...", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "Didn't Cool Cweampuff said he wanted to draw this event?", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "If a mere word is enough, then having a picture will surely bring them here!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "The captain was right, it's like a beacon for them!", emotion: Emotion::Surprised },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "With their hive mind, the word of our hidden gem is going to spread like never before!", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "Our family will surely grow!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "And the curse will be no more.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "Go to the factory Cweampuff, that's where you'll find them.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "I will!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "We will cure our hidden gem!", emotion: Emotion::Happy },
                ];

                cool_cweampuff.conversation = &[
                    ConversationEntry { position: ConversationPosition::Right, npc_name: COOL_CWEAMPUFF, text: "Cweampuff, you're right on time.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: COOL_CWEAMPUFF, text: "I just finished drawing the picture.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Already?", emotion: Emotion::Surprised },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "I didn't know you were good at drawing!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: COOL_CWEAMPUFF, text: "Well, I don't like to brag about it.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Wow, you're so cool!", emotion: Emotion::Surprised },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: COOL_CWEAMPUFF, text: "Here, take it.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: COOL_CWEAMPUFF, text: "*** You received a picture of your hidden gem and a robot girl ***", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "It's so good! Thank you, Cool Cweampuff!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "By the way, are you feeling better now?", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: COOL_CWEAMPUFF, text: "A little bit, yeah...", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: COOL_CWEAMPUFF, text: "You see, something's been weighing on my mind.", emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: COOL_CWEAMPUFF, text: "I don't really talk to our hidden gem...", emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: COOL_CWEAMPUFF, text: "It's not like I don't like her, quite the opposite.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: COOL_CWEAMPUFF, text: "But I just enjoy watching her from the sidelines.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: COOL_CWEAMPUFF, text: "Just lurking, you know?", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: COOL_CWEAMPUFF, text: "So I've been thinking...", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: COOL_CWEAMPUFF, text: "What if it isn't something she wants from me?", emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: COOL_CWEAMPUFF, text: "What if she doesn't want me around?", emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: COOL_CWEAMPUFF, text: "What if I leave and she doesn't even miss me?", emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "What?", emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Our hidden gem would never think any of that!", emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "I bet she likes you just as much as any of us!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Why don't you ask her about your worries?", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "I'm sure you'll feel a lot better after talking to her!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: COOL_CWEAMPUFF, text: "...", emotion: Emotion::Sad },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: COOL_CWEAMPUFF, text: "You're right, Cweampuff.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: COOL_CWEAMPUFF, text: "At least this one time I have to be brave. I'll need some time to prepare, though.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: COOL_CWEAMPUFF, text: "Thank you, Cweampuff!", emotion: Emotion::Happy },
                ];

                cool_cweampuff.floor_info.position.x = 1750.;

                masked_cweampuff.conversation = &[
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MASKED_CWEAMPUFF, text: "...", emotion: Emotion::Surprised },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MASKED_CWEAMPUFF, text: "I've been to a lot of places in these lands...", emotion: Emotion::Surprised },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MASKED_CWEAMPUFF, text: "But I've never seen anything like this before...", emotion: Emotion::Surprised },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MASKED_CWEAMPUFF, text: "I'll prepare a worthy present for our hidden gem!", emotion: Emotion::Happy },
                ];
            },
            Progression::RisingStar => {
                og_cweampuff.conversation = &[
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Old Cweampuff! Old Cweampuff!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "There you are Cweampuff!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "Look how many of us are here today!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "They all came to see our hidden gem!", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "My old heart can't take this! I'm so happy!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "Our family's grown so much!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "I hope they all enjoy their stay here!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "We don't have to worry about the curse now too.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "I know how to cure it!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "What?! How?", emotion: Emotion::Surprised },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "We need to make her a rising star!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "Rising star?", emotion: Emotion::Surprised },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "But of course!", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "Now that there's enough of us we can surely make it happen!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "I can't wait to see her shine!", emotion: Emotion::Surprised },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: OG_CWEAMPUFF, text: "Let's go inside the house, Cweampuff. Our rising star is waiting for us.", emotion: Emotion::Regular },
                ];

                cool_cweampuff.conversation = &[
                    ConversationEntry { position: ConversationPosition::Right, npc_name: COOL_CWEAMPUFF, text: "Yo, Cweampuff.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Hello!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: COOL_CWEAMPUFF, text: "You were right about everything.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: COOL_CWEAMPUFF, text: "What a fool I was for even thinking all of that.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: COOL_CWEAMPUFF, text: "I had a great talk with our hidden gem.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: COOL_CWEAMPUFF, text: "She let me know how much she appreciates me!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: COOL_CWEAMPUFF, text: "I think I'll dedicate myself to drawing art of her while I'm lurking around.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: COOL_CWEAMPUFF, text: "Have to put my skills to a good use.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Wow, you're are so cool!", emotion: Emotion::Surprised },
                ];

                cool_cweampuff.floor_info.position.x = 1750.;

                masked_cweampuff.conversation = &[
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Hello, Masked Cweampuff!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MASKED_CWEAMPUFF, text: "Hello, Cweampuff!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MASKED_CWEAMPUFF, text: "Do you remember how I wanted to prepare our hidden gem a present?", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MASKED_CWEAMPUFF, text: "After the crew had given her a walkie-talkie I heard her talk with their captain.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MASKED_CWEAMPUFF, text: "I'm pretty sure I've heard some train whistles.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MASKED_CWEAMPUFF, text: "So I thought she likes trains and left her a little plushy train!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "That's such a cute present!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "I'm sure she loves it!", emotion: Emotion::Happy },
                ];

                crew_member.conversation = &[
                    ConversationEntry { position: ConversationPosition::Right, npc_name: CREW_MEMBER, text: "Hello, Cweampuff!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Hello!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: CREW_MEMBER, text: "Our captain asked me to deliver a walkie-talkie to your hidden gem.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: CREW_MEMBER, text: "Now they can talk to each other whenever they want!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "That's great news!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "I'm so happy they're friends now!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: CREW_MEMBER, text: "Same here!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: CREW_MEMBER, text: "I can't wait to see them go on an adventure together!", emotion: Emotion::Happy },
                ];

                crew_member.floor_info.position.x = 2500.;
                crew_member.floor_info.position.y = 750.;

                npcs.push(crew_member);

                minawan.conversation = &[
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MINAWAN, text: "Wan! Wan!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Minawan! I'm so happy to see you here! Wan, wan!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MINAWAN, text: "I've completed my mission, and really wanted to see your hidden gem once more.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MINAWAN, text: "Thanks to you, the word of her's been spreading quite fast around these lands.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MINAWAN, text: "This place seems so lively now!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MINAWAN, text: "It's just like with my rising star... Speaking of...", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MINAWAN, text: "I'd love to see your hidden gem and my rising star together someday!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MINAWAN, text: "I think they could be very good friends!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "I'd love that! Wan, wan!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: MINAWAN, text: "Wan! Wan!", emotion: Emotion::Happy },
                ];

                minawan.floor_info.position.x = 2900.;

                npcs.push(minawan);

                drone.conversation = &[
                    ConversationEntry { position: ConversationPosition::Right, npc_name: DRONE, text: "I LOVE OUR HIDDEN GEM!!!!", emotion: Emotion::Surprised },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "'Our'?..", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: DRONE, text: "YES!!! I AM NOW A CWEAMPUFF TOO!!!!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: DRONE, text: "SHE IS THE BEST THING IN THE WORLD!!!!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: DRONE, text: "SHE WILL BE ASSIMIL... Wait...", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: DRONE, text: "Maybe it's not her who's being assimilated...", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: DRONE, text: "Maybe it's ME who got assimilated...", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Oh! We'll be happy to make you a part of our family!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: DRONE, text: "THANK YOU!!!!", emotion: Emotion::Happy },
                ];

                npcs.push(drone);

                rich_cweampuff.conversation = &[
                    ConversationEntry { position: ConversationPosition::Right, npc_name: RICH_CWEAMPUFF, text: "Yes, this'll be quite the investment", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Hello! What kind of investment are you talking about?", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: RICH_CWEAMPUFF, text: "Oh, hello Cweampuff!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: RICH_CWEAMPUFF, text: "Drone told me about your hidden gem, so I wanted to see her for myself.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: RICH_CWEAMPUFF, text: "Now that I'm here, I'm confident, she's worth the investment!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: RICH_CWEAMPUFF, text: "She got so flustered after I'd bought her a yacht!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: RICH_CWEAMPUFF, text: "Maybe I should double my investment...", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Wow! That's so much money!", emotion: Emotion::Surprised },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "You're calling it 'investment', are you planning to get something in return?", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: RICH_CWEAMPUFF, text: "Well, naturally.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: RICH_CWEAMPUFF, text: "In return, I want to see her happy!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: RICH_CWEAMPUFF, text: "I'm confident, if I do it for her then nothing's a waste.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: RICH_CWEAMPUFF, text: "If my money can make her just a bit happier, then that's more than enough for me!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: RICH_CWEAMPUFF, text: "They say money can't buy happiness.", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: RICH_CWEAMPUFF, text: "But that's the case only if you spend them on yourself!", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: RICH_CWEAMPUFF, text: "If you spend money to make someone else's life better...", emotion: Emotion::Regular },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: RICH_CWEAMPUFF, text: "Then you bet it's going to make you feel happy!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Left, npc_name: CWEAMPUFF, text: "Yeah! Our hidden gem deserves our support!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: RICH_CWEAMPUFF, text: "Indeed!", emotion: Emotion::Happy },
                    ConversationEntry { position: ConversationPosition::Right, npc_name: RICH_CWEAMPUFF, text: "On the second thought, maybe I should triple my investment...", emotion: Emotion::Regular },
                ];
            }
        };

        npcs.push(og_cweampuff);

        if cweampuff.progression >= Progression::HasCherish && cweampuff.progression != Progression::HasLetter {
            npcs.push(cool_cweampuff);
        }

        if cweampuff.progression >= Progression::HasLetter {
            npcs.push(masked_cweampuff);
        }

        if cweampuff.progression >= Progression::RisingStar {
            npcs.push(rich_cweampuff);
        }

        Some(npcs.into_boxed_slice())
    }

    fn get_floor_modifications(&self, _cweampuff: &crate::Cweampuff) -> Option<Box<[FloorModification]>> {
        None
    }

    fn get_bgm(&self) -> Option<&'static str> {
        Some("forest")
    }
}