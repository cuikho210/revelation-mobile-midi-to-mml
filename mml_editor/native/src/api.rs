use midi_to_mml::Song;

pub fn parse_midi(bytes: Vec<u8>) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let song = Song::from_bytes(bytes).unwrap();

    for track in song.tracks.iter() {
        result.push(track.to_mml());
    }

    result
}
