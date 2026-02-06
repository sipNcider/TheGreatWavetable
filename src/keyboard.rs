use crate::synthesizer::Synth;
use std::sync::Arc;

use winit::application::ApplicationHandler;
use winit::event::{ElementState, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};
use winit::keyboard::{KeyCode, PhysicalKey};


// I hard coded the frequencies dont give a fuck
fn get_frequency_from_key(key: PhysicalKey) -> Option<f32> {
    if let PhysicalKey::Code(code) = key {
        match code {
            // White Keys (Middle Row)
            KeyCode::KeyA => Some(261.63), // C4
            KeyCode::KeyS => Some(293.66), // D4
            KeyCode::KeyD => Some(329.63), // E4
            KeyCode::KeyF => Some(349.23), // F4
            KeyCode::KeyG => Some(392.00), // G4
            KeyCode::KeyH => Some(440.00), // A4
            KeyCode::KeyJ => Some(493.88), // B4
            KeyCode::KeyK => Some(523.25), // C5
            KeyCode::KeyL => Some(587.33), // D5

            // Black Keys (Top Row)
            KeyCode::KeyW => Some(277.18), // C#4
            KeyCode::KeyE => Some(311.13), // D#4
            // KeyR is empty (between E and F)
            KeyCode::KeyT => Some(369.99), // F#4
            KeyCode::KeyY => Some(415.30), // G#4
            KeyCode::KeyU => Some(466.16), // A#4
            // KeyI is empty
            KeyCode::KeyO => Some(554.37), // C#5
            KeyCode::KeyP => Some(622.25), // D#5

            _ => None,
        }
    } else {
        None
    }
}

struct App {
    window: Option<Window>,
    synth: Arc<Synth>
}

impl ApplicationHandler for App {
    
    // Start window 
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = Window::default_attributes().with_title("Synth Interface");
        self.window = Some(event_loop.create_window(window_attributes).unwrap());
    }

    // Input handling
    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("Closing window...");
                event_loop.exit();
            }
            WindowEvent::KeyboardInput {event, ..} => {
                if let Some(freq) = get_frequency_from_key(event.physical_key) {
                    match event.state {
                        ElementState::Pressed => {
                            if !event.repeat {
                                self.synth.on(freq);
                            }
                        }
                        ElementState::Released => self.synth.off(freq)
                    };
                }
            }
            _ => (),
        }
    }
}

pub fn run_keyboard_listener(synth: Arc<Synth>) {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Wait);

    let mut app = App {
        window: None,
        synth,
    };

    event_loop.run_app(&mut app).unwrap();
}