use bevy::{color::{palettes::css::RED, Color}, prelude::*};

use crate::{cutscene::{CutsceneEvent, CutsceneInfo}, level::{level_layout::starting_room_layout::StartingRoomInfo, Level}};

pub const DEFAULT_FONT: &str = "fonts/Shadows Into Light.ttf";

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

#[derive(Component)]
pub enum ButtonAction {
    StartGame,
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

pub fn button_interactions_handler(
    mut interaction_query: Query<(&Interaction, &ButtonAction), (Changed<Interaction>, With<Button>)>,
    mut exit: EventWriter<AppExit>,
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
                        CutsceneInfo { text: "A brave cweampuff decided to look for her...", background: "" },
                        CutsceneInfo { text: "*** Cweampuff walking on a bridge ****", background: "cutscenes/opening/1.png" },
                        CutsceneInfo { text: "*** The bridge collapses and they fall ****", background: "cutscenes/opening/2.png" },
                    ], Level::StartingRoom(StartingRoomInfo), "vine-boom.mp3"));
                },
                ButtonAction::Quit => {
                    exit.send(AppExit::Success);
                }
            };
        }
    }
}

pub fn spawn_main_menu_buttons(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {    
    commands
        .spawn(Node {
            width: Val::Percent(10.0),
            height: Val::Percent(5.0),
            top: Val::Percent(70.),
            left: Val::Percent(45.),
            justify_content: JustifyContent::Center,
            ..default()
        })
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
        .spawn(Node {
            width: Val::Percent(10.0),
            height: Val::Percent(5.0),
            top: Val::Percent(80.0),
            left: Val::Percent(45.),
            justify_content: JustifyContent::Center,
            ..default()
        })
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
}