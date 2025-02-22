pub mod interaction_state;

use bevy::prelude::*;

#[derive(Component)]
pub struct Interactable;

pub fn spawn_interaction_prompt(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands
        .spawn((
            Node {
            width: Val::Percent(100.0),
            height: Val::Percent(10.0),
            top: Val::Percent(90.),
            left: Val::Percent(0.),
            justify_content: JustifyContent::Center,
            ..default()
        }, Interactable))
        .with_children(|parent| {
            parent
                .spawn((
                    Text::new("[E]: Interact"),
                    TextFont {
                        font: asset_server.load("fonts/Shadows Into Light.ttf"),
                        font_size: 50.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                ));
        });
}

pub fn despawn_interaction_prompt(
    propmpts: Query<Entity, (With<Node>, With<Interactable>, Without<Camera2d>)>,
    mut commands: Commands
) {
    for prompt in propmpts.iter() {
        commands.entity(prompt).despawn_recursive();
    }
}