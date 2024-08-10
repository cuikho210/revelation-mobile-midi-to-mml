use std::{
    sync::{Arc, RwLock}, thread::sleep, time::{Duration, Instant}
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
    pub index: usize,
    pub raw_mml: String,
    pub notes: Vec<NoteEvent>,
    pub instrument: Instrument,
    pub connection: SynthOutputConnection,

    status: Arc<RwLock<PlaybackStatus>>,
    note_on_callback: Option<Arc<fn(NoteOnCallbackData)>>,
    note_before: Option<NoteEvent>,
    time_start: Option<Instant>,
    time_pause: Option<Instant>,
    current_chord: Vec<NoteEvent>,
    absolute_duration: isize,
    current_note_index: usize,
}

impl Parser {
    pub fn parse(
        index: usize,
        mml: String,
        instrument: Instrument,
        connection: SynthOutputConnection,
        playback_status: Arc<RwLock<PlaybackStatus>>,
    ) -> Self {
        let program_id = instrument.instrument_id;
        let channel = instrument.midi_channel;

        let mut result = Self {
            index,
            raw_mml: mml,
            notes: Vec::new(),
            instrument,
            connection,
            status: playback_status,
            note_on_callback: None,
            note_before: None,
            current_chord: Vec::new(),
            time_start: None,
            time_pause: None,
            absolute_duration: 0,
            current_note_index: 0,
        };

        result.parse_note_events();
        result.connection.program_change(channel, program_id);
        result
    }

    pub fn play(&mut self, note_on_callback: Option<Arc<fn(NoteOnCallbackData)>>) {
        self.note_on_callback = note_on_callback;

        if let Some(time_pause) = self.time_pause {
            let time_start = self.time_start.unwrap();

            let now = Instant::now();
            let diff = now - time_pause;
            let new_time = time_start + diff;
            println!("New time to now: {}", new_time.elapsed().as_secs());
            self.time_start = Some(new_time);
        } else {
            self.time_start = Some(Instant::now());
        }

        self.play_notes_linear();
    }
    
    pub fn pause(&mut self) {
        self.time_pause = Some(Instant::now());
    }

    pub fn reset_state(&mut self) {
        self.absolute_duration = 0;
        self.note_before = None;
        self.current_chord = Vec::new();
        self.time_start = None;
        self.time_pause = None;
    }

    fn handle_playback_status(&mut self) -> PlaybackStatus {
        let playback_status = self.status.clone();

        if let Ok(guard) = playback_status.read() {
            if *guard != PlaybackStatus::PLAY {
                if *guard == PlaybackStatus::PAUSE {
                    self.pause();
                    println!("[parser.play_notes_linear] Paused");

                    return PlaybackStatus::PAUSE;

                } else {
                    self.reset_state();
                    println!("[parser.play_notes_linear] Reset state");

                    return PlaybackStatus::STOP;
                }
            }
        }

        PlaybackStatus::PLAY
    }

    fn handle_note_blocking(&mut self, duration: Duration) {
        let time_start = Instant::now();
        let time_loop = Duration::from_millis(32);

        loop {
            let elapsed = time_start.elapsed();
            if elapsed >= duration {
                break;
            }

            let remaining = duration - elapsed;
            let sleep_duration = if remaining > time_loop { time_loop } else { remaining };

            let status = self.handle_playback_status();
            if status != PlaybackStatus::PLAY {
                break;
            }

            sleep(sleep_duration);
        }
    }

