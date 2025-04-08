use bevy::prelude::*;

use crate::{Cweampuff, CWEAMPUFF_Z_INDEX};

use super::{level_layout::{factory_1_layout::Factory1Info, factory_2_layout::Factory2Info, factory_3_layout::Factory3Info, factory_4_layout::Factory4Info, hell_1_layout::Hell1Info, hell_2_layout::Hell2Info, hell_3_layout::Hell3Info, hell_4_layout::Hell4Info, spaceship_1_layout::Spaceship1Info, spaceship_2_layout::Spaceship2Info, spaceship_3_layout::Spaceship3Info, spaceship_4_layout::Spaceship4Info}, manually_transition_to_level, progression::Progression, transition_states::TransitionState, Level, LevelLayout};

pub fn cheat_transition_to(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut cweampuff: Single<&mut Cweampuff, With<Cweampuff>>,
    mut commands: Commands,
    current_level_layout: Query<Entity, With<LevelLayout>>,
    mut transition_state: ResMut<NextState<TransitionState>>
) {
    if keyboard_input.pressed(KeyCode::KeyH) && keyboard_input.just_pressed(KeyCode::Digit1) {
        cweampuff.progression = Progression::MetMilk;
        cweampuff.has_double_jump = false;
        cweampuff.has_dash = false;
        cweampuff.has_wall_jump = false;

        manually_transition_to_level(&current_level_layout, &mut transition_state, &cweampuff, &mut commands, Level::Hell1(Hell1Info), Vec3::new(150.0, 1500.0, CWEAMPUFF_Z_INDEX));

        return;
    }

    if keyboard_input.pressed(KeyCode::KeyH) && keyboard_input.just_pressed(KeyCode::Digit2) {
        cweampuff.progression = Progression::MetMilk;
        cweampuff.has_double_jump = true;
        cweampuff.has_dash = false;
        cweampuff.has_wall_jump = false;

        manually_transition_to_level(&current_level_layout, &mut transition_state, &cweampuff, &mut commands, Level::Hell2(Hell2Info), Vec3::new(150.0, 1500.0, CWEAMPUFF_Z_INDEX));

        return;
    }

    if keyboard_input.pressed(KeyCode::KeyH) && keyboard_input.just_pressed(KeyCode::Digit3) {
        cweampuff.progression = Progression::MetMilk;
        cweampuff.has_double_jump = true;
        cweampuff.has_dash = false;
        cweampuff.has_wall_jump = false;

        manually_transition_to_level(&current_level_layout, &mut transition_state, &cweampuff, &mut commands, Level::Hell3(Hell3Info), Vec3::new(2400.0, 1350.0, CWEAMPUFF_Z_INDEX));

        return;
    }

    if keyboard_input.pressed(KeyCode::KeyH) && keyboard_input.just_pressed(KeyCode::Digit4) {
        cweampuff.progression = Progression::MetMilk;
        cweampuff.has_double_jump = true;
        cweampuff.has_dash = false;
        cweampuff.has_wall_jump = false;

        manually_transition_to_level(&current_level_layout, &mut transition_state, &cweampuff, &mut commands, Level::Hell4(Hell4Info), Vec3::new(1550.0, -2850.0, CWEAMPUFF_Z_INDEX));

        return;
    }

    if keyboard_input.pressed(KeyCode::KeyS) && keyboard_input.just_pressed(KeyCode::Digit1) {
        cweampuff.progression = Progression::MilkWokeUp;
        cweampuff.has_double_jump = true;
        cweampuff.has_dash = false;
        cweampuff.has_wall_jump = false;

        manually_transition_to_level(&current_level_layout, &mut transition_state, &cweampuff, &mut commands, Level::Spaceship1(Spaceship1Info), Vec3::new(-5600.0, -1950.0, CWEAMPUFF_Z_INDEX));

        return;
    }

    if keyboard_input.pressed(KeyCode::KeyS) && keyboard_input.just_pressed(KeyCode::Digit2) {
        cweampuff.progression = Progression::MilkWokeUp;
        cweampuff.has_double_jump = true;
        cweampuff.has_dash = true;
        cweampuff.has_wall_jump = false;

        manually_transition_to_level(&current_level_layout, &mut transition_state, &cweampuff, &mut commands, Level::Spaceship2(Spaceship2Info), Vec3::new(2400.0, -1300.0, CWEAMPUFF_Z_INDEX));

        return;
    }

    if keyboard_input.pressed(KeyCode::KeyS) && keyboard_input.just_pressed(KeyCode::Digit3) {
        cweampuff.progression = Progression::MilkWokeUp;
        cweampuff.has_double_jump = true;
        cweampuff.has_dash = true;
        cweampuff.has_wall_jump = false;

        manually_transition_to_level(&current_level_layout, &mut transition_state, &cweampuff, &mut commands, Level::Spaceship3(Spaceship3Info), Vec3::new(-4600.0, -1600.0, CWEAMPUFF_Z_INDEX));

        return;
    }

    if keyboard_input.pressed(KeyCode::KeyS) && keyboard_input.just_pressed(KeyCode::Digit4) {
        cweampuff.progression = Progression::MilkWokeUp;
        cweampuff.has_double_jump = true;
        cweampuff.has_dash = true;
        cweampuff.has_wall_jump = false;

        manually_transition_to_level(&current_level_layout, &mut transition_state, &cweampuff, &mut commands, Level::Spaceship4(Spaceship4Info), Vec3::new(1100.0, -2600.0, CWEAMPUFF_Z_INDEX));

        return;
    }

    if keyboard_input.pressed(KeyCode::KeyF) && keyboard_input.just_pressed(KeyCode::Digit1) {
        cweampuff.progression = Progression::GivenLetter;
        cweampuff.has_double_jump = true;
        cweampuff.has_dash = true;
        cweampuff.has_wall_jump = false;

        manually_transition_to_level(&current_level_layout, &mut transition_state, &cweampuff, &mut commands, Level::Factory1(Factory1Info), Vec3::new(-2450.0, -1450.0, CWEAMPUFF_Z_INDEX));

        return;
    }

    if keyboard_input.pressed(KeyCode::KeyF) && keyboard_input.just_pressed(KeyCode::Digit2) {
        cweampuff.progression = Progression::GivenLetter;
        cweampuff.has_double_jump = true;
        cweampuff.has_dash = true;
        cweampuff.has_wall_jump = true;

        manually_transition_to_level(&current_level_layout, &mut transition_state, &cweampuff, &mut commands, Level::Factory2(Factory2Info), Vec3::new(-3450.0, -1450.0, CWEAMPUFF_Z_INDEX));

        return;
    }

    if keyboard_input.pressed(KeyCode::KeyF) && keyboard_input.just_pressed(KeyCode::Digit3) {
        cweampuff.progression = Progression::GivenLetter;
        cweampuff.has_double_jump = true;
        cweampuff.has_dash = true;
        cweampuff.has_wall_jump = true;

        manually_transition_to_level(&current_level_layout, &mut transition_state, &cweampuff, &mut commands, Level::Factory3(Factory3Info), Vec3::new(1725.0, 2950.0, CWEAMPUFF_Z_INDEX));

        return;
    }

    if keyboard_input.pressed(KeyCode::KeyF) && keyboard_input.just_pressed(KeyCode::Digit4) {
        cweampuff.progression = Progression::GivenLetter;
        cweampuff.has_double_jump = true;
        cweampuff.has_dash = true;
        cweampuff.has_wall_jump = true;

        manually_transition_to_level(&current_level_layout, &mut transition_state, &cweampuff, &mut commands, Level::Factory4(Factory4Info), Vec3::new(1100.0, -2650.0, CWEAMPUFF_Z_INDEX));

        return;
    }
}