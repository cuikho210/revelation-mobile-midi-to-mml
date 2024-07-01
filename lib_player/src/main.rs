use std::{thread::sleep, time::Duration};
use lib_player::Synth;

fn main() {
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
