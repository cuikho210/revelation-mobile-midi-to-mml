// use std::time::Instant;
use midi_to_mml::{Song, SongOptions};

fn main() {
    // let time = Instant::now();

    let path = std::path::PathBuf::from("/Users/tonyk/Downloads/nxlpxv_2.mid");
    let song = Song::from_path(
        path,
        // SongOptions::default(),
        SongOptions {
            auto_boot_velocity: true,
            velocity_min: 11,
            velocity_max: 15,
        },
    )
    .unwrap();

    // for track in song.tracks.iter() {
    //     println!("\n{} -------------------------------------\n", track.instrument.name);
    //     println!("{}", track.to_mml());
    // }
    
    let mut piano_2 = song.tracks.get(0).unwrap().to_owned();
    piano_2.to_percussion();

    let piano_2 = piano_2.split();

    println!("\n{} -------------------------------------\n", piano_2.0.instrument.name);
    println!("{}", piano_2.0.notes.first().unwrap().is_percussion);
    println!("{}", piano_2.0.to_mml());
    println!("\n{} -------------------------------------\n", piano_2.1.instrument.name);
    println!("{}", piano_2.1.notes.first().unwrap().is_percussion);
    println!("{}", piano_2.1.to_mml());

    // let elapsed = time.elapsed();
    // println!("{:?}", elapsed);
}
