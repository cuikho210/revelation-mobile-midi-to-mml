// use std::time::Instant;
use midi_to_mml::{Song, SongOptions};

fn main() {
    // let time = Instant::now();

    let path = std::path::PathBuf::from("/home/cuikho210/Downloads/Hikaru_Nara__guangrunara_-_Goose_house_Piano__Violin.mid");
    let song = Song::from_path(
        path,
        // SongOptions::default(),
        SongOptions {
            is_split_track: false,
            merge_track: vec![],
        },
    )
    .unwrap();

    for track in song.tracks.iter() {
        println!("\n{} -------------------------------------\n", track.instrument_name);
        println!("{}", track.to_mml());
    }

    // let elapsed = time.elapsed();
    // println!("{:?}", elapsed);
}