    fn play_notes_linear(&mut self) {
        let connection = self.connection.clone();
        let time = self.time_start.unwrap();

        for index in self.current_note_index..self.notes.len() {
            if self.handle_playback_status() != PlaybackStatus::PLAY {
                return;
            }

            let note = self.notes.get(index).unwrap().to_owned();

            if note.is_connected_to_prev_note {
                if let Some(before_note) = self.note_before.as_ref() {
                    if self.current_chord.len() == 0 {
                        self.current_chord.push(before_note.to_owned());
                    }
                }

                self.current_chord.push(note.to_owned());
                continue;
            }

            let correct_duration = time.elapsed().as_millis() as isize;
            let duration_diff = correct_duration - self.absolute_duration;

            if self.current_chord.len() > 0 {
                let chord_duration = utils::get_longest_note_duration(&self.current_chord);
                let duration = chord_duration - duration_diff;
                let duration = Duration::from_millis(duration as u64);

                send_note_on_event_from_chord(&self.note_on_callback, &self.current_chord, self.index);

                let sleep_duration = utils::play_chord(
                    connection.to_owned(),
                    &self.current_chord,
                    self.instrument.midi_channel,
                    Some(duration),
                );

                self.handle_note_blocking(sleep_duration);

                utils::stop_chord(
                    connection.to_owned(),
                    &self.current_chord,
                    self.instrument.midi_channel,
                );

                self.absolute_duration += chord_duration;
                self.current_chord.clear();
                self.note_before = Some(note.to_owned());

                continue;
            }

            if let Some(before_note) = self.note_before.as_ref() {
                let note_duration = before_note.duration_in_ms as isize;
                let duration = note_duration - duration_diff;
                let duration = Duration::from_millis(duration as u64);

                send_note_on_event_from_note(&self.note_on_callback, before_note, self.index);

                let sleep_duration = utils::play_note(
                    connection.to_owned(),
                    before_note,
                    self.instrument.midi_channel,
                    Some(duration),
                );

                let before_note = before_note.to_owned();
                self.handle_note_blocking(sleep_duration);

                utils::stop_note(
                    connection.to_owned(),
                    &before_note,
                    self.instrument.midi_channel,
                );

                self.absolute_duration += note_duration;
            }

            self.note_before = Some(note.to_owned());
            continue;
        }

        if self.current_chord.len() > 0 {
            send_note_on_event_from_chord(&self.note_on_callback, &self.current_chord, self.index);

            let sleep_duration = utils::play_chord(
                connection.to_owned(),
                &self.current_chord,
                self.instrument.midi_channel,
                None,
            );

            self.handle_note_blocking(sleep_duration);

            utils::stop_chord(
                connection.to_owned(),
                &self.current_chord,
                self.instrument.midi_channel,
            );
        }

        if let Some(before_note) = self.note_before.as_ref() {
            send_note_on_event_from_note(&self.note_on_callback, &before_note, self.index);

            let sleep_duration = utils::play_note(
                connection.to_owned(),
                &before_note,
                self.instrument.midi_channel,
                None,
            );

            let before_note = before_note.to_owned();
            self.handle_note_blocking(sleep_duration);

            utils::stop_note(
                connection.to_owned(),
                &before_note,
                self.instrument.midi_channel,
            );
        }

        self.reset_state();
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
                    let mml_note_length = mml_note.len();

                    let note = NoteEvent::from_mml(
                        mml_note,
                        current_mml_octave,
                        current_mml_velocity,
                        current_tempo,
                        *is_connect_chord,
                        *index,
                    );

                    *is_connect_chord = false;
                    *index += mml_note_length;

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

fn send_note_on_event_from_note(note_on_tx: &Option<Arc<fn(NoteOnCallbackData)>>, note: &NoteEvent, track_index: usize) {
    if let Some(callback) = note_on_tx {
        callback(NoteOnCallbackData {
            track_index,
            char_index: note.char_index,
            char_length: note.char_length,
        });
    }
}

fn send_note_on_event_from_chord(
    note_on_callback: &Option<Arc<fn(NoteOnCallbackData)>>,
    chord: &Vec<NoteEvent>,
    track_index: usize,
) {
    if let Some(callback) = note_on_callback {
        let first_note = chord.first().unwrap();
        let char_index = first_note.char_index;
        let char_length = first_note.char_length;

        callback(NoteOnCallbackData {
            track_index,
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

        if is_break && (char.is_digit(10) || to_match.contains(&char)) {
            is_break = false;
        }

        if is_break && (char == note_name && before_char == '&') {
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
