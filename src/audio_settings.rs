use bevy::prelude::*;

#[derive(Resource)]
pub struct AudioSettings {
    pub bgm_volume: f32,
    pub sfx_volume: f32
}

impl Default for AudioSettings {
    fn default() -> Self {
        Self { 
            bgm_volume: 0.2,
            sfx_volume: 0.2
        }
    }
}