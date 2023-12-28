// use std::time::Instant;
use midi_to_mml::Song;

fn main() {
    // let time = Instant::now();

    let path = std::path::PathBuf::from("/home/cuikho210/Downloads/Itsumo_nando_demo_-_piano.mid");
    let song = Song::from_path(path, false).unwrap();

    for track in song.tracks.iter() {
        println!("{}", track.to_mml());
        println!("----------------------------------------");
    }

    // let elapsed = time.elapsed();
    // println!("{:?}", elapsed);
}
