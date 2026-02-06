use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::Arc;
use crate::synthesizer::Synth;

pub fn start_audio_engine(synth: Arc<Synth>) -> cpal::Stream {

    let host = cpal::default_host();
    let device = host.default_output_device().expect("No output device found");
    let config: cpal::StreamConfig = device.default_output_config().unwrap().into();

    let sample_rate = config.sample_rate as f32;
    let channels = config.channels as usize;

    let stream = device.build_output_stream(
        &config,
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            let mut voices = synth.voices.lock().unwrap();
            if !voices.is_empty() {
                println!("Active voices: {}", voices.len());
            }
            let table = &synth.wave_table;
            let table_len = table.len() as f32;

            for frame in data.chunks_mut(channels) {
                let mut sum = 0.0;

                for voice in voices.iter_mut() {

                    let index = (voice.phase * (table.len() as f32 - 1.0)) as usize;

                    sum += table[index];

                    let phase_increment = voice.freq / sample_rate;
                    voice.phase = (voice.phase + phase_increment) % 1.0;
                }

                let output = if !voices.is_empty() {
                    (sum / voices.len() as f32) * 0.3
                } else {
                    0.0
                };

                for sample in frame.iter_mut() {
                    *sample = output;
                }
            }
        },
        |err| eprintln!("Audio Error: {}", err),
        None
    ).unwrap();
    stream.play().unwrap();
    stream
}