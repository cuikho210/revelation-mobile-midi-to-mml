use std::{path::PathBuf, sync::mpsc::Receiver, thread::{self, JoinHandle}};

use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait}, FromSample, SizedSample
};
use oxisynth::{MidiEvent, SoundFont};

#[derive(Clone, Debug)]
pub struct SynthOutputConnection {
    pub tx: std::sync::mpsc::Sender<MidiEvent>,
}

impl SynthOutputConnection {
    pub fn note_on(&mut self, channel: u8, key: u8, vel: u8) {
        self.tx.send(MidiEvent::NoteOn { channel, key, vel }).unwrap();
    }
    
    pub fn note_off(&mut self, channel: u8, key: u8) {
        self.tx.send(MidiEvent::NoteOff { channel, key }).unwrap();
    }

    pub fn program_change(&mut self, channel: u8, program_id: u8) {
        self.tx.send(MidiEvent::ProgramChange { channel, program_id }).unwrap();
    }

    pub fn all_notes_off(&mut self, channel: u8) {
        self.tx.send(MidiEvent::AllNotesOff { channel }).unwrap();
    }
}

pub struct Synth {
    pub host: cpal::Host,
    pub device: cpal::Device,
    pub config: cpal::SupportedStreamConfig,
}

impl Synth {
    pub fn new() -> Self {
        let host = cpal::default_host();
        let device = host.default_output_device().unwrap();
        let config = device.default_output_config().unwrap();

        Self { host, device, config }
    }

    pub fn new_stream(&self, soundfont_path: Vec<PathBuf>)
        -> (cpal::Stream, SynthOutputConnection)
    {
        let (tx, rx) = std::sync::mpsc::channel::<MidiEvent>();

        let stream = match self.config.sample_format() {
            cpal::SampleFormat::I8 => self.make_stream::<i8>(rx, soundfont_path),
            cpal::SampleFormat::I16 => self.make_stream::<i16>(rx, soundfont_path),
            cpal::SampleFormat::I32 => self.make_stream::<i32>(rx, soundfont_path),
            cpal::SampleFormat::I64 => self.make_stream::<i64>(rx, soundfont_path),
            cpal::SampleFormat::U8 => self.make_stream::<u8>(rx, soundfont_path),
            cpal::SampleFormat::U16 => self.make_stream::<u16>(rx, soundfont_path),
            cpal::SampleFormat::U32 => self.make_stream::<u32>(rx, soundfont_path),
            cpal::SampleFormat::U64 => self.make_stream::<u64>(rx, soundfont_path),
            cpal::SampleFormat::F32 => self.make_stream::<f32>(rx, soundfont_path),
            cpal::SampleFormat::F64 => self.make_stream::<f64>(rx, soundfont_path),
            _ => panic!("[Synth.new_stream] Unsupported format")
        };

        (stream, SynthOutputConnection { tx })
    }
    
    fn make_stream<T>(&self, rx: Receiver<MidiEvent>, soundfont_path: Vec<PathBuf>) -> cpal::Stream
    where
        T: SizedSample + FromSample<f32>,
    {
        let config: cpal::StreamConfig = self.config.to_owned().into();

        let mut synth = {
            let sample_rate = config.sample_rate.0 as f32;

            let settings = oxisynth::SynthDescriptor {
                sample_rate,
                gain: 1.0,
                ..Default::default()
            };

            let mut synth = oxisynth::Synth::new(settings).unwrap();
            let mut handles: Vec<JoinHandle<SoundFont>> = Vec::new();

            for path in soundfont_path {
                let handle = thread::spawn(move || {
                    let mut file = std::fs::File::open(path).unwrap();
                    let font = oxisynth::SoundFont::load(&mut file).unwrap();
                    font
                });
                handles.push(handle);
            }

            for handle in handles {
                let font = handle.join().unwrap();
                synth.add_font(font, false);
            }

            synth
        };

        let mut next_value = move || {
            let (l, r) = synth.read_next();

            if let Ok(e) = rx.try_recv() {
                synth.send_event(e).ok();
            }

            (l, r)
        };

        let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

        let channels = config.channels as usize;

        let stream = self
            .device
            .build_output_stream(
                &self.config.to_owned().into(),
                move |output: &mut [T], _: &cpal::OutputCallbackInfo| {
                    for frame in output.chunks_mut(channels) {
                        let (l, r) = next_value();

                        let l: T = cpal::Sample::from_sample::<f32>(l);
                        let r: T = cpal::Sample::from_sample::<f32>(r);

                        let channels = [l, r];

                        for (id, sample) in frame.iter_mut().enumerate() {
                            *sample = channels[id % 2];
                        }
                    }
                },
                err_fn,
                None,
            ).unwrap();

        stream.play().unwrap();
        stream
    }
}
