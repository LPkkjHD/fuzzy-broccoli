use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioControl};

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        // for the compiler
        app.add_plugins(bevy_kira_audio::AudioPlugin);
        app.add_systems(Startup, test_music);
    }
}

pub fn test_music(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    audio
        .play(asset_server.load("sound/ogg/Shooter Synthwave 1 Loop.ogg"))
        .looped();
}

pub enum AudioTrack {
    MainMenuLoop,      // “Shooter Synthwave 1 Loop.wav”
    GameplayLoop1,     // “Shooter Synthwave 2 Loop.wav”
    GameplayLoop2,     // “Shooter Synthwave 3 Loop.wav”
    SoftLoopForDialog, // “(Soft Loop For Dialogues, Pause or Other Things) 9.wav”
}
