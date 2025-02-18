use std::{fs::File, io::Cursor, path::Path, sync::{mpsc::Receiver, Arc, Mutex}, thread::{self, JoinHandle}};
use cpal::{
    traits::{DeviceTrait, HostTrait},
    FromSample, SizedSample
};
use oxisynth::{MidiEvent, SoundFont};

pub struct OxisynthWrapper {
    pub synth: oxisynth::Synth,
}

impl OxisynthWrapper {
    pub fn from_cpal_config(config: cpal::StreamConfig) -> Self {
        let sample_rate = config.sample_rate.0 as f32;

        let settings = oxisynth::SynthDescriptor {
            sample_rate,
            gain: 1.0,
            ..Default::default()
        };

        let synth = oxisynth::Synth::new(settings).unwrap();

        Self { synth }
    }
    
    pub fn load_soundfont_from_file<P>(&mut self, path: P) -> Result<(), String>
        where P: AsRef<Path>,
    {
        let mut file = File::open(&path).ok().ok_or(
            format!("Cannot open file from path {:?}", path.as_ref())
        )?;

        let font = SoundFont::load(&mut file).ok().ok_or(
            format!("Cannot load soundfont from file {:?}", path.as_ref())
        )?;

        self.synth.add_font(font, false);

        Ok(())
    }

    pub fn load_soundfont_from_bytes<B>(&mut self, bytes: B) -> Result<(), String>
        where B: AsRef<[u8]>,
    {
        let mut cursor = Cursor::new(bytes);

        let font = SoundFont::load(&mut cursor).ok().ok_or(
            "Cannot load soundfont from bytes".to_owned()
        )?;

        self.synth.add_font(font, false);

        Ok(())
    }
}


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
    pub synth: Arc<Mutex<OxisynthWrapper>>,
}

impl Default for Synth {
    fn default() -> Self {
        Self::new()
    }
}

impl Synth {
    pub fn new() -> Self {
        let host = cpal::default_host();
        let device = host.default_output_device().unwrap();
        let config = device.default_output_config().unwrap();

        let synth = Arc::new(Mutex::new(
            OxisynthWrapper::from_cpal_config(config.to_owned().into())
        ));

        Self { host, device, config, synth }
    }

    pub fn new_stream(&self)
        -> (cpal::Stream, SynthOutputConnection)
    {
        let (tx, rx) = std::sync::mpsc::channel::<MidiEvent>();

        let stream = match self.config.sample_format() {
            cpal::SampleFormat::I8 => self.make_stream::<i8>(rx),
            cpal::SampleFormat::I16 => self.make_stream::<i16>(rx),
            cpal::SampleFormat::I32 => self.make_stream::<i32>(rx),
            cpal::SampleFormat::I64 => self.make_stream::<i64>(rx),
            cpal::SampleFormat::U8 => self.make_stream::<u8>(rx),
            cpal::SampleFormat::U16 => self.make_stream::<u16>(rx),
            cpal::SampleFormat::U32 => self.make_stream::<u32>(rx),
            cpal::SampleFormat::U64 => self.make_stream::<u64>(rx),
            cpal::SampleFormat::F32 => self.make_stream::<f32>(rx),
            cpal::SampleFormat::F64 => self.make_stream::<f64>(rx),
            _ => panic!("[Synth.new_stream] Unsupported format")
        };

        (stream, SynthOutputConnection { tx })
    }

    pub fn load_soundfont_from_file<P>(&mut self, path: P) -> Result<(), String>
        where P: AsRef<Path>,
    {
        if let Ok(mut synth) = self.synth.lock() {
            synth.load_soundfont_from_file(path)?;

            Ok(())
        } else {
            Err("[Synth.load_soundfont_from_file] Cannot lock synth".to_owned())
        }
    }

    pub fn load_soundfont_from_file_parallel<P>(&mut self, paths: Vec<P>) -> Result<(), String>
        where P: AsRef<Path> + Sync + Send + Clone + 'static,
    {
        let handles: Vec<JoinHandle<()>> = paths.iter().map::<JoinHandle<()>, _>(|path| {
            let synth = self.synth.clone();
            let path = path.to_owned();

            thread::spawn(move || {
                if let Ok(mut synth_guard) = synth.lock() {
                    synth_guard.load_soundfont_from_file(path).unwrap();
                }
            })
        }).collect();

        for handle in handles {
            handle.join().ok().ok_or(
                "[Synth.load_soundfont_from_file_parallel] Cannot join thread".to_owned()
            )?;
        }

        Ok(())
    }

    pub fn load_soundfont_from_bytes<B>(&mut self, bytes: B) -> Result<(), String>
        where B: AsRef<[u8]>,
    {
        if let Ok(mut synth) = self.synth.lock() {
            synth.load_soundfont_from_bytes(bytes)?;

            Ok(())
        } else {
            Err("[Synth.load_soundfont_from_file] Cannot lock synth".to_owned())
        }
    }

    pub fn load_soundfont_from_bytes_parallel<B>(&mut self, list_bytes: Vec<B>) -> Result<(), String>
        where B: AsRef<[u8]> + Sync + Send + Clone + 'static,
    {
        let handles: Vec<JoinHandle<()>> = list_bytes.iter().map::<JoinHandle<()>, _>(|bytes| {
            let synth = self.synth.clone();
            let bytes = bytes.to_owned();

            thread::spawn(move || {
                if let Ok(mut synth_guard) = synth.lock() {
                    synth_guard.load_soundfont_from_bytes(bytes).unwrap();
                }
            })
        }).collect();

        for handle in handles {
            handle.join().ok().ok_or(
                "[Synth.load_soundfont_from_bytes_parallel] Cannot join thread".to_owned()
            )?;
        }

        Ok(())
    }

    fn make_stream<T>(&self, rx: Receiver<MidiEvent>) -> cpal::Stream
    where
        T: SizedSample + FromSample<f32>,
    {
        let config: cpal::StreamConfig = self.config.to_owned().into();
        let synth = self.synth.clone();

        let next_value = move || {
            if let Ok(mut synth) = synth.lock() {
                let (l, r) = synth.synth.read_next();

                if let Ok(e) = rx.try_recv() {
                    synth.synth.send_event(e).ok();
                }

                (l, r)
            } else {
                (0.0, 0.0)
            }
        };

        let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

        let channels = config.channels as usize;

        

        self
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
            ).unwrap()
    }
}

