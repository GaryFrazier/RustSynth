use crate::audio;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button};

pub fn build_ui(app: &Application) {

    // create audio device
    let device = audio::device::get_device();
    let config = audio::stream::get_stream_config(&device);
    
    // Create a sine wave generator
    let sample_rate = config.sample_rate.0 as f32;
    let mut phase = 0.0;
    let frequency = 440.0; // Frequency of the sine wave in Hz
    let volume = 0.03;

    let callback = move |output: &mut [f32], _: &cpal::OutputCallbackInfo| {
        for sample in output.iter_mut() {
            let value = (2.0 * std::f32::consts::PI * frequency * phase).sin() * volume;
            *sample = value;
            phase = (phase + 1.0 / sample_rate) % 1.0;
        }
    };

    let stream = audio::stream::create_audio_stream(device, config, callback);

    // Create a button with label and margins
    let button = Button::builder()
        .label("Press me!")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    // Connect to "clicked" signal of `button`
    button.connect_clicked(move |button| {
        // Set the label to "Hello World!" after the button has been clicked on
        button.set_label("Hello World!");
        audio::stream::play_stream(&stream);
    });

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("V-Track")
        .child(&button)
        .build();

    // Present window
    window.present();
}