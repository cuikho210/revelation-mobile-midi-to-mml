extern crate revelation_mobile_midi_to_mml;

use revelation_mobile_midi_to_mml::{MmlSong, MmlSongOptions};

fn main() {
    let path = "/home/cuikho210/Projects/revelation-mobile-midi-to-mml/lib_player/test_resources/midi/Yoasobi_-_Heart_Beat.mid";
    let mut song = MmlSong::from_path(path, MmlSongOptions {
        auto_boot_velocity: true,
        ..Default::default()
    }).unwrap();

    song.split_track(0).unwrap();

    for track in song.tracks.iter() {
        println!(
            "Track {} - {} --------------------------",
            track.name,
            track.instrument.name,
        );

        println!("{}\n", track.to_mml_debug());
    }
}
