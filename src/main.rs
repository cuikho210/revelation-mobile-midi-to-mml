// use std::time::Instant;
use midi_to_mml::Song;

fn main() {
    // let time = Instant::now();
    let current_dir = std::path::PathBuf::from("/home/cuikho210/Documents/projects/lib/midi-to-mml");

    // let path = current_dir.join("midi_files/Orange_Your_Lie_in_April.mid");
    let path = current_dir.join("midi_files/Rush_E.mid");

    let song = Song::from_path(path).unwrap();

    for track in song.tracks.iter() {
        println!("{}", track.to_mml());
        println!("----------------------------------------");
    }

    // let elapsed = time.elapsed();
    // println!("{:?}", elapsed);
}
