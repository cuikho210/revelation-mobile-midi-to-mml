use lib_player::{MmlPlayer, MmlPlayerOptions, NoteOnCallbackData};
use revelation_mobile_midi_to_mml::Instrument;
use rinf::debug_print;
use std::{path::PathBuf, sync::Arc};
use crate::messages::{rust_to_dart::SignalMmlNoteOn, types::SignalPlayStatus};

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

    pub fn parse_mmls(&mut self, mmls: Vec<(String, Instrument)>) {
        self.player.parse_mmls(mmls);
    }

    pub fn play(&mut self) {
        let callback: Arc<fn(NoteOnCallbackData)> = Arc::new(|data: NoteOnCallbackData| {
            SignalMmlNoteOn {
                track_id: data.track_index as u64,
                char_index: data.char_index as u64,
                char_length: data.char_length as u64,
            }.send_signal_to_dart();
        });

        self.player.play(Some(callback));
        self.playback_state = SignalPlayStatus::Play;
        debug_print!("Set player play");
    }

    pub fn pause(&mut self) {
        self.player.pause();
        self.playback_state = SignalPlayStatus::Pause;
        debug_print!("Set player pause");
    }

    pub fn stop(&mut self) {
        self.player.stop();
        self.playback_state = SignalPlayStatus::Stop;
        debug_print!("Set player stop");
    }
}

