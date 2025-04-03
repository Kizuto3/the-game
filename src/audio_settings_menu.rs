use bevy::{color::Color, prelude::*};

use crate::{app_states::AppState, audio_settings::{AudioSettings, MAX_VOLUME}, fade_in_fade_out::FadeInFadeOutNode, main_menu::{DEFAULT_FONT, NORMAL_BUTTON}, Cweampuff};

const VOLUME_STEP: f32 = 0.1;

#[derive(Component)]
pub enum SoundMenuButtonAction {
    IncreaseMusicVolume,
    DecreaseMusicVolume,
    IncreaseSoundVolume,
    DecreaseSoundVolume,
    Back
}

#[derive(Component)]
pub struct AudioSettingsComponent;

#[derive(Component)]
pub struct SoundVolumeText;

#[derive(Component)]
pub struct MusicVolumeText;

pub fn settings_menu_input_reader(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut app_state: ResMut<NextState<AppState>>,
    current_app_state: Res<State<AppState>>,
    cweampuff_query: Query<&Cweampuff>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        if let AppState::AudioMenu = **current_app_state {
            if cweampuff_query.is_empty() {
                app_state.set(AppState::MainMenu);

                return;
            }

            app_state.set(AppState::InGame);

            return;
        }

        app_state.set(AppState::AudioMenu);
    }
}

pub fn audio_button_interactions_handler(
    mut interaction_query: Query<(&Interaction, &SoundMenuButtonAction), (Changed<Interaction>, With<Button>)>,
    mut sound_text_query: Query<&mut Text, (With<SoundVolumeText>, Without<MusicVolumeText>)>,
    mut music_text_query: Query<&mut Text, (With<MusicVolumeText>, Without<SoundVolumeText>)>,
    mut audio_query: Query<&mut AudioSink>,
    mut audio_settings: ResMut<AudioSettings>,
    mut app_state: ResMut<NextState<AppState>>,
    cweampuff_query: Query<&Cweampuff>,
) {
    for (interaction, action) in &mut interaction_query {
        if let Interaction::Pressed = *interaction  {
            match action {
                SoundMenuButtonAction::IncreaseMusicVolume => {
                    audio_settings.bgm_volume = (audio_settings.bgm_volume + VOLUME_STEP).min(MAX_VOLUME);
                },
                SoundMenuButtonAction::DecreaseMusicVolume => {
                    audio_settings.bgm_volume = (audio_settings.bgm_volume - VOLUME_STEP).max(0.0);
                    
                },
                SoundMenuButtonAction::IncreaseSoundVolume => {
                    audio_settings.sfx_volume = (audio_settings.sfx_volume + VOLUME_STEP).min(MAX_VOLUME);
                },
                SoundMenuButtonAction::DecreaseSoundVolume => {
                    audio_settings.sfx_volume = (audio_settings.sfx_volume - VOLUME_STEP).max(0.0);
                },
                SoundMenuButtonAction::Back => {
                    if !cweampuff_query.is_empty() {
                        app_state.set(AppState::InGame);
                    }
                    else {
                        app_state.set(AppState::MainMenu);
                    }
                },
            };

            let sound_text_value = ((audio_settings.sfx_volume * 100.) / MAX_VOLUME).round() as i32;
            let music_text_value = ((audio_settings.bgm_volume * 100.) / MAX_VOLUME).round() as i32;

            for mut sound_text in sound_text_query.iter_mut() {
                **sound_text = format!("{}", sound_text_value);
            }

            for mut music_text in music_text_query.iter_mut() {
                **music_text = format!("{}", music_text_value);
            }

            for settings in audio_query.iter_mut() {        
                settings.set_volume(audio_settings.bgm_volume);
            }
        }
    }
}

