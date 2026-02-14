mod keyboard;
mod synthesizer;
mod audio;

use std::sync::Arc;
use synthesizer::Synth;

fn main() {
    
    let synth = Arc::new(Synth::new());

    
    let _stream = audio::start_audio_engine(synth.clone());

    println!("Synth up.");
    println!("Use Home Row (A-L) for white keys and (W-P) for black keys.");
    println!("Close the window to exit.");

    keyboard::run_keyboard_listener(synth);
}