use bevy::ecs::event::{Event, EventReader};
use bevy::prelude::*;

use crate::app_states::AppState;
use crate::fade_in_fade_out::FadeState;
use crate::level::level_layout::starting_room_layout::StartingRoomInfo;
use crate::level::progression::Progression;
use crate::level::transition_states::TransitionState;
use crate::level::{manually_transition_to_level, Level, LevelLayout};
use crate::{Cweampuff, CWEAMPUFF_STARTING_POSITION};

#[derive(Event)]
pub enum CutsceneEvent {
    Started(Vec<CutsceneInfo>),
    Stopped
}

#[derive(Clone)]
pub struct CutsceneInfo {
    pub text: &'static str,
    pub background: &'static str,
    pub bgm: &'static str
}

#[derive(Component)]
pub struct Cutscene {
    infos: Vec<CutsceneInfo>,
    current_index: usize
}

#[derive(Component)]
pub struct CutsceneText;

pub fn cutscene_event_reader(
    mut cutscene_events: EventReader<CutsceneEvent>,
    mut state: ResMut<NextState<AppState>>,
    mut commands: Commands,
    current_level_layout: Query<Entity, With<LevelLayout>>,
    mut transition_state: ResMut<NextState<TransitionState>>,
) {
    for cutscene in cutscene_events.read() {
        if let CutsceneEvent::Started(infos) = cutscene {
            state.set(AppState::Cutscene);
            commands.spawn(Cutscene { infos: infos.to_vec(), current_index: 0 });
        }
        if let CutsceneEvent::Stopped = cutscene {
            state.set(AppState::InGame);

            manually_transition_to_level(&current_level_layout, &mut transition_state, &Cweampuff {progression: Progression::None, has_double_jump: false, has_wall_jump: false, has_dash: false}, &mut commands, Level::StartingRoom(StartingRoomInfo), CWEAMPUFF_STARTING_POSITION);
        }
    }
}

pub fn cutscene_input_reader(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_input: Res<ButtonInput<MouseButton>>, 
    current_cutscene: Single<&Cutscene, With<Cutscene>>,
    mut next_fade_state: ResMut<NextState<FadeState>>,
    current_fade_state: Res<State<FadeState>>
) {
    if current_cutscene.current_index == 0 || 
       keyboard_input.any_just_pressed([KeyCode::Space, KeyCode::Enter]) ||
       mouse_input.any_just_pressed([MouseButton::Left, MouseButton::Right]) {
        match current_fade_state.get() {
            FadeState::FadeIn | FadeState::FadeInFinished | FadeState::FadeOut => return,
            FadeState::None => next_fade_state.set(FadeState::FadeIn),
        };
    }
}

pub fn cutscene_player(
    mut cutscene_events: EventWriter<CutsceneEvent>, 
    mut text_query: Query<&mut Text, With<CutsceneText>>,
    mut current_cutscene: Single<&mut Cutscene, With<Cutscene>>,
    mut next_fade_state: ResMut<NextState<FadeState>>
) {    
    {
        let current_cutscene_info = match current_cutscene.infos.get(current_cutscene.current_index) {
            Some(info) => info,
            None => {
                cutscene_events.send(CutsceneEvent::Stopped);
                return;
            }
        };

        for mut text in text_query.iter_mut() {
            **text = current_cutscene_info.text.to_string();
        }
    }
    
    current_cutscene.current_index += 1;

    next_fade_state.set(FadeState::FadeOut);
}

pub fn spawn_cutscene_resources(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
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
                        font: asset_server.load("fonts/Shadows Into Light.ttf"),
                        font_size: 33.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                ));
        });
}