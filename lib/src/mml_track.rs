use crate::{
    Instrument,
    mml_event::{BridgeEvent, MidiNoteState, MmlEvent},
    mml_song::MmlSongOptions,
    parser::bridge_events_to_mml_events,
    utils,
};

#[derive(Debug, Clone)]
pub struct MmlTrack {
    pub name: String,
    pub instrument: Instrument,
    pub events: Vec<MmlEvent>,
    pub song_options: MmlSongOptions,
    pub bridge_meta_events: Vec<BridgeEvent>,
    pub bridge_note_events: Vec<BridgeEvent>,
    pub bridge_events: Vec<BridgeEvent>,
    pub ppq: u16,
    pub mml_note_length: usize,
}

impl MmlTrack {
    pub fn from_bridge_events(
        name: String,
        bridge_meta_events: Vec<BridgeEvent>,
        bridge_note_events: Vec<BridgeEvent>,
        song_options: MmlSongOptions,
        ppq: u16,
    ) -> Self {
        let mut mml_track = Self {
            name,
            events: Vec::new(),
            instrument: Instrument::default(),
            bridge_meta_events,
            bridge_note_events,
            bridge_events: Vec::new(),
            song_options,
            ppq,
            mml_note_length: 0,
        };

        mml_track.generate_mml_events();
        mml_track
    }

    pub fn split(&self) -> (Self, Self) {
        let (mut track_a, mut track_b) = self.split_track_by_override();

        if self.song_options.auto_equalize_note_length {
            let is_out_of_range = track_a.mml_note_length > 3000 || track_b.mml_note_length > 3000;

            let is_too_different = {
                let ratio = 0.5_f32;
                let diff_a =
                    track_a.mml_note_length > (track_b.mml_note_length as f32 * ratio) as usize;
                let diff_b =
                    track_b.mml_note_length > (track_a.mml_note_length as f32 * ratio) as usize;
                diff_a || diff_b
            };

            if is_out_of_range || is_too_different {
                utils::equalize_tracks(&mut track_a, &mut track_b);
            }
        }

        track_a.instrument = self.instrument.to_owned();
        track_b.instrument = self.instrument.to_owned();

        (track_a, track_b)
    }

    pub fn merge(&mut self, other: &mut Self) {
        self.bridge_note_events
            .append(&mut other.bridge_note_events);
        self.bridge_note_events.sort();

        self.name = format!("{}+{}", &self.name, other.name);
        self.generate_mml_events();
    }

    pub fn to_mml(&self) -> String {
        let mut mml = String::new();

        for event in self.events.iter() {
            mml.push_str(&event.to_mml(self.song_options.smallest_unit));
        }

        mml
    }

    // TODO: Whitespace might cause errors in the game
    // pub fn to_mml_debug(&self) -> String {
    //     let mut mml = String::new();
    //     let mut notes_on_row: usize = 0;
    //
    //     for event in self.events.iter() {
    //         let current_mml = &event.to_mml_debug(self.song_options.smallest_unit);
    //         mml.push_str(&current_mml);
    //
    //         notes_on_row += event.get_duration();
    //         if notes_on_row >= 64 {
    //             notes_on_row = 0;
    //             mml.push('\n');
    //         }
    //     }
    //
    //     mml
    // }

    pub fn apply_boot_velocity(&mut self, velocity_diff: u8) {
        if velocity_diff > 0 {
            for event in self.events.iter_mut() {
                if let MmlEvent::Velocity(velocity) = event {
                    *velocity += velocity_diff
                }
            }
        }
    }

    pub fn generate_mml_events(&mut self) {
        self.apply_meta_events();

        let (events, instrument) =
            bridge_events_to_mml_events(&self.bridge_events, &self.song_options, self.ppq);

        if let Some(instrument) = instrument {
            self.instrument = instrument;
        }

        self.events = events;
        self.update_mml_note_length();
    }

    fn apply_meta_events(&mut self) {
        self.bridge_events = self.bridge_note_events.to_owned();
        self.bridge_events
            .append(&mut self.bridge_meta_events.to_owned());
        self.bridge_events.sort();
    }

    fn update_mml_note_length(&mut self) {
        let mut note_length = 0usize;

        for event in self.events.iter() {
            if let MmlEvent::Note(note) = event {
                note_length += note.mml_note_length;
            }
        }

        self.mml_note_length = note_length;
    }

    fn split_track_by_override(&self) -> (Self, Self) {
        let mut max_end_position = 0usize;
        let mut before_note: Option<MidiNoteState> = None;
        let mut bridges_a: Vec<BridgeEvent> = Vec::new();
        let mut bridges_b: Vec<BridgeEvent> = Vec::new();

        for i in 0..self.bridge_note_events.len() {
            let current_bridge_event_ref = self.bridge_note_events.get(i).unwrap();
            let current_bridge_event = current_bridge_event_ref.to_owned();

            if let BridgeEvent::Note(current_note) = current_bridge_event_ref {
                let current_end_position = current_note.midi_state.position_in_tick
                    + current_note.midi_state.duration_in_tick;

                if current_end_position > max_end_position {
                    max_end_position = current_end_position;
                }

                if let Some(before_note) = before_note {
                    let note_pos_isize = current_note.midi_state.position_in_tick as isize;
                    let before_note_pos_isize = before_note.midi_state.position_in_tick as isize;
                    let start_pos_diff = note_pos_isize - before_note_pos_isize;
                    let min_gap_for_chord_isize = self.song_options.min_gap_for_chord as isize;
                    let min_gap_for_chord_in_smallest_unit =
                        min_gap_for_chord_isize * self.song_options.smallest_unit as isize;

                    if start_pos_diff <= min_gap_for_chord_in_smallest_unit {
                        bridges_a.push(current_bridge_event);
                    } else if current_note.midi_state.position_in_tick < max_end_position {
                        bridges_b.push(current_bridge_event);
                    } else {
                        bridges_a.push(current_bridge_event);
                    }
                } else {
                    bridges_a.push(current_bridge_event);
                }

                before_note = Some(current_note.to_owned());
            }
        }

        let track_a = Self::from_bridge_events(
            format!("{}.0", self.name),
            self.bridge_meta_events.to_owned(),
            bridges_a,
            self.song_options.to_owned(),
            self.ppq,
        );

        let track_b = Self::from_bridge_events(
            format!("{}.1", self.name),
            self.bridge_meta_events.to_owned(),
            bridges_b,
            self.song_options.to_owned(),
            self.ppq,
        );

        (track_a, track_b)
    }
}
