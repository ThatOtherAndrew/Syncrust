mod output;

use crate::output::Output;
use cpal::traits::{DeviceTrait, HostTrait};
use rand::random;
use std::sync::mpsc::channel;

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

        outputs.push(Output::new(device));
    }

    let (sigint_tx, sigint_rx) = channel();
    ctrlc::set_handler(move || sigint_tx.send(()).unwrap()).unwrap();

    for output in &mut outputs {
        output.play();
    }

    loop {
        if sigint_rx.try_recv().is_ok() {
            break;
        };
        let random_sample = random::<f32>() / 20.;
        for output in &mut outputs {
            output.write(&[random_sample])
        }
    }
}
