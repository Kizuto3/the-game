use bevy::math::{Vec2, Vec3};

use crate::{level::{level_layout::{cweamcat_house_layout::CweamcatHouseInfo, hell_1_layout::Hell1Info, starting_room_layout::StartingRoomInfo}, progression::Progression, Level}, npc::{conversation_entry::{ConversationEntry, ConversationPosition, Emotion}, COOL_CWEAMPUFF, CWEAMPUFF, MINAWAN, NPC, OG_CWEAMPUFF}, CWEAMPUFF_Z_INDEX};

use super::{cerber_lair_layout::CerberLairInfo, spaceship_1_layout::Spaceship1Info, DoorCollider, EntityInfo, FloorAssetType, FloorInfo, FloorModification, LevelInfo, TransitionCollider};

#[derive(Clone, Copy)]
pub struct CweamcatLairInfo;

impl LevelInfo for CweamcatLairInfo {
    fn get_floor_info(&self, cweampuff: &crate::Cweampuff) -> Box<[FloorInfo]> {
        let mut floors = vec![
            FloorInfo { position: Vec3::new(-450.0, 350.0, 1.0), size: Vec2::new(300.0, 1000.0), breakable_wall: None, floor_asset: FloorAssetType::Forest },
            FloorInfo { position: Vec3::new(-450.0, 1250.0, 1.0), size: Vec2::new(300.0, 400.0), breakable_wall: None, floor_asset: FloorAssetType::Forest },
            FloorInfo { position: Vec3::new(-300.0, 1600.0, 1.0), size: Vec2::new(600.0, 300.0), breakable_wall: None, floor_asset: FloorAssetType::Forest },
            FloorInfo { position: Vec3::new(1300.0, 1600.0, 1.0), size: Vec2::new(2000.0, 300.0), breakable_wall: None, floor_asset: FloorAssetType::Forest },
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
        ];

        if cweampuff.progression >= Progression::MetMilk {
            transitions.push(
                TransitionCollider { exit_index: 1, safe_position: Vec3::new(2200.0, -50.0, CWEAMPUFF_Z_INDEX), transition_to_level: Level::Hell1(Hell1Info), floor_info: EntityInfo { position: Vec3::new(2450.0, -350.0, 2.0), size: Vec2::new(300.0, 100.0) }  },
            );
        }

        if cweampuff.progression >= Progression::HasCherish {
            transitions.push(
                TransitionCollider { exit_index: 2, safe_position: Vec3::new(350.0, -50.0, CWEAMPUFF_Z_INDEX), transition_to_level: Level::CerberLair(CerberLairInfo), floor_info: EntityInfo { position: Vec3::new(150.0, -350.0, 2.0), size: Vec2::new(300.0, 100.0) }  },
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
            DoorCollider { floor_info: EntityInfo { position: Vec3 { x: 1350., y: -50., z: 0.0 }, size: Vec2 { x: 100., y: 200. } },
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
                // TODO
            }
        };

        npcs.push(og_cweampuff);

        if cweampuff.progression >= Progression::HasCherish {
            npcs.push(cool_cweampuff);
        }

        Some(npcs.into_boxed_slice())
    }

    fn get_floor_modifications(&self, _cweampuff: &crate::Cweampuff) -> Option<Box<[FloorModification]>> {
        None
    }
}