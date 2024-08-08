use lib_player::{MmlPlayer, MmlPlayerOptions};
use std::path::PathBuf;

use crate::messages::types::SignalPlayStatus;

pub struct PlayerState {
    pub player: MmlPlayer,
    pub playback_state: SignalPlayStatus,
} 

impl PlayerState {
    pub fn new() -> Self {
        PlayerState {
            player: MmlPlayer::new(MmlPlayerOptions {
                soundfont_path: vec![
                    PathBuf::from("/home/cuikho210/Documents/soundfonts/FluidR3_GM.sf2"),
                ],
            }),
            playback_state: SignalPlayStatus::Stop,
        }
    }

    pub fn play(&mut self) {
        self.playback_state = SignalPlayStatus::Play;
        self.player.play(None);
    }

    pub fn stop(&mut self) {
        self.playback_state = SignalPlayStatus::Stop;
    }
}
