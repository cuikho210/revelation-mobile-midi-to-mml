extern crate revelation_mobile_midi_to_mml;
use revelation_mobile_midi_to_mml::{Song, SongOptions};

fn main() {
    let path = std::path::PathBuf::from("/home/cuikho210/Downloads/Shinunoga_E-Wa__Fujii_Kaze__.mid");
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

    let track0 = song.tracks.get(0).unwrap().to_owned();
    let track1 = song.tracks.get(1).unwrap().split();
    let tracks = [track0, track1.0, track1.1];

    for track in tracks.iter() {
        println!(
            "\nTrack {} - {} - {} notes ----------------------------------\n",
            track.name,
            track.instrument.name,
            track.mml_note_length,
        );
        println!("{}", track.to_mml());
    }
}
