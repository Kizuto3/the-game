use bevy::prelude::*;

pub const FADE_DELTA: f32 = 2.0;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum FadeState {
    #[default]
    None,
    FadeIn,
    FadeInFinished,
    FadeOut
}

#[derive(Component)]
pub struct FadeInFadeOutNode;

pub fn spawn_fade_in_fade_out_node(
    mut commands: Commands
) {
    commands.spawn((
    Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        top: Val::Percent(0.),
        left: Val::Percent(0.),
        ..default()
        },
        FadeInFadeOutNode,
        GlobalZIndex(i32::MAX),
        BackgroundColor(Color::Srgba(Srgba { red: 0.0, green: 0.0, blue: 0.0, alpha: 0.0 }))
    ));
}

pub fn despawn_fade_in_fade_out_node(
    fade_in_entities: Query<Entity, With<FadeInFadeOutNode>>,
    mut commands: Commands
) {
    for entity in fade_in_entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn set_fade_in_state(mut fade_state: ResMut<NextState<FadeState>>) {
    fade_state.set(FadeState::FadeIn);
}

pub fn set_fade_out_state(mut fade_state: ResMut<NextState<FadeState>>) {
    fade_state.set(FadeState::FadeOut);
}

pub fn fade_in(
    mut fade_in_node: Single<&mut BackgroundColor, With<FadeInFadeOutNode>>,
    time: Res<Time>,
    mut fade_state: ResMut<NextState<FadeState>>
) {
    let a = fade_in_node.0.alpha();

    if a < 1.0 {
        fade_in_node.0.set_alpha(a + time.delta_secs() * FADE_DELTA);

        return;
    }

    fade_state.set(FadeState::FadeInFinished);
}

pub fn fade_out(
    mut fade_in_node: Single<&mut BackgroundColor, With<FadeInFadeOutNode>>,
    time: Res<Time>,
    mut fade_state: ResMut<NextState<FadeState>>
) {
    let a = fade_in_node.0.alpha();
    
    if a > 0.0 {
        fade_in_node.0.set_alpha(a - time.delta_secs() * FADE_DELTA);

        return;
    }
    
    fade_state.set(FadeState::None);
}