pub fn spawn_audio_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio_settings: Res<AudioSettings>,
    cweampuff_query: Query<&Cweampuff>,
) {
    let sound_text_value = ((audio_settings.sfx_volume * 100.) / MAX_VOLUME).round() as i32;
    let music_text_value = ((audio_settings.bgm_volume * 100.) / MAX_VOLUME).round() as i32;

    let sound_text_value = format!("{}", sound_text_value);
    let music_text_value = format!("{}", music_text_value);

    let background_alpha = if cweampuff_query.is_empty() {
        0.
    }
    else {
        0.95
    };

    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            position_type: PositionType::Absolute,
            ..default()
        },
        PickingBehavior::IGNORE,
        BackgroundColor(Color::Srgba(Srgba { red: 0.1, green: 0.1, blue: 0.1, alpha: background_alpha })),
        AudioSettingsComponent
    )).with_children(|main_parent| {
    main_parent
        .spawn((Node {
            width: Val::Percent(30.0),
            height: Val::Percent(20.0),
            top: Val::Percent(10.),
            left: Val::Percent(35.),
            position_type: PositionType::Absolute,
            justify_content: JustifyContent::Center,
            ..default()
        }, AudioSettingsComponent))
        .with_children(|parent| {
            parent
                .spawn((
                    Text::new("Audio"),
                    TextFont {
                        font: asset_server.load(DEFAULT_FONT),
                        font_size: 40.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                ));
        });

    main_parent
        .spawn((Node {
            width: Val::Percent(60.0),
            height: Val::Percent(60.0),
            top: Val::Percent(30.),
            left: Val::Percent(15.),
            position_type: PositionType::Absolute,
            ..default()
        }, AudioSettingsComponent))
        .with_children(|parent| {                
            parent
                .spawn((
                    Node {
                        width: Val::Percent(50.0),
                        height: Val::Percent(20.0),
                        top: Val::Percent(8.),
                        left: Val::Percent(10.),
                        position_type: PositionType::Absolute,
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                ))
                .with_child((
                    Text::new("Music Volume:"),
                    TextFont {
                        font: asset_server.load(DEFAULT_FONT),
                        font_size: 33.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                ));
            
            parent
                .spawn((
                    Node {
                        width: Val::Percent(20.0),
                        height: Val::Percent(20.0),
                        top: Val::Percent(8.),
                        left: Val::Percent(75.),
                        position_type: PositionType::Absolute,
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                ))
                .with_child((
                    Text::new(music_text_value),
                    MusicVolumeText,
                    TextFont {
                        font: asset_server.load(DEFAULT_FONT),
                        font_size: 33.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                ));

            parent
                .spawn((
                    Button,
                    SoundMenuButtonAction::IncreaseMusicVolume,
                    Node {
                        width: Val::Percent(5.0),
                        height: Val::Percent(5.0),
                        top: Val::Percent(13.),
                        left: Val::Percent(90.),
                        position_type: PositionType::Absolute,
                        border: UiRect::all(Val::Px(5.0)),
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
                    Text::new("^"),
                    TextFont {
                        font: asset_server.load(DEFAULT_FONT),
                        font_size: 14.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                ));

            parent
                .spawn((
                    Button,
                    SoundMenuButtonAction::DecreaseMusicVolume,
                    Node {
                        width: Val::Percent(5.0),
                        height: Val::Percent(5.0),
                        top: Val::Percent(19.),
                        left: Val::Percent(90.),
                        position_type: PositionType::Absolute,
                        border: UiRect::all(Val::Px(5.0)),
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
                    Text::new("v"),
                    TextFont {
                        font: asset_server.load(DEFAULT_FONT),
                        font_size: 14.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                ));

            parent
                .spawn((
                    Node {
                        width: Val::Percent(50.0),
                        height: Val::Percent(20.0),
                        top: Val::Percent(38.),
                        left: Val::Percent(10.),
                        position_type: PositionType::Absolute,
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                ))
                .with_child((
                    Text::new("Sound Volume:"),
                    TextFont {
                        font: asset_server.load(DEFAULT_FONT),
                        font_size: 33.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                ));
            
            parent
                .spawn((
                    Node {
                        width: Val::Percent(20.0),
                        height: Val::Percent(20.0),
                        top: Val::Percent(38.),
                        left: Val::Percent(75.),
                        position_type: PositionType::Absolute,
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                ))
                .with_child((
                    Text::new(sound_text_value),
                    SoundVolumeText,
                    TextFont {
                        font: asset_server.load(DEFAULT_FONT),
                        font_size: 33.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                ));

            parent
                .spawn((
                    Button,
                    SoundMenuButtonAction::IncreaseSoundVolume,
                    Node {
                        width: Val::Percent(5.0),
                        height: Val::Percent(5.0),
                        top: Val::Percent(43.),
                        left: Val::Percent(90.),
                        position_type: PositionType::Absolute,
                        border: UiRect::all(Val::Px(5.0)),
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
                    Text::new("^"),
                    TextFont {
                        font: asset_server.load(DEFAULT_FONT),
                        font_size: 14.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                ));

            parent
                .spawn((
                    Button,
                    SoundMenuButtonAction::DecreaseSoundVolume,
                    Node {
                        width: Val::Percent(5.0),
                        height: Val::Percent(5.0),
                        top: Val::Percent(49.),
                        left: Val::Percent(90.),
                        position_type: PositionType::Absolute,
                        border: UiRect::all(Val::Px(5.0)),
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
                    Text::new("v"),
                    TextFont {
                        font: asset_server.load(DEFAULT_FONT),
                        font_size: 14.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                ));
        });

    main_parent
        .spawn((
            Button,
            AudioSettingsComponent,
            SoundMenuButtonAction::Back,
            Node {
                width: Val::Percent(10.0),
                height: Val::Percent(5.0),
                top: Val::Percent(80.),
                left: Val::Percent(45.),
                position_type: PositionType::Absolute,
                border: UiRect::all(Val::Px(5.0)),
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
            Text::new("Back"),
            TextFont {
                font: asset_server.load(DEFAULT_FONT),
                font_size: 33.0,
                ..default()
            },
            TextColor(Color::srgb(0.9, 0.9, 0.9)),
        ));
    });
    
}

pub fn despawn_audio_settings(
    mut commands: Commands, 
    query: Query<Entity, (With<Node>, With<AudioSettingsComponent>, Without<Camera2d>, Without<FadeInFadeOutNode>)>
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}