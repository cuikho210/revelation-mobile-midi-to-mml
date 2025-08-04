use std::{path::PathBuf, thread::sleep, time::Duration};

use lib_player::{MmlPlayer, MmlPlayerOptions};
use midi_to_mml::{MmlSong, MmlSongOptions};
use tracing::Level;

pub fn main() {
    tracing_subscriber::fmt()
        .with_line_number(true)
        .with_file(true)
        .with_max_level(Level::DEBUG)
        .init();

    let song = MmlSong::from_path(
        "../assets/FIRE_BIRD_(full_ver_)_(BanG_Dream!_Roselia_9th_Single)_(piano_cover).mid",
        // "../assets/hishokunosora.mid",
        // "../assets/Stay_With_Me_-_Miki_Matsubara.mid",
        // "../assets/cloudless-yorushika.mid",
        MmlSongOptions {
            auto_boot_velocity: true,
            ..Default::default()
        },
    )
    .unwrap();
    let mut player = MmlPlayer::from_song(
        &song,
        MmlPlayerOptions {
            soundfont_path: vec![PathBuf::from("../gui/assets/soundfonts/a320-neo.sf2")],
        },
    )
    .unwrap();
    player.play(None, None).unwrap();
    sleep(Duration::from_secs(120));
}
