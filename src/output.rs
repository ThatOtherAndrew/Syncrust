use cpal::traits::{DeviceTrait, StreamTrait};
use cpal::{BufferSize, Device, SampleRate, Stream, StreamConfig};
use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex};

pub struct Output {
    stream: Stream,
    buffer: Arc<(Mutex<VecDeque<f32>>, Condvar)>,
}

impl Output {
    pub fn new(device: Device, sample_rate: u32) -> Self {
        let name = device.name().unwrap();
        let buffer = Arc::new((Mutex::new(VecDeque::new()), Condvar::new()));
        let callback_buffer = buffer.clone();

        Output {
            stream: device
                .build_output_stream(
                    &StreamConfig {
                        channels: 2,
                        sample_rate: SampleRate(sample_rate),
                        buffer_size: BufferSize::Default,
                    },
                    move |data: &mut [f32], _| {
                        let (buffer, condvar) = &*callback_buffer;
                        let mut buffer = condvar
                            .wait_while(buffer.lock().unwrap(), |buf| buf.len() < data.len())
                            .unwrap();
                        for sample in data {
                            *sample = buffer.pop_front().unwrap();
                        }
                    },
                    move |error| {
                        eprintln!("{}: error: {:?}", name, error);
                    },
                    None,
                )
                .unwrap(),
            buffer,
        }
    }

    pub fn write(&mut self, data: &[f32]) {
        let (buffer, condvar) = &*self.buffer;
        buffer.lock().unwrap().extend(data);
        condvar.notify_one();
    }
    
    pub fn play(&mut self) {
        self.stream.play().unwrap();
    }
}
