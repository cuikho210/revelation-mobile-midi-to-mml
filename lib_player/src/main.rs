use std::{
    thread::{sleep, self},
    time::{Duration, Instant},
};
use lib_player::{Parser, Synth, SynthOutputConnection};

fn main() {
    test_from_midi();
}

fn test_from_midi() {
    use revelation_mobile_midi_to_mml::{Song, SongOptions};

    let synth = Synth::new(String::from("./test_resouces/YDP-GrandPiano-SF2-20160804/YDP-GrandPiano-20160804.sf2"));
    let (_stream, connection) = synth.new_stream();

    let midi_path = std::path::PathBuf::from("./test_resouces/rex_incognito.mid");
    let song = Song::from_path(midi_path, SongOptions {
        auto_boot_velocity: true,
        ..Default::default()
    }).unwrap();

    let mut parses: Vec<(SynthOutputConnection, Parser)> = Vec::new();
    let mut max_duration = 0;

    for track in song.tracks.iter() {
        let conn = connection.clone();
        let parsed = Parser::parse(track.to_mml());

        if parsed.duration_in_ms > max_duration {
            max_duration = parsed.duration_in_ms;
        }

        parses.push((conn, parsed));
    }

    let mut index = 0;
    for (mut conn, parsed) in parses {
        thread::spawn(move || parsed.play(&mut conn, index));
        index += 1;
    }

    sleep(Duration::from_millis(max_duration as u64));
}

fn _test_simple() {
    let mml_1 = "t82v8o5C8C16<B16>C16<G8G16>D16C8D16E8r16C16G16E16E16E16D8D16E16D8C16<B16>C8r16<G16>C16C16C16C16C16C16<G16G16>D16E16D16C16D16E16D16C16F8E16E16D8C16G8.&G32.";

    let time = Instant::now();
    let parsed = Parser::parse(mml_1.to_string());
    println!("parse 1: {}", time.elapsed().as_nanos());

    let synth = Synth::new(String::from("./test_resouces/YDP-GrandPiano-SF2-20160804/YDP-GrandPiano-20160804.sf2"));
    let (_stream, mut connection) = synth.new_stream();

    parsed.play(&mut connection, 0);
    sleep(Duration::from_millis(parsed.duration_in_ms as u64));
}
