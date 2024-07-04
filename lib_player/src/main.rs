use std::{
    thread::sleep,
    time::{Duration, Instant},
};
use lib_player::{Parser, Synth, MmlPlayer, MmlPlayerOptions};
use revelation_mobile_midi_to_mml::{Song, SongOptions};

fn main() {
    test_from_midi();
}

fn test_from_midi() {
    // let midi_path = std::path::PathBuf::from("./test_resouces/rex_incognito.mid");
    // let midi_path = std::path::PathBuf::from("./test_resouces/Hitchcock.mid");
    // let midi_path = std::path::PathBuf::from("./test_resouces/Cloudless_Yorushika.mid");
    // let midi_path = std::path::PathBuf::from("./test_resouces/Kiseki.mid");
    // let midi_path = std::path::PathBuf::from("./test_resouces/ghost_in_a_flower.mid");
    // let midi_path = std::path::PathBuf::from("./test_resouces/Senbonzakura.mid");
    let midi_path = std::path::PathBuf::from("./test_resouces/Kirameki_Your_Lie_in_April_ED_.mid");
    // let midi_path = std::path::PathBuf::from("./test_resouces/Yorushika_-_Rain_with_Cappuccino.mid");

    let song = Song::from_path(midi_path, SongOptions {
        auto_boot_velocity: true,
        ..Default::default()
    }).unwrap();
    let mmls: Vec<String> = song.tracks.iter().map::<String, _>(|track| track.to_mml()).collect();
    let track_length = mmls.len();

    let sf2 = String::from("./test_resouces/East_West_-_The_Ultimate_Piano_Collection.sf2");
    // let sf2 = String::from("./test_resouces/Yamaha_C3_Grand_Piano.sf2");
    
    let time = Instant::now();

    let player = MmlPlayer::from_mmls(mmls, MmlPlayerOptions {
        soundfont_path: sf2,
    });

    println!("Created player with {} tracks in {}ms", track_length, time.elapsed().as_millis());

    player.play();
}

fn _test_simple() {
    let mml_1 = "t82v8o5C8C16<B16>C16<G8G16>D16C8D16E8r16C16G16E16E16E16D8D16E16D8C16<B16>C8r16<G16>C16C16C16C16C16C16<G16G16>D16E16D16C16D16E16D16C16F8E16E16D8C16G8.&G32.";

    let time = Instant::now();
    let parsed = Parser::parse(mml_1.to_string());
    println!("parse 1: {}", time.elapsed().as_nanos());

    let synth = Synth::new();
    let (_stream, connection) = synth.new_stream(
        String::from("./test_resouces/YDP-GrandPiano-SF2-20160804/YDP-GrandPiano-20160804.sf2")
    );

    parsed.play(connection, 0);
    sleep(Duration::from_millis(parsed.duration_in_ms as u64));
}
