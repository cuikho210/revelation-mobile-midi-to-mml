use std::{thread::sleep, time::Duration};
use lib_player::{Synth, Parser};

fn test_synth() {
    let synth = Synth::new(String::from("./test_resouces/YDP-GrandPiano-SF2-20160804/YDP-GrandPiano-20160804.sf2"));
    let (_stream, mut connection) = synth.new_stream();

    connection.note_on(0, 60, 100);
    connection.note_on(0, 64, 100);
    connection.note_on(0, 67, 100);
    sleep(Duration::from_millis(700));
    connection.note_off(0, 60);
    connection.note_off(0, 64);
    connection.note_off(0, 67);

    sleep(Duration::from_secs(3));
}

fn test_mml_parser() {
    let mml = "t80v12r1o4F8:C8:<A8:F8+:C8:<F8o4F8:C8:<A8:F8:C8:<F8o4F8:C8:<A8:F8:C8:<F8o4F16:C16:<A16:F16:C16:<F16o4F16:C16:<A16:F16:C16:<F16o4F16:C16:<A16:F16:C16:<F16o4F16:C16:<A16:F16:C16:<F16o4F8:C8:<A8:F8:C8:<F8o4F8:C8:<A8:F8:C8:<F8o4F16:C16:<A16:F16:C16:<F16o4F16:C16:<A16:F16:C16:<F16o4F8:C8:<A8:F8:C8:<F8o4F8:C8:<A8:F8:C8:<F8o4F8:C8:<A8:F8:C8:<F8o4F16:C16:<A16:F16:C16:<F16";
    Parser::parse(mml.to_string());
}

fn main() {
    test_mml_parser();
}
