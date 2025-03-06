use bevy::asset::{LoadState, UntypedAssetId};
use bevy::audio::{PlaybackMode, Volume};
use bevy::ecs::event::{Event, EventReader};
use bevy::prelude::*;
use bevy::ui::widget::NodeImageMode;

use crate::app_states::AppState;
use crate::fade_in_fade_out::{FadeInFadeOutNode, FadeState};
use crate::level::level_layout::starting_room_layout::StartingRoomInfo;
use crate::level::progression::Progression;
use crate::level::transition_states::TransitionState;
use crate::level::{manually_transition_to_level, Level, LevelLayout};
use crate::main_menu::DEFAULT_FONT;
use crate::npc::{LeftCharacterImageNode, RightCharacterImageNode};
use crate::{Cweampuff, CWEAMPUFF_STARTING_POSITION};

#[derive(Component)]
pub struct LoadingAssets{
    assets: Vec<UntypedAssetId>
}

#[derive(Event)]
pub enum CutsceneEvent {
    Started(&'static[CutsceneInfo], Level, &'static str),
    Stopped(Cweampuff, Level, Vec3)
}

#[derive(Clone)]
pub struct CutsceneInfo {
    pub text: &'static str,
    pub background: &'static str
}

#[derive(Component)]
pub struct Cutscene {
    infos: &'static[CutsceneInfo],
    bgm: &'static str,
    current_index: usize,
    transition_to_level_after: Level
}

#[derive(Component)]
pub struct CutsceneText;

#[derive(Component)]
pub struct CutsceneAudio;

#[derive(Component)]
pub struct CutsceneBackground;

pub fn cutscene_event_reader(
    mut cutscene_events: EventReader<CutsceneEvent>,
    mut state: ResMut<NextState<AppState>>,
    mut commands: Commands,
    current_level_layout: Query<Entity, With<LevelLayout>>,
    mut transition_state: ResMut<NextState<TransitionState>>,
) {
    for cutscene in cutscene_events.read() {
        if let CutsceneEvent::Started(infos, level, bgm) = cutscene {
            commands.spawn(Cutscene { infos, bgm, current_index: 0, transition_to_level_after: *level});

            state.set(AppState::Cutscene);
        }
        if let CutsceneEvent::Stopped(cweampuff, level, position) = cutscene {
            state.set(AppState::InGame);

            manually_transition_to_level(&current_level_layout, &mut transition_state, cweampuff, &mut commands, *level, *position);
        }
    }
}

pub fn cutscene_input_reader(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_input: Res<ButtonInput<MouseButton>>, 
    current_cutscene: Single<&Cutscene, With<Cutscene>>,
    mut next_fade_state: ResMut<NextState<FadeState>>,
    current_fade_state: Res<State<FadeState>>,

) {
    if current_cutscene.current_index == 0 || 
       keyboard_input.any_just_pressed([KeyCode::Space, KeyCode::Enter]) ||
       mouse_input.any_just_pressed([MouseButton::Left, MouseButton::Right]) {
        match current_fade_state.get() {
            FadeState::FadeIn | FadeState::FadeInFinished | FadeState::FadeOut => (),
            FadeState::None => next_fade_state.set(FadeState::FadeIn),
        };
    }
}

pub fn cutscene_player(
    mut cutscene_events: EventWriter<CutsceneEvent>, 
    mut text_query: Query<&mut Text, With<CutsceneText>>,
    mut background_image: Single<&mut ImageNode, (With<Node>, With<CutsceneBackground>, Without<RightCharacterImageNode>, Without<LeftCharacterImageNode>)>,
    mut current_cutscene: Single<&mut Cutscene, With<Cutscene>>,
    cweampuff_query: Query<(&Cweampuff, &Transform), With<Cweampuff>>,
    asset_server: Res<AssetServer>,
    mut loading_assets: Single<&mut LoadingAssets, With<LoadingAssets>>,
) {    
    {
        let current_cutscene_info = match current_cutscene.infos.get(current_cutscene.current_index) {
            Some(info) => info,
            None => {
                if cweampuff_query.is_empty() {
                    cutscene_events.send(CutsceneEvent::Stopped(
                        Cweampuff {progression: Progression::None, has_double_jump: false, has_wall_jump: false, has_dash: false}, 
                        Level::StartingRoom(StartingRoomInfo),
                        CWEAMPUFF_STARTING_POSITION
                    ));
                    return;
                };
                
                let (cweampuff, transform) = cweampuff_query.single();

                cutscene_events.send(CutsceneEvent::Stopped(*cweampuff, current_cutscene.transition_to_level_after, transform.translation));

                return;
            }
        };

        background_image.image = asset_server.load(current_cutscene_info.background);
        loading_assets.assets.push(background_image.image.id().untyped());

        for mut text in text_query.iter_mut() {
            **text = current_cutscene_info.text.to_string();
        }
    }
    
    current_cutscene.current_index += 1;
}

pub fn wait_for_resources_to_load(
    server: Res<AssetServer>,
    mut loading_assets: Single<&mut LoadingAssets, With<LoadingAssets>>,
    mut next_fade_state: ResMut<NextState<FadeState>>,
) {
    if loading_assets.assets.iter().all(|res| 
        if let Some(state) = server.get_load_state(*res) {
            match state {
                LoadState::Loaded | LoadState::Failed(_) => return true,
                _ => return false
            }
        }
        else {
            return true;
        }
    ) {
        loading_assets.assets.clear();
        next_fade_state.set(FadeState::FadeOut);
    }
}

pub fn spawn_cutscene_resources(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    current_cutscene: Single<&Cutscene, With<Cutscene>>,
) {
    commands
        .spawn((
            ImageNode::default()
            .with_mode(NodeImageMode::Stretch),
            CutsceneBackground,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Default,
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                ..default()
            }
        ))
        .with_children(|parent| {
            parent
                .spawn(
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(15.0),
                        top: Val::Percent(85.0),
                        left: Val::Percent(0.),
                        position_type: PositionType::Absolute,
                        align_items: AlignItems::Default,
                        justify_content: JustifyContent::Center,
                        ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn((
                            Text::new(""),
                            CutsceneText,
                            TextFont {
                                font: asset_server.load(DEFAULT_FONT),
                                font_size: 33.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.9, 0.9, 0.9))
                        ));
                });
    });

    let mut playback_settings = PlaybackSettings::default().with_volume(Volume::new(0.2));
    playback_settings.mode = PlaybackMode::Once;

    commands.spawn((
        AudioPlayer::new(asset_server.load(current_cutscene.bgm)),
        CutsceneAudio,
        playback_settings
    ));

    commands.spawn(LoadingAssets { assets: vec![] });
}

pub fn despawn_cutscene_resources(
    mut commands: Commands,
    nodes: Query<Entity, (With<Node>, Without<Camera2d>, Without<FadeInFadeOutNode>)>,
    cutsnenes: Query<Entity, (With<Cutscene>, Without<Node>)>,
    loading_assets: Query<Entity, (With<LoadingAssets>, Without<Node>)>,
    audio_players: Query<Entity, (With<AudioPlayer>, With<CutsceneAudio>)>
) {
    for entity in nodes.iter() {
        commands.entity(entity).despawn_recursive();
    }

    for entity in cutsnenes.iter() {
        commands.entity(entity).despawn_recursive();
    }

    for entity in audio_players.iter() {
        commands.entity(entity).despawn_recursive();
    }

    for entity in loading_assets.iter() {
        commands.entity(entity).despawn_recursive();
    }
}