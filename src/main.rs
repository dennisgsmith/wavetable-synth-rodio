use rodio::{OutputStream, Source};
use std::thread::sleep;
use std::time::Duration;

struct WavetableOscillator {
    sample_rate: u32,
    wavetable: Vec<f32>,
    index: f32,
    index_increment: f32,
}

impl WavetableOscillator {
    fn new(sample_rate: u32, wavetable: Vec<f32>) -> WavetableOscillator {
        return WavetableOscillator {
            sample_rate,
            wavetable,
            index: 0.0,
            index_increment: 0.0,
        };
    }

    fn set_frequency(&mut self, frequency: f32) {
        self.index_increment = frequency * self.wavetable.len() as f32 / self.sample_rate as f32;
    }

    fn get_sample(&mut self) -> f32 {
        let sample = self.lerp();
        self.index += self.index_increment;
        self.index %= self.wavetable.len() as f32;
        return sample;
    }

    fn lerp(&self) -> f32 {
        let truncated_index = self.index as usize;
        let next_index = (truncated_index + 1) % self.wavetable.len();

        let next_index_weight = self.index - truncated_index as f32;
        let truncated_index_weight = 1.0 - next_index_weight;

        return truncated_index_weight * self.wavetable[truncated_index]
            + next_index_weight * self.wavetable[next_index];
    }
}

impl Iterator for WavetableOscillator {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        return Some(self.get_sample());
    }
}

impl Source for WavetableOscillator {
    fn channels(&self) -> u16 {
        return 1;
    }

    fn sample_rate(&self) -> u32 {
        return self.sample_rate;
    }

    fn current_frame_len(&self) -> Option<usize> {
        return None;
    }

    fn total_duration(&self) -> Option<Duration> {
        return None;
    }
}

fn main() {
    let wavetable_size = 64;
    let mut wavetable: Vec<f32> = Vec::with_capacity(wavetable_size);

    for n in 0..wavetable_size {
        wavetable.push((2.0 * std::f32::consts::PI * n as f32 / wavetable_size as f32).sin())
    }

    let mut osc = WavetableOscillator::new(44100, wavetable);
    osc.set_frequency(440.0);
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let _result = stream_handle.play_raw(osc.convert_samples());
    sleep(Duration::from_secs(5))
}
