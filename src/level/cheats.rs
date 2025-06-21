use std::sync::atomic::Ordering::{Acquire, SeqCst};
use bevy::prelude::*;

use crate::{Cweampuff, CWEAMPUFF_Z_INDEX, USE_PROGRAMMER_ART};

use super::{
    manually_transition_to_level,
    progression::Progression,
    transition_states::TransitionState,
    Level, LevelLayout,
};

pub fn programmer_art_cheats(
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.all_pressed([KeyCode::KeyA, KeyCode::KeyR]) && keyboard_input.just_pressed(KeyCode::KeyT) {
        USE_PROGRAMMER_ART.store(!USE_PROGRAMMER_ART.load(Acquire), SeqCst);
    }
}

pub fn cheat_transition_to(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    cweampuff: Single<&mut Cweampuff, With<Cweampuff>>,
    commands: Commands,
    current_level_layout: Query<Entity, With<LevelLayout>>,
    transition_state: ResMut<NextState<TransitionState>>,
) {
    if keyboard_input.pressed(KeyCode::KeyH) {
        h_cheats(
            keyboard_input,
            cweampuff,
            commands,
            current_level_layout,
            transition_state,
        );
    } else if keyboard_input.pressed(KeyCode::KeyF) {
        f_cheats(
            keyboard_input,
            cweampuff,
            commands,
            current_level_layout,
            transition_state,
        );
    } else if keyboard_input.pressed(KeyCode::KeyS) {
        s_cheats(
            keyboard_input,
            cweampuff,
            commands,
            current_level_layout,
            transition_state,
        );
    }
}

fn f_cheats(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut cweampuff: Single<&mut Cweampuff, With<Cweampuff>>,
    mut commands: Commands,
    current_level_layout: Query<Entity, With<LevelLayout>>,
    mut transition_state: ResMut<NextState<TransitionState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Digit1) {
        cweampuff.progression = Progression::GivenLetter;
        cweampuff.has_double_jump = true;
        cweampuff.has_dash = true;
        cweampuff.has_wall_jump = false;

        manually_transition_to_level(
            &current_level_layout,
            &mut transition_state,
            &cweampuff,
            &mut commands,
            Level::Factory1,
            Vec3::new(-2450.0, -1450.0, CWEAMPUFF_Z_INDEX),
        );
    } else if keyboard_input.just_pressed(KeyCode::Digit2) {
        cweampuff.progression = Progression::GivenLetter;
        cweampuff.has_double_jump = true;
        cweampuff.has_dash = true;
        cweampuff.has_wall_jump = true;

        manually_transition_to_level(
            &current_level_layout,
            &mut transition_state,
            &cweampuff,
            &mut commands,
            Level::Factory2,
            Vec3::new(-3450.0, -1450.0, CWEAMPUFF_Z_INDEX),
        );
    } else if keyboard_input.just_pressed(KeyCode::Digit3) {
        cweampuff.progression = Progression::GivenLetter;
        cweampuff.has_double_jump = true;
        cweampuff.has_dash = true;
        cweampuff.has_wall_jump = true;

        manually_transition_to_level(
            &current_level_layout,
            &mut transition_state,
            &cweampuff,
            &mut commands,
            Level::Factory3,
            Vec3::new(1725.0, 2950.0, CWEAMPUFF_Z_INDEX),
        );
    } else if keyboard_input.just_pressed(KeyCode::Digit4) {
        cweampuff.progression = Progression::GivenLetter;
        cweampuff.has_double_jump = true;
        cweampuff.has_dash = true;
        cweampuff.has_wall_jump = true;

        manually_transition_to_level(
            &current_level_layout,
            &mut transition_state,
            &cweampuff,
            &mut commands,
            Level::Factory4,
            Vec3::new(1100.0, -2650.0, CWEAMPUFF_Z_INDEX),
        );
    } else if keyboard_input.just_pressed(KeyCode::Digit0) {
        cweampuff.progression = Progression::GivenLetter;
        cweampuff.has_double_jump = true;
        cweampuff.has_dash = true;
        cweampuff.has_wall_jump = true;

        manually_transition_to_level(
            &current_level_layout,
            &mut transition_state,
            &cweampuff,
            &mut commands,
            Level::NeuroLair,
            Vec3::new(1950.0, -700.0, CWEAMPUFF_Z_INDEX),
        );
    }
}

