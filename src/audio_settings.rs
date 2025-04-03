use bevy::prelude::*;

pub const MAX_VOLUME: f32 = 2.0; 

#[derive(Resource)]
pub struct AudioSettings {
    pub bgm_volume: f32,
    pub sfx_volume: f32
}

impl Default for AudioSettings {
    fn default() -> Self {
        Self { 
            bgm_volume: 1.0,
            sfx_volume: 1.0
        }
    }
}