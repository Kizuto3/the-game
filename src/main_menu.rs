use bevy::{audio::{PlaybackMode, Volume}, color::Color, prelude::*, ui::widget::NodeImageMode};

use crate::{app_states::AppState, cutscene::{CutsceneEvent, CutsceneInfo, PostCutsceneAction}, fade_in_fade_out::FadeInFadeOutNode, level::{level_bgm::{LevelBGM, LevelBGMState}, Level}};
use crate::asset_loader::load_asset;

pub const DEFAULT_FONT: &str = "fonts/Shadows Into Light.ttf";

pub const NORMAL_BUTTON: Color = Color::srgba(0.15, 0.15, 0.15, 0.75);
const HOVERED_BUTTON: Color = Color::srgba(0.25, 0.25, 0.25, 0.75);
const PRESSED_BUTTON: Color = Color::srgba(0.75, 0.75, 0.75, 0.75);

#[derive(Component)]
pub struct MainMenuComponent;

#[derive(Component)]
pub struct MainMenuAudio;

#[derive(Component)]
pub struct MainBackground;

#[derive(Component)]
pub enum ButtonAction {
    StartGame,
    Settings,
    Credits,
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
                border_color.0 = Color::WHITE;
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
                    cutscene.write(CutsceneEvent::Started(&[
                        CutsceneInfo { text: "The legend has it...", background: "" },
                        CutsceneInfo { text: "Somewhere in these lands, there is a Hidden Gem.", background: ""  },
                        CutsceneInfo { text: "Capable of giving eternal happiness to those who find it.", background: "" },
                        CutsceneInfo { text: "A brave Cweampuff decided to look for it...", background: "" },
                        CutsceneInfo { text: "", background: "cutscenes/opening/1.png" },
                        CutsceneInfo { text: "", background: "cutscenes/opening/2.png" },
                    ], "ost/cutscene.mp3", PostCutsceneAction::TransitionTo(Level::StartingRoom)));
                },
                ButtonAction::Settings => {
                    app_state.set(AppState::AudioMenu);
                },
                ButtonAction::Credits => {
                    app_state.set(AppState::CreditsMenu);
                },
                ButtonAction::Quit => {
                    exit.write(AppExit::Success);
                }
            };
        }
    }
}

pub fn spawn_background_image(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    background_query: Query<Entity, (With<Node>, With<MainBackground>)>,
) {
    if !background_query.is_empty() {
        return;
    }
    
    let background_image_handle = load_asset(&asset_server, "main.png");

    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            position_type: PositionType::Absolute,
            ..default()
        },
        MainBackground,
        ImageNode::new(background_image_handle).with_mode(NodeImageMode::Auto),
    ));
}

pub fn spawn_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut next_bgm_state: ResMut<NextState<LevelBGMState>>,
    bgm_query: Query<Entity, (With<LevelBGM>, Without<MainMenuAudio>)>,
    background: Single<Entity, (With<Node>, With<MainBackground>)>,
    main_menu_bgm_query: Query<&MainMenuAudio>,
) {    
    for entity in bgm_query.iter() {
        commands.entity(entity).despawn();
    }

    commands
        .spawn((Node {
            width: Val::Percent(70.0),
            height: Val::Percent(25.0),
            top: Val::Percent(35.),
            left: Val::Percent(15.),
            position_type: PositionType::Absolute,
            justify_content: JustifyContent::Center,
            ..default()
        }, MainMenuComponent))
        .with_children(|parent| {
            parent
                .spawn((
                    Text::new("CWEAMPUFF'S ADVENTURE"),
                    TextFont {
                        font: asset_server.load(DEFAULT_FONT),
                        font_size: 90.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                    TextShadow {
                        offset: Vec2::splat(2.),
                        color: Color::linear_rgba(0., 0., 0., 1.),
                    },
                ));
        }).insert(ChildOf(*background));

    commands
        .spawn((Node {
            width: Val::Percent(10.0),
            height: Val::Percent(5.0),
            top: Val::Percent(70.),
            left: Val::Percent(45.),
            position_type: PositionType::Absolute,
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
        }).insert(ChildOf(*background));

    commands
        .spawn((Node {
            width: Val::Percent(10.0),
            height: Val::Percent(5.0),
            top: Val::Percent(77.5),
            left: Val::Percent(45.),
            position_type: PositionType::Absolute,
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
        }).insert(ChildOf(*background));

    commands
        .spawn((Node {
            width: Val::Percent(10.0),
            height: Val::Percent(5.0),
            top: Val::Percent(85.),
            left: Val::Percent(45.),
            position_type: PositionType::Absolute,
            justify_content: JustifyContent::Center,
            ..default()
        }, MainMenuComponent))
        .with_children(|parent| {
            parent
                .spawn((
                    Button,
                    ButtonAction::Credits,
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
                    Text::new("Credits"),
                    TextFont {
                        font: asset_server.load(DEFAULT_FONT),
                        font_size: 33.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                ));
        }).insert(ChildOf(*background));

    commands
        .spawn((Node {
            width: Val::Percent(10.0),
            height: Val::Percent(5.0),
            top: Val::Percent(92.5),
            left: Val::Percent(45.),
            position_type: PositionType::Absolute,
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
        }).insert(ChildOf(*background));

    if main_menu_bgm_query.is_empty() {
        let mut playback_settings = PlaybackSettings::default().with_volume(Volume::Linear(0.));
        playback_settings.mode = PlaybackMode::Loop;
    
        commands.spawn((
            AudioPlayer::new(load_asset(&asset_server, "ost/main.mp3")),
            LevelBGM,
            MainMenuAudio,
            playback_settings
        ));
    
        next_bgm_state.set(LevelBGMState::Changing);
    }
}

pub fn despawn_main_menu(
    mut commands: Commands, 
    query: Query<Entity, (With<Node>, With<MainMenuComponent>, Without<Camera2d>, Without<FadeInFadeOutNode>)>
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn despawn_background(
    mut commands: Commands, 
    query: Query<Entity, (With<Node>, With<MainBackground>, Without<Camera2d>, Without<FadeInFadeOutNode>)>
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}