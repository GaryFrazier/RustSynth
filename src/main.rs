use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::StreamConfig;

fn main() {
    // Initialize the default host
    let host = cpal::default_host();

    // Get the default output device
    let default_output_device = host.default_output_device().expect("No output device available");

    let mut supported_configs_range = default_output_device.supported_output_configs()
        .expect("error while querying configs");

    let supported_config: StreamConfig = supported_configs_range.next()
        .expect("no supported config?!")
        .with_max_sample_rate()
        .into();

    // Create a sine wave generator
    let sample_rate = supported_config.sample_rate.0 as f32;
    let mut phase = 0.0;
    let frequency = 440.0; // Frequency of the sine wave in Hz
    let volume = 1.0;

    // Create a callback that generates audio samples
    let callback = move |output: &mut [f32], _: &cpal::OutputCallbackInfo| {
        for sample in output.iter_mut() {
            let value = (2.0 * std::f32::consts::PI * frequency * phase).sin() * volume;
            *sample = value;
            phase = (phase + 1.0 / sample_rate) % 1.0;
        }
    };

    // Create an audio stream with the given format and callback
    let stream = default_output_device.build_output_stream(
        &supported_config.into(),
        callback,
        // Error callback (optional)
        |err| eprintln!("Error in audio stream: {:?}", err),
        None
    ).expect("Failed to build output stream");

    // Start the audio stream
    stream.play().expect("Failed to play stream");

    // Sleep for a while to allow the audio to play
    std::thread::sleep(std::time::Duration::from_secs(5));

    // Stop the audio stream
    stream.pause().expect("Failed to pause stream");
}