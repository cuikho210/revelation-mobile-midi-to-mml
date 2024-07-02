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
    let mml = "t82v8o5C8v11r1.&r1.&r1.&r1.&r1.&r1.&r1.&r1.&r1.&r1.&r1.&r1.&r1&r4.G8.&G32.v12r4&r64G8r16G8r8G8&G32.r1&r4.&r64C8r1.&r4.G8r16G8r8G8&G32.r1&r4.&r64C8v8r1.&r1.&r1&r8C8r1.&r4.C8r1.&r1.&r1.&r1.&r1.&r1.&r1.&r1.&r1.&r1.&r1.&r1&r8E8.&E32.v11r1.&r1.&r4&r64G8v12r4.G8r16G8r8G8&G32.r64>C8<B8G8A8r2.&r8C8r1.&r4.G8r16G8r8G8&G32.r4.&r64A8G16G8r2&r8.C8v8r1.&r1.&r1.&r1&r4.<F8.&F32.r1.&r1.&r2.&r64F4&F16.&F64v14r1.&r1.&r1.&r1.&r1.&r1.&r8&r64>G+8r16G+8r8G+8&G+32.r1&r4.&r64C+8r1.&r8>D+16C+16C+8<G+8r16G+8r8G+8&G+32.r4.&r64>C+8.&C+32.r4&r64F8&F32.r4&r16&r64<C+8r1.&r4.C+8v12r1.&r4.C+8&C+32.r1.&r1.&r1.&r1.&r1.&r1&r16&r64A+8.&A+32.:>C+8.&C+32.";
    Parser::parse(mml.to_string());
}

fn main() {
    test_mml_parser();
}
