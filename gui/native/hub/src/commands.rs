use rinf::debug_print;
use midi_to_mml_cui::commands;
use revelation_mobile_midi_to_mml::{Song, SongOptions};
use crate::{
    messages::{
        commands::ImportMidiData,
        rust_to_dart::{
            ImportMidiDataOutput,
            SongStatus,
            SongOptions as SignalSongOptions,
        },
    },
    state,
    utils,
};

pub async fn import_midi_data() {
    let mut receiver = ImportMidiData::get_dart_signal_receiver();

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
