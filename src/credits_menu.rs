use bevy::{color::Color, prelude::*};

use crate::{app_states::AppState, fade_in_fade_out::FadeInFadeOutNode, main_menu::{DEFAULT_FONT, NORMAL_BUTTON}};

#[derive(Component)]
pub enum CreditsMenuButtonAction {
    Back
}

#[derive(Component)]
pub struct CreditsMenuComponent;

pub fn credits_button_interactions_handler(
    mut interaction_query: Query<(&Interaction, &CreditsMenuButtonAction), (Changed<Interaction>, With<Button>)>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    for (interaction, action) in &mut interaction_query {
        if let Interaction::Pressed = *interaction  {
            match action {
                CreditsMenuButtonAction::Back => {
                    app_state.set(AppState::MainMenu);
                },
            };
        }
    }
}

pub fn spawn_credits_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            position_type: PositionType::Absolute,
            ..default()
        },
        PickingBehavior::IGNORE,
        BackgroundColor(Color::Srgba(Srgba { red: 0.1, green: 0.1, blue: 0.1, alpha: 0. })),
        CreditsMenuComponent
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
        }, CreditsMenuComponent))
        .with_children(|parent| {
            parent
                .spawn((
                    Text::new("Credits"),
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
            height: Val::Percent(50.0),
            top: Val::Percent(20.),
            left: Val::Percent(15.),
            position_type: PositionType::Absolute,
            ..default()
        }, CreditsMenuComponent))
        .with_children(|parent| {                
            parent
                .spawn((
                    Node {
                        width: Val::Percent(50.0),
                        height: Val::Percent(10.0),
                        top: Val::Percent(0.),
                        left: Val::Percent(35.),
                        position_type: PositionType::Absolute,
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                ))
                .with_child((
                    Text::new("-Game design, Programming, Story-"),
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
                        height: Val::Percent(10.0),
                        top: Val::Percent(10.),
                        left: Val::Percent(48.),
                        position_type: PositionType::Absolute,
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                ))
                .with_child((
                    Text::new("Kizuto"),
                    TextFont {
                        font: asset_server.load(DEFAULT_FONT),
                        font_size: 30.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                ));

            parent
                .spawn((
                    Node {
                        width: Val::Percent(50.0),
                        height: Val::Percent(10.0),
                        top: Val::Percent(25.),
                        left: Val::Percent(35.),
                        position_type: PositionType::Absolute,
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                ))
                .with_child((
                    Text::new("-Code review, Story editing-"),
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
                        height: Val::Percent(10.0),
                        top: Val::Percent(35.),
                        left: Val::Percent(48.),
                        position_type: PositionType::Absolute,
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                ))
                .with_child((
                    Text::new("CPU_Blanc"),
                    TextFont {
                        font: asset_server.load(DEFAULT_FONT),
                        font_size: 30.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                ));

            parent
                .spawn((
                    Node {
                        width: Val::Percent(50.0),
                        height: Val::Percent(20.0),
                        top: Val::Percent(50.),
                        left: Val::Percent(33.),
                        position_type: PositionType::Absolute,
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                ))
                .with_child((
                    Text::new("-Art-"),
                    TextFont {
                        font: asset_server.load(DEFAULT_FONT),
                        font_size: 40.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                ));
            
            parent
                .spawn((
                    Node {
                        width: Val::Percent(20.0),
                        height: Val::Percent(20.0),
                        top: Val::Percent(60.),
                        left: Val::Percent(48.),
                        position_type: PositionType::Absolute,
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                ))
                .with_child((
                    Text::new("Grim"),
                    TextFont {
                        font: asset_server.load(DEFAULT_FONT),
                        font_size: 30.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                ));

            parent
                .spawn((
                    Node {
                        width: Val::Percent(50.0),
                        height: Val::Percent(20.0),
                        top: Val::Percent(80.),
                        left: Val::Percent(33.),
                        position_type: PositionType::Absolute,
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                ))
                .with_child((
                    Text::new("-Music-"),
                    TextFont {
                        font: asset_server.load(DEFAULT_FONT),
                        font_size: 40.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                ));
            
            parent
                .spawn((
                    Node {
                        width: Val::Percent(20.0),
                        height: Val::Percent(20.0),
                        top: Val::Percent(90.),
                        left: Val::Percent(48.),
                        position_type: PositionType::Absolute,
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                ))
                .with_child((
                    Text::new("Avilia"),
                    TextFont {
                        font: asset_server.load(DEFAULT_FONT),
                        font_size: 30.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                ));
        });

    main_parent
        .spawn((
            Button,
            CreditsMenuComponent,
            CreditsMenuButtonAction::Back,
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

pub fn despawn_credits_menu(
    mut commands: Commands, 
    query: Query<Entity, (With<Node>, With<CreditsMenuComponent>, Without<Camera2d>, Without<FadeInFadeOutNode>)>
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}