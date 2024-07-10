extern crate revelation_mobile_midi_to_mml;

use revelation_mobile_midi_to_mml::{MmlSong, MmlSongOptions};

fn main() {
    let path = "../lib_player/test_resources/midi/_Racing_into_the_NIght_Full_score.mid";
    let song = MmlSong::from_path(path, MmlSongOptions::default()).unwrap();

    for track in song.tracks.iter() {
        println!("Track ... --------------------------\n");
        println!("{}\n", track.to_mml());
    }
}
