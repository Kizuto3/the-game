use std::path::Path;
use std::sync::atomic::Ordering::SeqCst;
use bevy::asset::{Asset, AssetPath, AssetServer, Handle};
use bevy::prelude::Res;
use crate::USE_PROGRAMMER_ART;

pub fn load_asset<'a, T: Asset>(asset_server: &Res<AssetServer>, path: impl Into<AssetPath<'a>>) -> Handle<T> {
    if USE_PROGRAMMER_ART.load(SeqCst) {
        let path = path.into();
        let prog_art = Path::new("programmer-art").join(path.path());
        if Path::new("assets/").join(&prog_art).exists() {
            asset_server.load(prog_art)
        } else {
            asset_server.load(path)
        }
    } else {
        asset_server.load(path)
    }
}