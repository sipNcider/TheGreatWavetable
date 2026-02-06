use std::{sync::{Arc, Mutex}};

pub enum WaveType {
    Sine,
    Square,
    Saw,
    Triangle
}

fn make_sine_wave(size: i32) -> Arc<Vec<f32>> {
    let mut table = Vec::with_capacity(size as usize);
    let full_circle = std::f32::consts::TAU;
    for i in 0..size as usize {
        let percent_through = i as f32 / size as f32;
        let value = percent_through * full_circle;
        table.push(value.sin());
    }
    Arc::new(table)
}

fn make_square_wave(size: i32) -> Arc<Vec<f32>> {
    let mut table = Vec::with_capacity(size as usize);
    let full_circle = std::f32::consts::TAU;
    for i in 0..size as usize {
        let percent_through = i as f32 / size as f32;
        let value = (percent_through * full_circle).sin();
        if value >= 0.0 {
            table.push(1.0);
        } else {
            table.push(-1.0);
        };
    }
    Arc::new(table)
}

fn make_triangle_wave(size: i32) -> Arc<Vec<f32>> {
    let half_size = size / 2;
    let mut table: Vec<f32> = Vec::with_capacity(size as usize);
    for i in 0..half_size as usize {
        let percent_through = i as f32 / half_size as f32;
        let value = (1.0 * percent_through) - 1.0;
        table.push(value);
    }
    for i in 0..half_size as usize {
        let percent_through = i as f32 / half_size as f32;
        let value = 1.0 * percent_through;
        table.push(value);
    }
    Arc::new(table)
}

pub struct Voice {
    pub freq: f32,
    pub phase: f32
}
pub struct Synth {
    pub wave_table: Arc<Vec<f32>>,
    pub voices: Mutex<Vec<Voice>>
}

impl Synth {

    pub fn new() -> Self {
        Self {
            wave_table: make_sine_wave(1024),
            voices: Mutex::new(Vec::new())
        }
    }

    pub fn on(&self, freq:f32) {
        let mut voices = self.voices.lock().unwrap();
        if voices.iter().any(|v| v.freq == freq) {
            return
        }
        voices.push(Voice {
            freq: freq,
            phase: 0.0
        });
    }

    pub fn off(&self, freq:f32) {
        let mut voices = self.voices.lock().unwrap();
        voices.retain(|v| (v.freq - freq).abs() > 0.1);
    }
}
