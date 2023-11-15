
use cpal::{StreamConfig, Device, OutputCallbackInfo, SizedSample};
use cpal::traits::{DeviceTrait, StreamTrait};

pub fn get_stream_config(output_device: &Device) -> StreamConfig {
    let mut supported_configs_range = output_device.supported_output_configs()
    .expect("error while querying configs");

    let supported_config: StreamConfig = supported_configs_range.next()
        .expect("no supported config?!")
        .with_max_sample_rate()
        .into();

    return supported_config;
}

pub fn play_stream<C, T>(output_device: Device, stream_config: StreamConfig, stream_callback: C)
    where
    T: SizedSample,
    C: FnMut(&mut [T], &OutputCallbackInfo) + Send + 'static {
        
    // Create an audio stream with the given format and callback
    let stream = output_device.build_output_stream(
        &stream_config.into(),
        stream_callback,
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