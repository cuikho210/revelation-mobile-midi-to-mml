// use std::time::Instant;
use midi_to_mml::Song;

fn main() {
    // let time = Instant::now();

    let path = std::path::PathBuf::from("/home/cuikho210/Downloads/3184436_My_Dearest_Guilty_Crown_TheIshter_Sheets_90_Done_2.mid");
    let song = Song::from_path(path).unwrap();

    for track in song.tracks.iter() {
        println!("{}", track.to_mml());
        println!("----------------------------------------");
    }

    // let elapsed = time.elapsed();
    // println!("{:?}", elapsed);
}
