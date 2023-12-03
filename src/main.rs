use midi_to_mml::Song;

fn main() {
    // let song = Song::from_path("midi_files/Canon_in_D.mid").unwrap();
    // let song = Song::from_path("midi_files/Orange_Your_Lie_in_April.mid").unwrap();
    // let song = Song::from_path("midi_files/Fur_Elise.mid").unwrap();
    let song = Song::from_path("midi_files/Rush_E.mid").unwrap();

    for track in song.tracks.iter() {
        println!("{:#?}", track.to_mml());
        println!("----------------------------------------");
    }
}