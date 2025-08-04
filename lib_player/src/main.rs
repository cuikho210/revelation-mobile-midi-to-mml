use std::{path::PathBuf, thread::sleep, time::Duration};

use lib_player::{MmlPlayer, MmlPlayerOptions};
use midi_to_mml::{MmlSong, MmlSongOptions};

pub fn main() {
    let song = MmlSong::from_path(
        // "/home/cuikho210/Documents/assets/midi-files/chinese/ban_khong_thuc_su_hanh_phuc.mid",
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
            soundfont_path: vec![PathBuf::from(
                // "/home/cuikho210/Documents/assets/soundfonts/FluidR3_GM.sf2",
                // "/home/cuikho210/Documents/assets/soundfonts/MonalisaGMv2_06_5.sf2",
                "/home/cuikho210/Downloads/a320-neo.sf2",
            )],
        },
    )
    .unwrap();
    player.play(None, None).unwrap();
    sleep(Duration::from_secs(120));
}
