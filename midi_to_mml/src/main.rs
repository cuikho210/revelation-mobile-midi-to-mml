// use std::time::Instant;
use midi_to_mml::Song;

fn main() {
    // let time = Instant::now();

    let path = std::path::PathBuf::from("/home/cuikho210/Downloads/y2mate.com - 振り子  Uru  Furiko pendulum  耳コピして弾いてみた ピアノ ひぽさんふらわー映画罪の声主題歌.mid");
    let song = Song::from_path(path).unwrap();
    let track_0 = song.tracks.get(0).unwrap().to_owned();
    let (track_a, track_b) = track_0.split();

    println!("{}", track_a.to_mml());
    println!("----------------------------------------");
    println!("{}", track_b.to_mml());

    // for track in song.tracks.iter() {
    //     println!("{}", track.to_mml());
    //     println!("----------------------------------------");
    // }

    // let elapsed = time.elapsed();
    // println!("{:?}", elapsed);
}
