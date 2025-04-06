use bevy::prelude::*;

use crate::audio_settings::AudioSettings;

use super::LevelLayout;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum LevelBGMState {
    Changing,
    #[default]
    Changed
}

#[derive(Component)]
pub struct LevelBGM;

pub fn set_bgm_state(
    level_layout_query: Query<&LevelLayout, With<LevelLayout>>,
    bgm_query: Query<&AudioPlayer, With<LevelBGM>>,
    mut next_bgm_state: ResMut<NextState<LevelBGMState>>,
    asset_server: Res<AssetServer>,
) {
    for level_layout in level_layout_query.iter() {
        match level_layout.bgm {
            Some(bgm) => {
                if bgm_query.is_empty() {
                    next_bgm_state.set(LevelBGMState::Changing);

                    return;
                }

                for current_bgm in bgm_query.iter() {
                    let audio_handle = asset_server.load(format!("ost/{}.mp3", bgm));
                    
                    if current_bgm.0 != audio_handle {
                        next_bgm_state.set(LevelBGMState::Changing);

                        return;
                    }
                }
            }
            None => {
                next_bgm_state.set(LevelBGMState::Changing);

                return;
            }
        }
    }
}

pub fn fade_out_bgm(
    mut bgm_query: Query<&mut AudioSink, With<LevelBGM>>,
    audio_settings: Res<AudioSettings>,
    mut next_bgm_state: ResMut<NextState<LevelBGMState>>,
) {
    for settings in bgm_query.iter_mut() {
        if settings.volume() < audio_settings.bgm_volume {
            let volume_step = audio_settings.bgm_volume * 0.05;

            settings.set_volume((settings.volume() + volume_step).min(audio_settings.bgm_volume));
        }
        else {
            next_bgm_state.set(LevelBGMState::Changed);
            return;
        }
    }
}

pub fn fade_in_bgm(
    mut bgm_query: Query<&mut AudioSink, With<LevelBGM>>,
    audio_settings: Res<AudioSettings>
) {
    for settings in bgm_query.iter_mut() {
        if settings.volume() > 0.01 {
            let volume_step = audio_settings.bgm_volume * 0.05;
            
            settings.set_volume((settings.volume() - volume_step).max(0.0));
        }
    }
}