use bevy::{audio::{PlaybackMode, Volume}, color::{palettes::css::RED, Color}, prelude::*};

use crate::{app_states::AppState, cutscene::{CutsceneEvent, CutsceneInfo, PostCutsceneAction}, fade_in_fade_out::FadeInFadeOutNode, level::{level_bgm::{LevelBGM, LevelBGMState}, level_layout::starting_room_layout::StartingRoomInfo, Level}};

pub const DEFAULT_FONT: &str = "fonts/Shadows Into Light.ttf";

pub const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

#[derive(Component)]
pub struct MainMenuComponent;

#[derive(Component)]
pub enum ButtonAction {
    StartGame,
    Settings,
    Quit
}

pub fn button_visuals_handler(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor
        ),
        (Changed<Interaction>, With<Button>),
    >
) {
    for (interaction, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                border_color.0 = RED.into();
                
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

pub fn main_menu_button_interactions_handler(
    mut interaction_query: Query<(&Interaction, &ButtonAction), (Changed<Interaction>, With<Button>)>,
    mut exit: EventWriter<AppExit>,
    mut app_state: ResMut<NextState<AppState>>,
    mut cutscene: EventWriter<CutsceneEvent>
) {
    for (interaction, action) in &mut interaction_query {
        if let Interaction::Pressed = *interaction  {
            match action {
                ButtonAction::StartGame => {
                    cutscene.send(CutsceneEvent::Started(&[
                        CutsceneInfo { text: "The legend has it...", background: "" },
                        CutsceneInfo { text: "Somewhere in these lands, there is a hidden gem.", background: ""  },
                        CutsceneInfo { text: "Capable of giving eternal happiness to those who find it.", background: "" },
                        CutsceneInfo { text: "A brave cweampuff decided to look for it...", background: "" },
                        CutsceneInfo { text: "", background: "cutscenes/opening/1.png" },
                        CutsceneInfo { text: "", background: "cutscenes/opening/2.png" },
                    ], "ost/factory.mp3", PostCutsceneAction::TransitionTo(Level::StartingRoom(StartingRoomInfo))));
                },
                ButtonAction::Settings => {
                    app_state.set(AppState::AudioMenu);
                }
                ButtonAction::Quit => {
                    exit.send(AppExit::Success);
                }
            };
        }
    }
}

pub fn spawn_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut next_bgm_state: ResMut<NextState<LevelBGMState>>,
    bgm_query: Query<Entity, With<LevelBGM>>,
) {    
    for entity in bgm_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    
    commands
        .spawn((Node {
            width: Val::Percent(10.0),
            height: Val::Percent(5.0),
            top: Val::Percent(70.),
            left: Val::Percent(45.),
            justify_content: JustifyContent::Center,
            ..default()
        }, MainMenuComponent))
        .with_children(|parent| {
            parent
                .spawn((
                    Button,
                    ButtonAction::StartGame,
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        border: UiRect::all(Val::Percent(2.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BorderColor(Color::BLACK),
                    BorderRadius::MAX,
                    BackgroundColor(NORMAL_BUTTON),
                ))
                .with_child((
                    Text::new("Start"),
                    TextFont {
                        font: asset_server.load(DEFAULT_FONT),
                        font_size: 33.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                ));
        });

    commands
        .spawn((Node {
            width: Val::Percent(10.0),
            height: Val::Percent(5.0),
            top: Val::Percent(80.),
            left: Val::Percent(45.),
            justify_content: JustifyContent::Center,
            ..default()
        }, MainMenuComponent))
        .with_children(|parent| {
            parent
                .spawn((
                    Button,
                    ButtonAction::Settings,
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        border: UiRect::all(Val::Percent(2.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BorderColor(Color::BLACK),
                    BorderRadius::MAX,
                    BackgroundColor(NORMAL_BUTTON),
                ))
                .with_child((
                    Text::new("Settings"),
                    TextFont {
                        font: asset_server.load(DEFAULT_FONT),
                        font_size: 33.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                ));
        });

    commands
        .spawn((Node {
            width: Val::Percent(10.0),
            height: Val::Percent(5.0),
            top: Val::Percent(90.0),
            left: Val::Percent(45.),
            justify_content: JustifyContent::Center,
            ..default()
        }, MainMenuComponent))
        .with_children(|parent| {
            parent
                .spawn((
                    Button,
                    ButtonAction::Quit,
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        border: UiRect::all(Val::Percent(2.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BorderColor(Color::BLACK),
                    BorderRadius::MAX,
                    BackgroundColor(NORMAL_BUTTON),
                ))
                .with_child((
                    Text::new("Quit"),
                    TextFont {
                        font: asset_server.load(DEFAULT_FONT),
                        font_size: 33.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                ));
        });

    let mut playback_settings = PlaybackSettings::default().with_volume(Volume::new(0.));
    playback_settings.mode = PlaybackMode::Loop;

    commands.spawn((
        AudioPlayer::new(asset_server.load("ost/hell.mp3")),
        LevelBGM,
        playback_settings
    ));

    next_bgm_state.set(LevelBGMState::Changing);
}

pub fn despawn_main_menu(
    mut commands: Commands, 
    query: Query<Entity, (With<Node>, With<MainMenuComponent>, Without<Camera2d>, Without<FadeInFadeOutNode>)>
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}