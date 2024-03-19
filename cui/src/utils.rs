use std::{
    path::PathBuf,
    io::Error,
    fs,
};
use revelation_mobile_midi_to_mml::{Track, Song, SongOptions};
use crate::types::PathGroup;

pub fn to_path_group(input: &String, output: &Option<String>) -> PathGroup {
    let mut path = PathBuf::from(input);
    let midi_path = path.to_owned();

    let file_name = path.file_name().expect("Invalid file name").to_owned();
    let file_name = file_name.to_str().unwrap();

    path.set_file_name("");
    let dir_path = path;

    let mut json_path = PathBuf::from(&dir_path);

    if let Some(output_path) = output {
        json_path = PathBuf::from(output_path);
    } else {
        json_path = json_path.join(file_name);
        json_path.set_extension("mid_to_mml.json");
    }

    PathGroup {
        json_path,
        midi_path,
    }
}

pub fn print_track_title(track: &Track) {
    println!(
        "Track {} - {} - {} notes --------------------\n",
        track.name,
        track.instrument.name,
        track.mml_note_length,
    );
}

pub fn print_track_mml(track: &Track) {
    println!("{}\n", track.to_mml());
}

pub fn get_song_from_path(path: &String) -> Song {
    let path = PathBuf::from(path);
    
    // If has extension
    if let Some(ext_name) = path.extension() {
        if ext_name == "mid" {
            return get_song_from_midi_path(&path).unwrap();
        } else if ext_name == "json" {
            return get_song_from_json_path(&path).unwrap();
        }
    }

    // Else
    if let Ok(song) = get_song_from_json_path(&path) {
        return song;
    } else if let Ok(song) = get_song_from_midi_path(&path) {
        return song;
    }

    panic!("Cannot open this file");
}

pub fn get_song_from_json_path(path: &PathBuf) -> Result<Song, Error> {
    let json_data = fs::read_to_string(path)?;
    let song = serde_json::from_str(&json_data)?;

    Ok(song)
}

pub fn get_song_from_midi_path(path: &PathBuf) -> Result<Song, Error> {
    Song::from_path(path, SongOptions::default())
}
