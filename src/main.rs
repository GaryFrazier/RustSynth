mod audio;

fn main() {
    let device = audio::device::get_device();
    let config = audio::stream::get_stream_config(&device);
    
    // Create a sine wave generator
    let sample_rate = config.sample_rate.0 as f32;
    let mut phase = 0.0;
    let frequency = 440.0; // Frequency of the sine wave in Hz
    let volume = 1.0;

    let callback = move |output: &mut [f32], _: &cpal::OutputCallbackInfo| {
        for sample in output.iter_mut() {
            let value = (2.0 * std::f32::consts::PI * frequency * phase).sin() * volume;
            *sample = value;
            phase = (phase + 1.0 / sample_rate) % 1.0;
        }
    };

    audio::stream::play_stream(device, config, callback);
}