use std::{sync::Mutex};

pub enum WaveType {
    Sine,
    Square,
    Saw,
    Triangle
}

fn make_sine_wave(size: i32) -> Vec<f32> {
    let mut table = Vec::with_capacity(size as usize);
    let full_circle = std::f32::consts::TAU;
    for i in 0..size as usize {
        let percent_through = i as f32 / size as f32;
        let value = percent_through * full_circle;
        table.push(value.sin());
    }
    table
}

fn make_square_wave(size: i32) -> Vec<f32> {
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
    table
}

fn make_triangle_wave(size: i32) -> Vec<f32> {
    let half_size = size / 2;
    let mut table: Vec<f32> = Vec::with_capacity(size as usize);
    for i in 0..half_size as usize {
        let percent_through = i as f32 / half_size as f32;
        let value = (percent_through * 2.0) - 1.0;
        table.push(value);
    }
    for i in 0..half_size as usize {
        let percent_through = i as f32 / half_size as f32;
        let n_value  = percent_through * 2.0;
        let value = 1.0 - n_value;
        table.push(value);
    }
    table
}

fn make_saw_wave(size: i32) -> Vec<f32> {
    let mut table: Vec<f32> = Vec::with_capacity(size as usize);
    for i in 0..size as usize {
        let percent_through = i as f32 / size as f32;
        let n_value = percent_through as f32 * 2.0;
        let value = 1.0 - n_value;
        table.push(value);
    }
    table
}

pub struct Voice {
    pub freq: f32,
    pub phase: f32
}
pub struct Synth {
    pub wave_table: Mutex<Vec<f32>>,
    pub voices: Mutex<Vec<Voice>>
}

impl Synth {

    pub fn new() -> Self {
        Self {
            wave_table: Mutex::new(make_sine_wave(1024)),
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

    pub fn change_wave(& self, wave: WaveType) {
        let mut table = self.wave_table.lock().unwrap();
        *table = match wave {
            WaveType::Sine => make_sine_wave(1024),
            WaveType::Triangle => make_triangle_wave(1024),
            WaveType::Saw => make_saw_wave(1024),
            WaveType::Square => make_square_wave(1024),
        };
    }

    pub fn get_lerp(table: &Vec<f32>, index: &f32) -> f32 {
        let floor = index.floor() as usize;
        let ceiling = index.ceil() as usize;
        let difference = table[ceiling] - table[floor];
        if *index < 1.0 {
            return (difference * index) + table[floor];
        } else {
            let percent = index % 1.0;
            return (difference * percent) + table[floor];
        }
    }
}
