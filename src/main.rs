use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use rand::random;
use std::sync::mpsc::channel;

fn write_audio(data: &mut [f32], _: &cpal::OutputCallbackInfo) {
    for sample in data.iter_mut() {
        *sample = random::<f32>() / 10.;
    }
}

fn main() {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("no output device available");

    for device in host.output_devices().unwrap() {
        println!("{}", device.name().unwrap());
        for config in device.supported_output_configs().unwrap() {
            println!(
                "  {: <5}-{: <5} Hz {: <3} samples across {} channels",
                config.min_sample_rate().0,
                config.max_sample_rate().0,
                config.sample_format(),
                config.channels(),
            )
        }
    }

    println!(
        "{:#?}",
        host.output_devices()
            .unwrap()
            .map(|device| { device.name().unwrap() })
            .collect::<Vec<_>>()
    );

    let mut configs = device
        .supported_output_configs()
        .expect("error while querying configs");
    let config = configs
        .next()
        .expect("no supported config")
        .try_with_sample_rate(cpal::SampleRate(48000))
        .expect("44.1kHz sample rate not supported");
    let stream = device
        .build_output_stream(
            &config.config(),
            write_audio,
            move |err| {
                eprintln!("stream error: {}", err);
            },
            None,
        )
        .unwrap();

    let (sigint_tx, sigint_rx) = channel();
    ctrlc::set_handler(move || sigint_tx.send(()).unwrap()).unwrap();

    stream.play().unwrap();
    sigint_rx.recv().unwrap();
}
