extern crate revelation_mobile_midi_to_mml;

use revelation_mobile_midi_to_mml::{MmlSong, MmlSongOptions};

fn main() {
    let path = "/home/cuikho210/Projects/revelation-mobile-midi-to-mml/lib_player/test_resources/midi/The_cat_returns_-_Become_the_wind_-_Kaze_ni_naru.mid";
    let mut song = MmlSong::from_path(path, MmlSongOptions {
        auto_boot_velocity: true,
        ..Default::default()
    }).unwrap();

    song.merge_tracks(0, 1).unwrap();

    for track in song.tracks.iter() {
        println!(
            "Track {} - {} --------------------------",
            track.name,
            track.instrument.name,
        );

        println!("{}\n", track.to_mml_debug());
    }
}
