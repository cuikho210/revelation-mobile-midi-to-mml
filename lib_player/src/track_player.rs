use crate::{
    Parser, PlaybackStatus,
    SynthOutputConnection,
    NoteOnCallbackData,
    NoteEvent,
    utils,
};
use std::{
    sync::{Arc, RwLock},
    time::{Instant, Duration},
    thread::sleep,
};
use revelation_mobile_midi_to_mml::Instrument;

pub struct TrackPlayer {
    pub index: usize,
    pub parser: Parser,
    pub playback_status: Arc<RwLock<PlaybackStatus>>,
    pub instrument: Instrument,
    pub connection: SynthOutputConnection,

    note_before: Option<NoteEvent>,
    time_start: Option<Instant>,
    time_pause: Option<Instant>,
    current_chord: Vec<NoteEvent>,
    absolute_duration: isize,
    current_note_index: usize,
    note_on_callback: Option<Arc<fn(NoteOnCallbackData)>>,
}

impl TrackPlayer {
    pub fn from_parser(
        index: usize,
        parser: Parser,
        playback_status: Arc<RwLock<PlaybackStatus>>,
        instrument: Instrument,
        mut connection: SynthOutputConnection,
    ) -> Self {

        connection.program_change(
            instrument.midi_channel,
            instrument.instrument_id,
        );

        Self {
            index,
            parser,
            playback_status,
            instrument,
            connection,
            note_before: None,
            time_start: None,
            time_pause: None,
            current_chord: Vec::new(),
            absolute_duration: 0,
            current_note_index: 0,
            note_on_callback: None,
        }
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
        let playback_status = self.playback_status.clone();

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

        for index in self.current_note_index..self.parser.notes.len() {
            if self.handle_playback_status() != PlaybackStatus::PLAY {
                return;
            }

            let note = self.parser.notes.get(index).unwrap().to_owned();

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

