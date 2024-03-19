use crate::utils;
use revelation_mobile_midi_to_mml::{Song, SongOptions};
use std::{
    fs::File,
    io::Write,
};

pub fn midi_to_json(input: &String, output: &Option<String>) {
    let path_group = utils::to_path_group(input, output);
    let song = Song::from_path(
        path_group.midi_path.to_owned(),
        SongOptions::default(),
    ).unwrap();

    let json = serde_json::to_string(&song).unwrap();
    let mut file = File::create(&path_group.json_path).unwrap();
    file.write_all(json.as_bytes()).unwrap();

    println!("Saved json file to {}", path_group.json_path.display());
}

pub fn list_tracks(song: &Song) {
    for track in song.tracks.iter() {
        utils::print_track_title(track);
    }
}

pub fn to_mml(input: &String) {
    let song = utils::get_song_from_path(input);

    println!("\n------------------------------------------------------------------------------------");
    println!("|     MIDI to MML - https://github.com/cuikho210/revelation-mobile-midi-to-mml     |");
    println!("------------------------------------------------------------------------------------\n");

    for track in song.tracks.iter() {
        utils::print_track_title(track);
        utils::print_track_mml(track);
    }
}
