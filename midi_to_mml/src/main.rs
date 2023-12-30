// use std::time::Instant;
use midi_to_mml::{Song, SongOptions};

fn main() {
    // let time = Instant::now();

    let path = std::path::PathBuf::from("/home/cuikho210/Downloads/midi/Kamihitoe_Uru.mid");
    let song = Song::from_path(
        path,
        SongOptions::default(),
        // SongOptions {
        //     is_split_track: false,
        //     merge_track: vec![(0, 1)],
        // },
    ).unwrap();

    for track in song.tracks.iter() {
        println!("{}", track.to_mml());
        println!("----------------------------------------");
    }

    // let elapsed = time.elapsed();
    // println!("{:?}", elapsed);
}
