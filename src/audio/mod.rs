use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioControl, AudioSource};

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        // for the compiler
        // app.init_state::<AudioTrack>() // No more State<AudioTrack>
        app.add_plugins(bevy_kira_audio::AudioPlugin)
            .add_event::<TrackSwitchEvent>() // Register the new event
            .add_systems(Startup, setup_audio_assets) // Load assets at startup
            .add_systems(Startup, initial_audio_playback) // Initial audio playback using event
            .add_systems(Update, handle_track_switch_events) // System to handle TrackSwitchEvents
            .add_systems(Update, cycle_sound_system);
    }
}

// Event to signal a track switch is requested (unchanged)
#[derive(Event, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TrackSwitchEvent {
    pub track: AudioTrack,
}

// Function to get the correct audio handle based on AudioTrack state (unchanged)
fn get_audio_track_handle(
    track: &AudioTrack,
    asset_server: &Res<AssetServer>,
) -> Handle<AudioSource> {
    match track {
        AudioTrack::MainMenuLoop => asset_server.load("sound/ogg/Shooter Synthwave 1 Loop.ogg"),
        AudioTrack::GameplayLoop2 => asset_server.load("sound/ogg/Shooter Synthwave 2 Loop.ogg"),
        AudioTrack::GameplayLoop3 => asset_server.load("sound/ogg/Shooter Synthwave 3 Loop.ogg"),
        AudioTrack::GameplayLoop4 => asset_server.load("sound/ogg/Shooter Synthwave 4 Loop.ogg"),
        AudioTrack::GameplayLoop5 => asset_server.load("sound/ogg/Shooter Synthwave 5 Loop.ogg"),
        AudioTrack::GameplayLoop6 => asset_server.load("sound/ogg/Shooter Synthwave 6 Loop.ogg"),
        AudioTrack::GameplayLoop7 => asset_server.load("sound/ogg/Shooter Synthwave 7 Loop.ogg"),
        AudioTrack::GameplayLoop8 => asset_server.load("sound/ogg/Shooter Synthwave 8 Loop.ogg"),
        AudioTrack::SoftLoopForDialog => {
            asset_server.load("sound/ogg/(Soft Loop For Dialogues, Pause or Other Things) 9.ogg")
        }
    }
}

// Function to get the next AudioTrack in cycle - now takes AudioTrack and returns AudioTrack (unchanged)
fn get_next_audio_track(current_track: &AudioTrack) -> AudioTrack {
    match current_track {
        AudioTrack::MainMenuLoop => AudioTrack::GameplayLoop2,
        AudioTrack::GameplayLoop2 => AudioTrack::GameplayLoop3,
        AudioTrack::GameplayLoop3 => AudioTrack::GameplayLoop4,
        AudioTrack::GameplayLoop4 => AudioTrack::GameplayLoop5,
        AudioTrack::GameplayLoop5 => AudioTrack::GameplayLoop6,
        AudioTrack::GameplayLoop6 => AudioTrack::GameplayLoop7,
        AudioTrack::GameplayLoop7 => AudioTrack::GameplayLoop8,
        AudioTrack::GameplayLoop8 => AudioTrack::SoftLoopForDialog,
        AudioTrack::SoftLoopForDialog => AudioTrack::MainMenuLoop,
    }
}

// Load audio assets - Startup (unchanged - empty)
pub fn setup_audio_assets(_commands: Commands) {}

// Startup system to trigger initial audio playback via event - now hardcoded initial track
pub fn initial_audio_playback(
    mut event_writer: EventWriter<TrackSwitchEvent>,
    // current_track_state: Res<State<AudioTrack>>, // No more State resource
) {
    // let initial_track = current_track_state.get(); // No state to get from
    let initial_track = AudioTrack::MainMenuLoop; // Hardcode initial track to MainMenuLoop
    event_writer.send(TrackSwitchEvent {
        track: initial_track,
    }); // Send event for initial track
}

// System to handle TrackSwitchEvents and play audio (unchanged)
pub fn handle_track_switch_events(
    mut event_reader: EventReader<TrackSwitchEvent>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
) {
    for event in event_reader.read() {
        let track_to_play = event.track;
        audio.stop(); // Stop any currently playing audio

        let track_handle = get_audio_track_handle(&track_to_play, &asset_server);
        audio.play(track_handle.clone()).looped();

        println!("Playing track due to event: {:?}", track_to_play); // Optional debug log
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)] // Removed States derive, no longer a Bevy State
pub enum AudioTrack {
    #[default]
    MainMenuLoop,
    GameplayLoop2,
    GameplayLoop3,
    GameplayLoop4,
    GameplayLoop5,
    GameplayLoop6,
    GameplayLoop7,
    GameplayLoop8,
    SoftLoopForDialog,
}

pub fn cycle_sound_system(
    input: Res<ButtonInput<KeyCode>>,
    // mut next_track_state: ResMut<NextState<AudioTrack>>, // No more State resource
    // current_track_state: Res<State<AudioTrack>>, // No more State resource
    mut event_writer: EventWriter<TrackSwitchEvent>, // Event writer for TrackSwitchEvent
    mut current_track: Local<AudioTrack>, // Local to track "current" track within this system
) {
    if input.just_pressed(KeyCode::Space) {
        // let current_track_state = current_track_state.get(); // No State resource anymore
        let next_track_enum = get_next_audio_track(&current_track); // Get next track based on *Local* 'current_track'
        println!("Requesting Track Switch to: {:?}", next_track_enum); // Optional debug log

        event_writer.send(TrackSwitchEvent {
            track: next_track_enum,
        }); // Send TrackSwitchEvent

        // next_track_state.set(next_track); // No more State resource to set
        *current_track = next_track_enum; // Update the Local 'current_track' for the next cycle
    }
}
