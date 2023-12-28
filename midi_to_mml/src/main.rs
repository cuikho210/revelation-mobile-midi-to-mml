// use std::time::Instant;
use midi_to_mml::Song;

fn main() {
    // let time = Instant::now();

    let path = std::path::PathBuf::from("/home/cuikho210/Downloads/midi/furiko_ai_1.mid");
    let song = Song::from_path(path).unwrap();
    let track_0 = song.tracks.get(0).unwrap().to_owned();

    println!("{}", track_0.to_mml());

    // for track in song.tracks.iter() {
    //     println!("{}", track.to_mml());
    //     println!("----------------------------------------");
    // }

    // let elapsed = time.elapsed();
    // println!("{:?}", elapsed);
}
