use midi_to_mml::Song;

fn main() {
    let song = Song::from_path("/home/cuikho210/Desktop/midi/Happy_Birthday_To_You_Piano.mid").unwrap();
    let track = song.tracks.first().unwrap();
    println!("{:#?}", track.to_mml());
}