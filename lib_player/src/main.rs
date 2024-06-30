use std::{
    fs::File,
    sync::Arc,
};
use rustysynth::{
    SoundFont,
    Synthesizer,
    SynthesizerSettings,
};
use rodio::{
    OutputStream,
    Sink,
    buffer::SamplesBuffer,
};

const SAMPLE_RATE: u32 = 44100;

fn play_waveform(left: Vec<f32>, right: Vec<f32>) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let mut waveform = Vec::with_capacity(left.len() * 2);
    for i in 0..left.len() {
        waveform.push(left[i]);
        waveform.push(right[i]);
    }

    let source = SamplesBuffer::new(1, SAMPLE_RATE, waveform);
    sink.append(source);

    sink.sleep_until_end();
}

fn main() {
    let mut sf2 = File::open("./test_resouces/YDP-GrandPiano-SF2-20160804/YDP-GrandPiano-20160804.sf2").unwrap();
    let sound_font = Arc::new(SoundFont::new(&mut sf2).unwrap());

    let settings = SynthesizerSettings::new(SAMPLE_RATE as i32);
    let mut synthesizer = Synthesizer::new(&sound_font, &settings).unwrap();

    synthesizer.note_on(1, 60, 100);
    synthesizer.note_on(1, 64, 100);
    synthesizer.note_on(1, 67, 100);

    let sample_count = (3 * settings.sample_rate) as usize;
    let mut left: Vec<f32> = vec![0_f32; sample_count];
    let mut right: Vec<f32> = vec![0_f32; sample_count];

    synthesizer.render(&mut left[..], &mut right[..]);
    play_waveform(left, right);
}
