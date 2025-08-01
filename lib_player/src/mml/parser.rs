use crate::NoteEvent;
use anyhow::{Context, Result};

use super::event::MmlEvent;

const NOTE_NAMES: [char; 8] = ['c', 'd', 'e', 'f', 'g', 'a', 'b', 'r'];
const NOTE_EXTRAS: [char; 3] = ['&', '.', '+'];

#[derive(Debug, Clone)]
pub struct Parser {
    pub index: usize,
    pub raw_mml: String,
    pub notes: Vec<NoteEvent>,
}

impl Parser {
    pub fn parse(index: usize, mml: String) -> Result<Self> {
        let notes = parse_note_events(&mml)?;
        Ok(Self {
            index,
            raw_mml: mml,
            notes,
        })
    }
}

fn parse_note_events(mml: &str) -> Result<Vec<NoteEvent>> {
    let mut index = 0;
    let mut current_mml_velocity = 12u8;
    let mut current_octave = 4u8;
    let mut current_tempo = 120usize;
    let mut is_connect_chord = false;
    let mut notes: Vec<NoteEvent> = Vec::with_capacity(mml.len() / 2);

    while let Some(event) = parse_event(
        mml,
        &mut index,
        current_mml_velocity,
        current_octave,
        current_tempo,
        &mut is_connect_chord,
    )? {
        match event {
            MmlEvent::SetNote(note) => notes.push(note),
            MmlEvent::SetTempo(tempo) => current_tempo = tempo,
            MmlEvent::SetOctave(octave) => current_octave = octave,
            MmlEvent::IncreOctave => current_octave += 1,
            MmlEvent::DecreOctave => {
                current_octave = current_octave.saturating_sub(1);
            }
            MmlEvent::SetVelocity(velocity) => current_mml_velocity = velocity,
            MmlEvent::ConnectChord => is_connect_chord = true,
            MmlEvent::Empty => (),
        }
    }

    Ok(notes)
}

fn parse_event(
    raw_mml: &str,
    index: &mut usize,
    current_mml_velocity: u8,
    current_mml_octave: u8,
    current_tempo: usize,
    is_connect_chord: &mut bool,
) -> Result<Option<MmlEvent>> {
    let char = match raw_mml.as_bytes().get(*index) {
        Some(&c) => c as char,
        None => return Ok(None),
    };

    match char {
        't' => {
            let (value, len) = get_first_mml_value(&raw_mml[*index..]);
            *index += len + 1;
            let tempo = value.parse::<usize>().context("Invalid tempo value")?;
            Ok(Some(MmlEvent::SetTempo(tempo)))
        }
        'o' => {
            let value = raw_mml
                .get(*index + 1..*index + 2)
                .context("Missing octave value")?;
            *index += 2;
            let octave = value.parse::<u8>().context("Invalid octave value")?;
            Ok(Some(MmlEvent::SetOctave(octave)))
        }
        'v' => {
            let (value, len) = get_first_mml_value(&raw_mml[*index..]);
            *index += len + 1;
            let velocity = value.parse::<u8>().context("Invalid velocity value")?;
            Ok(Some(MmlEvent::SetVelocity(velocity)))
        }
        '>' => {
            *index += 1;
            Ok(Some(MmlEvent::IncreOctave))
        }
        '<' => {
            *index += 1;
            Ok(Some(MmlEvent::DecreOctave))
        }
        ':' => {
            *index += 1;
            Ok(Some(MmlEvent::ConnectChord))
        }
        c if NOTE_NAMES.contains(&c) => {
            let (mml_note, len) = get_first_mml_note(&raw_mml[*index..])?;
            let note = NoteEvent::from_mml(
                mml_note.to_string(),
                current_mml_octave,
                current_mml_velocity,
                current_tempo,
                *is_connect_chord,
                *index,
            )?;
            *is_connect_chord = false;
            *index += len;
            Ok(Some(MmlEvent::SetNote(note)))
        }
        _ => {
            *index += 1;
            Ok(Some(MmlEvent::Empty))
        }
    }
}

fn get_first_mml_note(mml: &str) -> Result<(&str, usize)> {
    let mut len = 1;
    let mut is_note_extra_checked = false;
    let mut before_char = mml.chars().next().context("Empty MML string")?;
    let note_name = before_char;

    for (i, char) in mml.chars().enumerate().skip(1) {
        if !is_note_extra_checked {
            if char == '+' {
                len += 1;
                continue;
            }
            is_note_extra_checked = true;
        }

        let is_break = !(char.is_ascii_digit()
            || NOTE_EXTRAS.contains(&char)
            || (char == note_name && before_char == '&'));

        if is_break {
            return Ok((mml.get(..i).context("Invalid slice")?, i));
        }
        before_char = char;
        len += 1;
    }

    Ok((mml.get(..len).context("Invalid slice")?, len))
}

fn get_first_mml_value(mml: &str) -> (&str, usize) {
    let mut len = 0;
    for (i, char) in mml.chars().skip(1).enumerate() {
        if !char.is_ascii_digit() {
            return (mml.get(1..=i).unwrap_or(""), i);
        }
        len = i + 1;
    }
    (mml.get(1..=len).unwrap_or(""), len)
}
