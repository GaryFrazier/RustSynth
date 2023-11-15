use cpal::traits::HostTrait;
use cpal::Device;

pub fn get_device () -> Device {
    // Initialize the default host
    let host = cpal::default_host();

    // Get the default output device
    let default_output_device = host.default_output_device().expect("No output device available");
    return default_output_device;
}