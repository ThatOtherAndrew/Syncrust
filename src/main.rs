mod output;

use std::f32::consts::{TAU};
use crate::output::Output;
use cpal::traits::{DeviceTrait, HostTrait};
use std::sync::mpsc::channel;

const RATE: u32 = 48_000;
const FRATE: f32 = RATE as f32;  // short for FrustRATE, because why tf is this necessary

fn main() {
    let host = cpal::default_host();

    let mut outputs = Vec::new();
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

        outputs.push(Output::new(device, RATE));
    }

    let (sigint_tx, sigint_rx) = channel();
    ctrlc::set_handler(move || sigint_tx.send(()).unwrap()).unwrap();

    for output in &mut outputs {
        output.play();
    }

    let mut t = 0;
    loop {
        if sigint_rx.try_recv().is_ok() {
            break;
        };

        let sine = f32::sin(t as f32 * TAU * 440. / FRATE) / 5.;

        for output in &mut outputs {
            output.write(&[sine])
        }
        t += 1;
    }
}
