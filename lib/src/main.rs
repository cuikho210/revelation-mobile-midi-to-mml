extern crate revelation_mobile_midi_to_mml;

use revelation_mobile_midi_to_mml::{MmlSong, MmlSongOptions};

fn main() {
    let path = "/home/cuikho210/Downloads/My_Neighbor_Totoro_-_Joe_Hisaishi1900_followers_SP.mid";
    let song = MmlSong::from_path(path, MmlSongOptions::default()).unwrap();

    for track in song.tracks.iter() {
        println!("Track ... --------------------------\n");
        println!("{}\n", track.to_mml_debug());
    }
}
