use std::{
    sync::{Arc, Mutex}, time::{Duration, Instant}
};
use revelation_mobile_midi_to_mml::Instrument;
use crate::{
    mml_event::MmlEvent,
    note_event::NoteEvent, SynthOutputConnection,
    mml_player::NoteOnCallbackData,
    utils,
};

const NOTE_NAMES: [char; 8] = ['c', 'd', 'e', 'f', 'g', 'a', 'b', 'r'];

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlaybackStatus {
    PLAY,
    PAUSE,
    STOP,
}

#[derive(Debug, Clone)]
pub struct Parser {
    pub raw_mml: String,
    pub notes: Vec<NoteEvent>,
    pub instrument: Instrument,
    pub connection: SynthOutputConnection,

    status: Arc<Mutex<PlaybackStatus>>,
    note_on_callback: Option<Arc<fn(NoteOnCallbackData)>>,
    note_before: Option<NoteEvent>,
    time_start: Option<Instant>,
    current_chord: Vec<NoteEvent>,
    absolute_duration: isize,
    current_note_index: usize,
}

impl Parser {
    pub fn parse(
        mml: String,
        instrument: Instrument,
        connection: SynthOutputConnection,
        playback_status: Arc<Mutex<PlaybackStatus>>,
    ) -> Self {
        let program_id = instrument.instrument_id;
        let channel = instrument.midi_channel;

        let mut result = Self {
            raw_mml: mml,
            notes: Vec::new(),
            instrument,
            connection,
            status: playback_status,
            note_on_callback: None,
            note_before: None,
            current_chord: Vec::new(),
            time_start: None,
            absolute_duration: 0,
            current_note_index: 0,
        };

        result.parse_note_events();
        result.connection.program_change(channel, program_id);
        result
    }

    pub fn play(&mut self, note_on_callback: Option<Arc<fn(NoteOnCallbackData)>>) {
        self.note_on_callback = note_on_callback;
        self.time_start = Some(Instant::now());
        self.set_playback_status(PlaybackStatus::PLAY);
        self.play_next();
    }

    pub fn pause(&mut self) {
        self.set_playback_status(PlaybackStatus::STOP);
    }

    pub fn stop(&mut self) {
        self.reset_state();
        self.set_playback_status(PlaybackStatus::STOP);
    }

    fn set_playback_status(&mut self, value: PlaybackStatus) {
        let mut guard = self.status.lock().unwrap();
        *guard = value;
    }

    fn reset_state(&mut self) {
        self.current_note_index = 0;
        self.absolute_duration = 0;
        self.note_before = None;
        self.current_chord = Vec::new();
        self.time_start = None;
    }

    fn play_next(&mut self) {
        let playback_status = self.status.lock().unwrap();
        if *playback_status != PlaybackStatus::PLAY {
            return;
        }
        drop(playback_status);

        let mut connection = self.connection.clone();

        if let Some(note) = self.notes.get(self.current_note_index) {
            let time = self.time_start.unwrap();

            if note.is_connected_to_prev_note {
                if let Some(before_note) = self.note_before.as_ref() {
                    if self.current_chord.len() == 0 {
                        self.current_chord.push(before_note.to_owned());
                    }
                }

                self.current_chord.push(note.to_owned());
                self.current_note_index += 1;
                return self.play_next();
            }

            let correct_duration = time.elapsed().as_millis() as isize;
            let duration_diff = correct_duration - self.absolute_duration;

            if self.current_chord.len() > 0 {
                let chord_duration = utils::get_longest_note_duration(&self.current_chord);
                let duration = chord_duration - duration_diff;
                let duration = Duration::from_millis(duration as u64);

                utils::play_chord(
                    &mut connection,
                    &self.current_chord,
                    self.instrument.midi_channel,
                    Some(duration),
                );
                send_note_on_event_from_chord(&self.note_on_callback, &self.current_chord);

                self.absolute_duration += chord_duration;
                self.current_chord.clear();
                self.note_before = Some(note.to_owned());

                self.current_note_index += 1;
                return self.play_next();
            }

            if let Some(before_note) = &self.note_before {
                let note_duration = before_note.duration_in_ms as isize;
                let duration = note_duration - duration_diff;
                let duration = Duration::from_millis(duration as u64);

                utils::play_note(
                    &mut connection,
                    before_note,
                    self.instrument.midi_channel,
                    Some(duration),
                );
                send_note_on_event_from_note(&self.note_on_callback, before_note);

                self.absolute_duration += note_duration;
            }

            self.note_before = Some(note.to_owned());
            self.current_note_index += 1;
            return self.play_next();
        }

        if self.current_chord.len() > 0 {
            utils::play_chord(
                &mut connection,
                &self.current_chord,
                self.instrument.midi_channel,
                None,
            );
            send_note_on_event_from_chord(&self.note_on_callback, &self.current_chord);
        }

        if let Some(before_note) = self.note_before.as_ref() {
            utils::play_note(
                &mut connection,
                &before_note,
                self.instrument.midi_channel,
                None,
            );
            send_note_on_event_from_note(&self.note_on_callback, &before_note);
        }
    }

