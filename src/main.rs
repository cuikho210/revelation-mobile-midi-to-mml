// use std::time::Instant;
use midi_to_mml::{Song, SongOptions};

fn main() {
    // let time = Instant::now();

    let path = std::path::PathBuf::from("/Users/tonyk/Downloads/test_drum_set.mid");
    let song = Song::from_path(
        path,
        // SongOptions::default(),
        SongOptions {
            auto_boot_velocity: true,
            velocity_min: 10,
            velocity_max: 15,
        },
    )
    .unwrap();

    for track in song.tracks.iter() {
        println!("\n{} -------------------------------------\n", track.instrument.name);
        println!("{}", track.to_mml());
    }

    // let elapsed = time.elapsed();
    // println!("{:?}", elapsed);
}
