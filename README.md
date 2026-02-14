Hello this is my unfinished wavetable synthesizer, eat my nards. (you need rust downloaded to run this)

Its a simple wavetable synthesizer that reads at a default 1024 sampled wave stored in a vector. 
Cpal is used to talk to the soundcard and send values to a buffer. 
Winit is the other library I used for key input (window needs to be focused for key event handling)

To run, make sure you have rust set up on your pc and just run the main function in "main.rs".
at some point I'll turn this into an executable but I want to actually make it more professional before that.