    fn parse_note_events(&mut self) {
        let mut index = 0usize;
        let mut current_mml_velocity = 12u8;
        let mut current_octave = 4u8;
        let mut current_tempo = 120usize;
        let mut is_connect_chord = false;

        loop {
            if let Some(event) = self.parse_event(
                &mut index,
                current_mml_velocity,
                current_octave,
                current_tempo,
                &mut is_connect_chord,
            ) {
                match event {
                    MmlEvent::SetNote(note) => self.notes.push(note),
                    MmlEvent::SetTempo(tempo) => current_tempo = tempo,
                    MmlEvent::SetOctave(octave) => current_octave = octave,
                    MmlEvent::IncreOctave => current_octave += 1,
                    MmlEvent::DecreOctave => {
                        if current_octave > 0 {
                            current_octave -= 1;
                        }
                    }
                    MmlEvent::SetVelocity(velocity) => current_mml_velocity = velocity,
                    MmlEvent::ConnectChord => is_connect_chord = true,
                    MmlEvent::Empty => (),
                }
            } else {
                break;
            }
        }
    }

    fn parse_event(
        &self,
        index: &mut usize,
        current_mml_velocity: u8,
        current_mml_octave: u8,
        current_tempo: usize,
        is_connect_chord: &mut bool,
    ) -> Option<MmlEvent> {
        match self.raw_mml.chars().nth(*index) {
            Some(char) => {
                let mml = &self.raw_mml.as_str()[*index..];

                if char == 't' {
                    let value = get_first_mml_value(mml);
                    *index += value.len() + 1;

                    let tempo = value.parse::<usize>().unwrap();

                    Some(MmlEvent::SetTempo(tempo))
                } else if char == 'o' {
                    let value = get_first_mml_value(mml);
                    *index += value.len() + 1;

                    let octave = value.parse::<u8>().unwrap();

                    Some(MmlEvent::SetOctave(octave))
                } else if char == 'v' {
                    let value = get_first_mml_value(mml);
                    *index += value.len() + 1;

                    let velocity = value.parse::<u8>().unwrap();

                    Some(MmlEvent::SetVelocity(velocity))
                } else if char == '>' {
                    *index += 1;

                    Some(MmlEvent::IncreOctave)
                } else if char == '<' {
                    *index += 1;

                    Some(MmlEvent::DecreOctave)
                } else if char == ':' {
                    *index += 1;

                    Some(MmlEvent::ConnectChord)
                } else if NOTE_NAMES.contains(&char) {
                    let mml_note = get_first_mml_note(mml);
                    *index += mml_note.len();

                    let note = NoteEvent::from_mml(
                        mml_note,
                        current_mml_octave,
                        current_mml_velocity,
                        current_tempo,
                        *is_connect_chord,
                        *index,
                    );

                    *is_connect_chord = false;
                    Some(MmlEvent::SetNote(note))
                } else {
                    *index += 1;
                    Some(MmlEvent::Empty)
                }
            },
            None => None
        }
    }
}

fn send_note_on_event_from_note(note_on_tx: &Option<Arc<fn(NoteOnCallbackData)>>, note: &NoteEvent) {
    if let Some(callback) = note_on_tx {
        callback(NoteOnCallbackData {
            char_index: note.char_index,
            char_length: note.char_length,
        });
    }
}

fn send_note_on_event_from_chord(note_on_callback: &Option<Arc<fn(NoteOnCallbackData)>>, chord: &Vec<NoteEvent>) {
    if let Some(callback) = note_on_callback {
        let first_note = chord.first().unwrap();
        let char_index = first_note.char_index;
        let mut char_length = first_note.char_length;

        for note in chord[1..].iter() {
            char_length += note.char_length;
        }

        callback(NoteOnCallbackData {
            char_index,
            char_length,
        });
    }
}

fn get_first_mml_note(mml: &str) -> String {
    let mut chars = mml.chars();
    let mut result = String::new();
    let mut is_note_extra_checked = false;
    let mut before_char = chars.next().unwrap();
    let note_name = before_char;
    let to_match = ['&', '.', '+'];

    result.push(note_name);

    while let Some(char) = chars.next() {
        if is_note_extra_checked == false {
            if char == '+' {
                result.push(char);
                continue;
            }

            is_note_extra_checked = true;
        }

        let mut is_break = true;

        if char.is_digit(10) || to_match.contains(&char) {
            is_break = false;
        }

        if char == note_name && before_char == '&' {
            is_break = false;
        }

        if is_break {
            break;
        } else {
            before_char = char;
            result.push(char);
        }
    }

    result
}

fn get_first_mml_value(mml: &str) -> String {
    let mut chars = mml[1..].chars();
    let mut result = String::new();

    while let Some(char) = chars.next() {
        if char.is_digit(10) {
            result.push(char);
        } else {
            break;
        }
    }

    result
}
