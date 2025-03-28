use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioControl, AudioSource};

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AudioTrack>()
            .add_plugins(bevy_kira_audio::AudioPlugin)
            .add_event::<TrackSwitchEvent>() // Register the new event
            .add_systems(Startup, initial_audio_playback) // Initial audio playback using event
            .add_systems(Update, handle_track_switch_events) // System to handle TrackSwitchEvents
            .add_systems(Update, cycle_sound_system);
    }
}

// Event to signal a track switch is requested
#[derive(Event, Debug, Clone, PartialEq, Eq, Hash)]
pub struct TrackSwitchEvent {
    pub track: AudioTrack,
}

// Function to get the correct audio handle based on AudioTrack state
fn get_audio_track_handle(
    track: &AudioTrack,
    asset_server: &Res<AssetServer>,
) -> Handle<AudioSource> {
    match track {
        AudioTrack::MainMenuLoop => {
            asset_server.load("sound/ogg/Shooter Synthwave 1 Loop.ogg") // Replace with your paths
        }
        AudioTrack::GameplayLoop1 => asset_server.load("sound/ogg/Shooter Synthwave 2 Loop.ogg"),
        AudioTrack::GameplayLoop2 => asset_server.load("sound/ogg/Shooter Synthwave 3 Loop.ogg"),
        AudioTrack::SoftLoopForDialog => {
            asset_server.load("sound/ogg/(Soft Loop For Dialogues, Pause or Other Things) 9.ogg")
        }
    }
}

// Function to get the next AudioTrack state in the cycle (unchanged)
fn get_next_audio_track(current_track: &AudioTrack) -> AudioTrack {
    match current_track {
        AudioTrack::MainMenuLoop => AudioTrack::GameplayLoop1,
        AudioTrack::GameplayLoop1 => AudioTrack::GameplayLoop2,
        AudioTrack::GameplayLoop2 => AudioTrack::SoftLoopForDialog,
        AudioTrack::SoftLoopForDialog => AudioTrack::MainMenuLoop,
    }
}

// Startup system to trigger initial audio playback via event
pub fn initial_audio_playback(
    mut event_writer: EventWriter<TrackSwitchEvent>,
    current_track_state: Res<State<AudioTrack>>,
) {
    let initial_track = current_track_state.get();
    event_writer.send(TrackSwitchEvent {
        track: initial_track.clone(),
    }); // Send event for initial track
}

// System to handle TrackSwitchEvents and play audio
pub fn handle_track_switch_events(
    mut event_reader: EventReader<TrackSwitchEvent>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
) {
    for event in event_reader.read() {
        let track_to_play = event.track.clone();
        audio.stop(); // Stop any currently playing audio

        let track_handle = get_audio_track_handle(&track_to_play, &asset_server);
        audio.play(track_handle.clone()).looped();

        println!("Playing track due to event: {:?}", track_to_play); // Optional debug log
    }
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum AudioTrack {
    #[default]
    MainMenuLoop,
    GameplayLoop1,
    GameplayLoop2,
    SoftLoopForDialog,
}

pub fn cycle_sound_system(
    input: Res<ButtonInput<KeyCode>>,
    mut next_track_state: ResMut<NextState<AudioTrack>>,
    current_track_state: Res<State<AudioTrack>>,
    mut event_writer: EventWriter<TrackSwitchEvent>, // Event writer for TrackSwitchEvent
) {
    if input.just_pressed(KeyCode::Space) {
        let current_track = current_track_state.get();
        let next_track = get_next_audio_track(current_track);

        println!("Requesting Track Switch to: {:?}", next_track); // Optional debug log

        event_writer.send(TrackSwitchEvent {
            track: next_track.clone(),
        }); // Send TrackSwitchEvent

        next_track_state.set(next_track); // Still manage state, for other systems if needed, or remove if purely event-driven
    }
}
