use anyhow::{anyhow, Context, Result};
use cpal::{
    traits::{DeviceTrait, HostTrait},
    FromSample, SizedSample,
};
use oxisynth::{MidiEvent, SoundFont};
use std::{
    fs::File,
    io::Cursor,
    path::Path,
    sync::{mpsc::Receiver, Arc, Mutex},
    thread::{self, JoinHandle},
};

pub struct OxisynthWrapper {
    pub synth: oxisynth::Synth,
}
impl OxisynthWrapper {
    pub fn from_cpal_config(config: cpal::StreamConfig) -> Result<Self> {
        let sample_rate = config.sample_rate.0 as f32;

        let settings = oxisynth::SynthDescriptor {
            sample_rate,
            gain: 1.0,
            ..Default::default()
        };

        let synth = oxisynth::Synth::new(settings)
            .map_err(|e| anyhow!("Failed to create synthesizer: {:?}", e))?;

        Ok(Self { synth })
    }

    pub fn load_soundfont_from_file<P>(&mut self, path: P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        let mut file =
            File::open(&path).context(format!("Cannot open file from path {:?}", path.as_ref()))?;

        let font = SoundFont::load(&mut file)
            .map_err(|_| anyhow!("Cannot load soundfont from file {:?}", path.as_ref()))?;

        self.synth.add_font(font, false);

        Ok(())
    }

    pub fn load_soundfont_from_bytes<B>(&mut self, bytes: B) -> Result<()>
    where
        B: AsRef<[u8]>,
    {
        let mut cursor = Cursor::new(bytes);

        let font = SoundFont::load(&mut cursor)
            .map_err(|_| anyhow!("Cannot load soundfont from bytes"))?;

        self.synth.add_font(font, false);

        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct SynthOutputConnection {
    pub tx: std::sync::mpsc::Sender<MidiEvent>,
}
impl SynthOutputConnection {
    pub fn note_on(&mut self, channel: u8, key: u8, vel: u8) -> Result<()> {
        Ok(self.tx.send(MidiEvent::NoteOn { channel, key, vel })?)
    }

    pub fn note_off(&mut self, channel: u8, key: u8) -> Result<()> {
        Ok(self.tx.send(MidiEvent::NoteOff { channel, key })?)
    }

    pub fn program_change(&mut self, channel: u8, program_id: u8) -> Result<()> {
        Ok(self.tx.send(MidiEvent::ProgramChange {
            channel,
            program_id,
        })?)
    }

    pub fn all_notes_off(&mut self, channel: u8) -> Result<()> {
        Ok(self.tx.send(MidiEvent::AllNotesOff { channel })?)
    }
}

pub struct Synth {
    pub host: cpal::Host,
    pub device: cpal::Device,
    pub config: cpal::SupportedStreamConfig,
    pub synth: Arc<Mutex<OxisynthWrapper>>,
}
impl Synth {
    pub fn new() -> Result<Self> {
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .context("No output device found")?;
        let config = device.default_output_config()?;

        let synth = Arc::new(Mutex::new(OxisynthWrapper::from_cpal_config(
            config.to_owned().into(),
        )?));

        Ok(Self {
            host,
            device,
            config,
            synth,
        })
    }

    pub fn new_stream(&self) -> Result<(cpal::Stream, SynthOutputConnection)> {
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
            _ => panic!("[Synth.new_stream] Unsupported format"),
        }?;

        Ok((stream, SynthOutputConnection { tx }))
    }

    pub fn load_soundfont_from_file<P>(&mut self, path: P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        let mut synth = self
            .synth
            .lock()
            .map_err(|e| anyhow!("Cannot lock synth: {}", e))?;
        synth.load_soundfont_from_file(path)
    }

    pub fn load_soundfont_from_file_parallel<P>(&mut self, paths: Vec<P>) -> Result<()>
    where
        P: AsRef<Path> + Sync + Send + Clone + 'static,
    {
        let handles: Vec<JoinHandle<Result<()>>> = paths
            .iter()
            .map::<JoinHandle<Result<()>>, _>(|path| {
                let synth = self.synth.clone();
                let path = path.to_owned();

                thread::spawn(move || -> Result<()> {
                    let mut synth_guard = synth
                        .lock()
                        .map_err(|e| anyhow::anyhow!("Cannot lock synth: {}", e))?;
                    synth_guard.load_soundfont_from_file(path)
                })
            })
            .collect();

        for handle in handles {
            handle
                .join()
                .map_err(|e| anyhow::anyhow!("Thread join error: {:?}", e))??;
        }

        Ok(())
    }

    pub fn load_soundfont_from_bytes<B>(&mut self, bytes: B) -> Result<()>
    where
        B: AsRef<[u8]>,
    {
        let mut synth = self
            .synth
            .lock()
            .map_err(|e| anyhow::anyhow!("Cannot lock synth: {}", e))?;
        synth.load_soundfont_from_bytes(bytes)
    }

    pub fn load_soundfont_from_bytes_parallel<B>(&mut self, list_bytes: Vec<B>) -> Result<()>
    where
        B: AsRef<[u8]> + Sync + Send + Clone + 'static,
    {
        let handles: Vec<JoinHandle<Result<()>>> = list_bytes
            .iter()
            .map::<JoinHandle<Result<()>>, _>(|bytes| {
                let synth = self.synth.clone();
                let bytes = bytes.to_owned();

                thread::spawn(move || {
                    let mut synth_guard = synth
                        .lock()
                        .map_err(|e| anyhow::anyhow!("Cannot lock synth: {}", e))?;
                    synth_guard.load_soundfont_from_bytes(bytes)
                })
            })
            .collect();

        for handle in handles {
            handle
                .join()
                .map_err(|e| anyhow::anyhow!("Thread join error: {:?}", e))??;
        }

        Ok(())
    }

    fn make_stream<T>(&self, rx: Receiver<MidiEvent>) -> Result<cpal::Stream>
    where
        T: SizedSample + FromSample<f32>,
    {
        let config: cpal::StreamConfig = self.config.to_owned().into();
        let synth = self.synth.clone();

        let next_value = move || {
            if let Ok(mut synth) = synth.lock() {
                let (l, r) = synth.synth.read_next();

                if let Ok(e) = rx.try_recv() {
                    synth
                        .synth
                        .send_event(e)
                        .map_err(|e| eprintln!("Failed to send midi event: {:?}", e))
                        .ok();
                }

                (l, r)
            } else {
                (0.0, 0.0)
            }
        };

        let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

        let channels = config.channels as usize;

        self.device
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
            )
            .context("Cannot build output stream")
    }
}