fn s_cheats(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut cweampuff: Single<&mut Cweampuff, With<Cweampuff>>,
    mut commands: Commands,
    current_level_layout: Query<Entity, With<LevelLayout>>,
    mut transition_state: ResMut<NextState<TransitionState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Digit1) {
        cweampuff.progression = Progression::MilkWokeUp;
        cweampuff.has_double_jump = true;
        cweampuff.has_dash = false;
        cweampuff.has_wall_jump = false;

        manually_transition_to_level(
            &current_level_layout,
            &mut transition_state,
            &cweampuff,
            &mut commands,
            Level::Spaceship1,
            Vec3::new(-5600.0, -1950.0, CWEAMPUFF_Z_INDEX),
        );
    } else if keyboard_input.just_pressed(KeyCode::Digit2) {
        cweampuff.progression = Progression::MilkWokeUp;
        cweampuff.has_double_jump = true;
        cweampuff.has_dash = true;
        cweampuff.has_wall_jump = false;

        manually_transition_to_level(
            &current_level_layout,
            &mut transition_state,
            &cweampuff,
            &mut commands,
            Level::Spaceship2,
            Vec3::new(2400.0, -1300.0, CWEAMPUFF_Z_INDEX),
        );
    } else if keyboard_input.just_pressed(KeyCode::Digit3) {
        cweampuff.progression = Progression::MilkWokeUp;
        cweampuff.has_double_jump = true;
        cweampuff.has_dash = true;
        cweampuff.has_wall_jump = false;

        manually_transition_to_level(
            &current_level_layout,
            &mut transition_state,
            &cweampuff,
            &mut commands,
            Level::Spaceship3,
            Vec3::new(-4600.0, -1600.0, CWEAMPUFF_Z_INDEX),
        );
    } else if keyboard_input.just_pressed(KeyCode::Digit4) {
        cweampuff.progression = Progression::MilkWokeUp;
        cweampuff.has_double_jump = true;
        cweampuff.has_dash = true;
        cweampuff.has_wall_jump = false;

        manually_transition_to_level(
            &current_level_layout,
            &mut transition_state,
            &cweampuff,
            &mut commands,
            Level::Spaceship4,
            Vec3::new(1100.0, -2600.0, CWEAMPUFF_Z_INDEX),
        );
    } else if keyboard_input.just_pressed(KeyCode::Digit0) {
        cweampuff.progression = Progression::MilkWokeUp;
        cweampuff.has_double_jump = true;
        cweampuff.has_dash = true;
        cweampuff.has_wall_jump = false;

        manually_transition_to_level(
            &current_level_layout,
            &mut transition_state,
            &cweampuff,
            &mut commands,
            Level::AquwaLair,
            Vec3::new(1950.0, -700.0, CWEAMPUFF_Z_INDEX),
        );
    }
}

fn h_cheats(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut cweampuff: Single<&mut Cweampuff, With<Cweampuff>>,
    mut commands: Commands,
    current_level_layout: Query<Entity, With<LevelLayout>>,
    mut transition_state: ResMut<NextState<TransitionState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Digit1) {
        cweampuff.progression = Progression::MetMilk;
        cweampuff.has_double_jump = false;
        cweampuff.has_dash = false;
        cweampuff.has_wall_jump = false;

        manually_transition_to_level(
            &current_level_layout,
            &mut transition_state,
            &cweampuff,
            &mut commands,
            Level::Hell1,
            Vec3::new(150.0, 1500.0, CWEAMPUFF_Z_INDEX),
        );
    } else if keyboard_input.just_pressed(KeyCode::Digit2) {
        cweampuff.progression = Progression::MetMilk;
        cweampuff.has_double_jump = true;
        cweampuff.has_dash = false;
        cweampuff.has_wall_jump = false;
        manually_transition_to_level(
            &current_level_layout,
            &mut transition_state,
            &cweampuff,
            &mut commands,
            Level::Hell2,
            Vec3::new(150.0, 1500.0, CWEAMPUFF_Z_INDEX),
        );
    } else if keyboard_input.just_pressed(KeyCode::Digit3) {
        cweampuff.progression = Progression::MetMilk;
        cweampuff.has_double_jump = true;
        cweampuff.has_dash = false;
        cweampuff.has_wall_jump = false;

        manually_transition_to_level(
            &current_level_layout,
            &mut transition_state,
            &cweampuff,
            &mut commands,
            Level::Hell3,
            Vec3::new(2400.0, 1350.0, CWEAMPUFF_Z_INDEX),
        );
    } else if keyboard_input.just_pressed(KeyCode::Digit4) {
        cweampuff.progression = Progression::MetMilk;
        cweampuff.has_double_jump = true;
        cweampuff.has_dash = false;
        cweampuff.has_wall_jump = false;

        manually_transition_to_level(
            &current_level_layout,
            &mut transition_state,
            &cweampuff,
            &mut commands,
            Level::Hell4,
            Vec3::new(1550.0, -2850.0, CWEAMPUFF_Z_INDEX),
        );
    } else if keyboard_input.just_pressed(KeyCode::Digit0) {
        cweampuff.progression = Progression::MetMilk;
        cweampuff.has_double_jump = true;
        cweampuff.has_dash = false;
        cweampuff.has_wall_jump = false;

        manually_transition_to_level(
            &current_level_layout,
            &mut transition_state,
            &cweampuff,
            &mut commands,
            Level::CerberLair,
            Vec3::new(2050.0, -700.0, CWEAMPUFF_Z_INDEX),
        );
    }
}
