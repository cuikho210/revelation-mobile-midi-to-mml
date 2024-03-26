use std::{
    path::PathBuf,
    io::{Error, Write},
    fs,
};
use revelation_mobile_midi_to_mml::{Track, Song, SongOptions};
use crate::{
    types::PathGroup,
    commands,
};

pub fn modify_json_file<C>(input: &String, callback: C)
where C: Fn(&mut Song)
{
    let path = PathBuf::from(input);
    let mut song = get_song_from_json_path(&path).unwrap();

    callback(&mut song);

    let json = commands::to_json(&song);
    save_json(&json, &path);
}

pub fn string_to_bool_arg(arg: &String) -> bool {
    let be_true = ["true", "0"];
    let arg = arg.to_lowercase();

    for value in be_true {
        if arg == value {
            return true;
        }
    }

    false
}

pub fn save_json(json: &String, path: &PathBuf) {
    let mut file = fs::File::create(path).unwrap();
    file.write_all(json.as_bytes()).unwrap();

    println!("Saved json file to {}", path.display());
}

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

pub fn get_track_title(index: &usize, track: &Track) -> String {
    format!(
        "{} - Track '{}' - {} - {} notes --------------------\n",
        index,
        track.name,
        track.instrument.name,
        track.mml_note_length,
    )
}

pub fn get_track_mml(track: &Track) -> String {
    format!("{}\n", track.to_mml())
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
