use rinf::debug_print;
use revelation_mobile_midi_to_mml::{Song, SongOptions};
use crate::{
    messages::{
        commands,
        rust_to_dart::{
            ImportMidiDataOutput,
            SplitTrackOutput,
            MergeTracksOutput,
            GetMmlOutput,
        },
        types::{
            SongStatus,
            SongOptions as SignalSongOptions,
        },
    },
    state,
    utils,
};

pub async fn to_mml() {
    let mut receiver = commands::ToMml::get_dart_signal_receiver();

    while let Some(dart_signal) = receiver.recv().await {
        let options = dart_signal.message.options.unwrap();
        state::set_song_options(options).await;

        let mml = state::get_mml().await;
        GetMmlOutput { mml }.send_signal_to_dart(None);
    }
}

pub async fn merge_tracks() {
    let mut receiver = commands::Merge::get_dart_signal_receiver();

    while let Some(dart_signal) = receiver.recv().await {
        let signal = dart_signal.message;
        let index_a: usize = signal.index_a.try_into().unwrap();
        let index_b: usize = signal.index_b.try_into().unwrap();
        let tracks = state::merge_tracks(index_a, index_b).await;

        MergeTracksOutput { tracks }.send_signal_to_dart(None);
    }
}

pub async fn split_track() {
    let mut receiver = commands::Split::get_dart_signal_receiver();

    while let Some(dart_signal) = receiver.recv().await {
        let signal = dart_signal.message;
        let split_index: usize = signal.index.try_into().unwrap();
        let tracks = state::split_track(split_index).await;

        SplitTrackOutput { tracks }.send_signal_to_dart(None);
    }
}

pub async fn import_midi_data() {
    let mut receiver = commands::ImportMidiData::get_dart_signal_receiver();

    while let Some(dart_signal) = receiver.recv().await {
        let signal = dart_signal.message;
        let song = Song::from_path(signal.path, SongOptions::default());

        if let Ok(song) = song {
            ImportMidiDataOutput {
                is_ok: true,
                song_status: Some(SongStatus {
                    options: Some(SignalSongOptions {
                        auto_boot_velocity: song.options.auto_boot_velocity,
                        velocity_min: song.options.velocity_min.into(),
                        velocity_max: song.options.velocity_max.into(),
                    }),
                    tracks: utils::get_tracks_from_song(&song),
                }),
            }.send_signal_to_dart(None);

            state::set_temp_song(song).await;
            debug_print!("import_midi_data: Song saved in state");
        } else {
            ImportMidiDataOutput {
                is_ok: false,
                song_status: None,
            }.send_signal_to_dart(None);
        }
    }
}
