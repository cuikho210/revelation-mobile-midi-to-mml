use lib_player::{MmlPlayer, MmlPlayerOptions};
use revelation_mobile_midi_to_mml::Instrument;
use std::{path::PathBuf, thread::sleep, time::Duration};

fn main() {
    let mut mml = MmlPlayer::from_mmls(
        vec![("".into(), Instrument::new(0, 0))],
        MmlPlayerOptions {
            soundfont_path: vec![PathBuf::from(
                "/home/cuikho210/Documents/assets/soundfonts/FluidR3_GM.sf2",
            )],
        },
    );

    mml.play(None, None);
    sleep(Duration::from_secs(30));
}
