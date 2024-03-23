extern crate revelation_mobile_midi_to_mml;
use revelation_mobile_midi_to_mml::{Song, SongOptions};

fn main() {
    let path = std::path::PathBuf::from("/Users/tonyk/Downloads/Shinunoga_E-Wa__Fujii_Kaze__.mid");
    let song = Song::from_path(
        path,
        // SongOptions::default(),
        SongOptions {
            auto_boot_velocity: false,
            velocity_min: 10,
            velocity_max: 15,
        },
    )
    .unwrap();

    for track in song.tracks.iter() {
        println!(
            "\nTrack {} - {} - {} notes ----------------------------------\n",
            track.name,
            track.instrument.name,
            track.mml_note_length,
        );
        println!("{}", track.to_mml());
    }
}
