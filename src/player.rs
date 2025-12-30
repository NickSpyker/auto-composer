/*
 * Copyright 2025 Nicolas Spijkerman
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use crate::{Error, Result, SoundFont};
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait}, Device, FromSample, OutputCallbackInfo, SampleFormat, SizedSample, Stream,
    StreamConfig,
};
use midly::{Format, Fps, MetaMessage, MidiMessage, Smf, Timing, TrackEventKind};
use rustysynth::{MidiFile, MidiFileSequencer, Synthesizer, SynthesizerSettings};
use std::{
    io::Cursor,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

#[derive(Debug)]
pub struct Player {
    smf: Smf<'static>,
    soundfont: SoundFont,
}

impl Player {
    pub fn new(smf: Smf<'static>, soundfont: SoundFont) -> Result<Self> {
        Ok(Self { smf, soundfont })
    }

    pub fn run(&self) -> Result<()> {
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .ok_or_else(|| Error::AudioPlayback("No output device available".to_string()))?;

        let config = device.default_output_config().map_err(|err| {
            Error::AudioPlayback(format!("Failed to get default output config: {err}"))
        })?;

        let sample_rate = config.sample_rate();

        let mut cursor = Cursor::new(self.soundfont.get_bytes());
        let soundfont = Arc::new(
            rustysynth::SoundFont::new(&mut cursor)
                .map_err(|err| Error::AudioPlayback(format!("Failed to load soundfont: {err}")))?,
        );

        let settings = SynthesizerSettings::new(sample_rate as i32);
        let synthesizer = Synthesizer::new(&soundfont, &settings)
            .map_err(|err| Error::AudioPlayback(format!("Failed to create synthesizer: {err}")))?;

        let midi_data = self.convert_midly_to_bytes()?;
        let midi_file = Arc::new(
            MidiFile::new(&mut Cursor::new(midi_data))
                .map_err(|err| Error::AudioPlayback(format!("Failed to parse MIDI: {err}")))?,
        );

        let mut sequencer = MidiFileSequencer::new(synthesizer);
        sequencer.play(&midi_file, false);

        let sequencer = Arc::new(Mutex::new(sequencer));
        let sequencer_clone = sequencer.clone();

        let stream = match config.sample_format() {
            SampleFormat::F32 => {
                self.build_stream::<f32>(&device, &config.into(), sequencer_clone)?
            }
            SampleFormat::I16 => {
                self.build_stream::<i16>(&device, &config.into(), sequencer_clone)?
            }
            SampleFormat::U16 => {
                self.build_stream::<u16>(&device, &config.into(), sequencer_clone)?
            }
            _ => {
                return Err(Error::AudioPlayback(
                    "Unsupported sample format".to_string(),
                ));
            }
        };

        stream
            .play()
            .map_err(|err| Error::AudioPlayback(format!("Failed to play stream: {err}")))?;

        while !sequencer.lock().unwrap().end_of_sequence() {
            thread::sleep(Duration::from_millis(100));
        }

        thread::sleep(Duration::from_millis(500));

        Ok(())
    }

    fn build_stream<T: SizedSample + FromSample<f32>>(
        &self,
        device: &Device,
        config: &StreamConfig,
        sequencer: Arc<Mutex<MidiFileSequencer>>,
    ) -> Result<Stream> {
        let channels = config.channels as usize;
        let mut left = vec![0_f32; 4096];
        let mut right = vec![0_f32; 4096];

        let stream = device
            .build_output_stream(
                config,
                move |data: &mut [T], _: &OutputCallbackInfo| {
                    let mut sequencer = sequencer.lock().unwrap();

                    for chunk in data.chunks_mut(channels) {
                        if left.is_empty() {
                            left.resize(4096, 0_f32);
                            right.resize(4096, 0_f32);
                            sequencer.render(&mut left, &mut right);
                        }

                        let left_sample = left.remove(0);
                        let right_sample = right.remove(0);

                        if channels == 1 {
                            chunk[0] = T::from_sample((left_sample + right_sample) * 0.5);
                        } else {
                            chunk[0] = T::from_sample(left_sample);
                            if chunk.len() > 1 {
                                chunk[1] = T::from_sample(right_sample);
                            }
                        }
                    }
                },
                |err| eprintln!("Stream error: {err}"),
                None,
            )
            .map_err(|err| Error::AudioPlayback(format!("Failed to build output stream: {err}")))?;

        Ok(stream)
    }

    fn convert_midly_to_bytes(&self) -> Result<Vec<u8>> {
        let mut output = Vec::new();

        output.extend_from_slice(b"MThd");
        output.extend_from_slice(&6_u32.to_be_bytes());

        let format = match self.smf.header.format {
            Format::SingleTrack => 0_u16,
            Format::Parallel => 1_u16,
            Format::Sequential => 2_u16,
        };
        output.extend_from_slice(&format.to_be_bytes());

        let num_tracks = self.smf.tracks.len() as u16;
        output.extend_from_slice(&num_tracks.to_be_bytes());

        let timing = match self.smf.header.timing {
            Timing::Metrical(ticks) => ticks.as_int(),
            Timing::Timecode(fps, ticks) => {
                let fps_val = match fps {
                    Fps::Fps24 => -24_i8,
                    Fps::Fps25 => -25_i8,
                    Fps::Fps29 => -29_i8,
                    Fps::Fps30 => -30_i8,
                };
                ((fps_val as u8 as u16) << 8) | (ticks as u16)
            }
        };
        output.extend_from_slice(&timing.to_be_bytes());

        for track in &self.smf.tracks {
            output.extend_from_slice(b"MTrk");

            let mut track_data = Vec::new();
            let mut running_status: Option<u8> = None;

            for event in track {
                self.write_variable_length(&mut track_data, event.delta.as_int());

                match event.kind {
                    TrackEventKind::Midi { channel, message } => {
                        let channel = channel.as_int();
                        match message {
                            MidiMessage::NoteOff { key, vel } => {
                                let status = 0x80 | channel;
                                if running_status != Some(status) {
                                    track_data.push(status);
                                    running_status = Some(status);
                                }
                                track_data.push(key.as_int());
                                track_data.push(vel.as_int());
                            }
                            MidiMessage::NoteOn { key, vel } => {
                                let status = 0x90 | channel;
                                if running_status != Some(status) {
                                    track_data.push(status);
                                    running_status = Some(status);
                                }
                                track_data.push(key.as_int());
                                track_data.push(vel.as_int());
                            }
                            MidiMessage::Aftertouch { key, vel } => {
                                let status = 0xA0 | channel;
                                if running_status != Some(status) {
                                    track_data.push(status);
                                    running_status = Some(status);
                                }
                                track_data.push(key.as_int());
                                track_data.push(vel.as_int());
                            }
                            MidiMessage::Controller { controller, value } => {
                                let status = 0xB0 | channel;
                                if running_status != Some(status) {
                                    track_data.push(status);
                                    running_status = Some(status);
                                }
                                track_data.push(controller.as_int());
                                track_data.push(value.as_int());
                            }
                            MidiMessage::ProgramChange { program } => {
                                let status = 0xC0 | channel;
                                if running_status != Some(status) {
                                    track_data.push(status);
                                    running_status = Some(status);
                                }
                                track_data.push(program.as_int());
                            }
                            MidiMessage::ChannelAftertouch { vel } => {
                                let status = 0xD0 | channel;
                                if running_status != Some(status) {
                                    track_data.push(status);
                                    running_status = Some(status);
                                }
                                track_data.push(vel.as_int());
                            }
                            MidiMessage::PitchBend { bend } => {
                                let status = 0xE0 | channel;
                                if running_status != Some(status) {
                                    track_data.push(status);
                                    running_status = Some(status);
                                }
                                let value = (bend.as_int() + 8192) as u16;
                                track_data.push((value & 0x7F) as u8);
                                track_data.push(((value >> 7) & 0x7F) as u8);
                            }
                        }
                    }
                    TrackEventKind::Meta(msg) => {
                        running_status = None;
                        track_data.push(0xFF);
                        match msg {
                            MetaMessage::Tempo(tempo) => {
                                track_data.push(0x51);
                                track_data.push(0x03);
                                let bytes = tempo.as_int().to_be_bytes();
                                track_data.extend_from_slice(&bytes[1..4]);
                            }
                            MetaMessage::EndOfTrack => {
                                track_data.push(0x2F);
                                track_data.push(0x00);
                            }
                            MetaMessage::TimeSignature(num, denom, clocks, thirty_seconds) => {
                                track_data.push(0x58);
                                track_data.push(0x04);
                                track_data.push(num);
                                track_data.push(denom);
                                track_data.push(clocks);
                                track_data.push(thirty_seconds);
                            }
                            MetaMessage::KeySignature(key, scale) => {
                                track_data.push(0x59);
                                track_data.push(0x02);
                                track_data.push(key as u8);
                                track_data.push(if scale { 0 } else { 1 });
                            }
                            MetaMessage::TrackName(name) => {
                                track_data.push(0x03);
                                self.write_variable_length(&mut track_data, name.len() as u32);
                                track_data.extend_from_slice(name);
                            }
                            MetaMessage::Text(text) | MetaMessage::Copyright(text) => {
                                track_data.push(0x01);
                                self.write_variable_length(&mut track_data, text.len() as u32);
                                track_data.extend_from_slice(text);
                            }
                            _ => {}
                        }
                    }
                    TrackEventKind::SysEx(data) => {
                        running_status = None;
                        track_data.push(0xF0);
                        self.write_variable_length(&mut track_data, data.len() as u32);
                        track_data.extend_from_slice(data);
                    }
                    TrackEventKind::Escape(data) => {
                        running_status = None;
                        track_data.push(0xF7);
                        self.write_variable_length(&mut track_data, data.len() as u32);
                        track_data.extend_from_slice(data);
                    }
                }
            }

            output.extend_from_slice(&(track_data.len() as u32).to_be_bytes());
            output.extend_from_slice(&track_data);
        }

        Ok(output)
    }

    fn write_variable_length(&self, output: &mut Vec<u8>, mut value: u32) {
        let mut buffer = [0u8; 4];
        let mut n = 0;

        buffer[n] = (value & 0x7F) as u8;
        value >>= 7;

        while value > 0 {
            n += 1;
            buffer[n] = ((value & 0x7F) | 0x80) as u8;
            value >>= 7;
        }

        while n > 0 {
            output.push(buffer[n]);
            n -= 1;
        }

        output.push(buffer[0]);
    }
}
