#[macro_use]
extern crate vst;

use vst::buffer::AudioBuffer;
use vst::plugin::{CanDo, Category, Info, Plugin};
use vst::event::Event;
use vst::api::{Events, Supported};
mod oscillator;
mod waves;

struct Synth {
    sample_rate: f64,
    time: f64,
    notes: Vec<u8>,
    oscillators: Vec<oscillator::Oscillator>,
    wave_types: u8,
    pan: f32,
    attack: f64,
    default_oscillator: oscillator::Oscillator,
}

impl Synth {
    fn time_per_sample(&self) -> f64 {
        1.0 / self.sample_rate
    }

    fn process_midi_event(&mut self, data: [u8; 3]) {
        match data[0] {
            128 => self.note_off(data[1]),
            144 => self.note_on(data[1]),
            _ => (),
        }
    }

    fn note_on(&mut self, note: u8) {
        self.notes.push(note)
    }

    fn note_off(&mut self, note: u8) {
        self.notes
            .iter()
            .position(|&n| n == note)
            .map(|n| self.notes.remove(n));
    }

    fn get_pan_text(&self) -> String {
        if self.pan < 0.01 && self.pan > -0.01 {
            "center".to_string()
        } else if self.pan < 0.0 {
            format!("{}% left", (self.pan * 100.0).round().abs())
        } else {
            format!("{}% right", (self.pan * 100.0).round())
        }
    }

    fn get_oscillator(&self, index: usize) -> &oscillator::Oscillator {
        match self.oscillators.get(index) {
            Some(oscillator) => oscillator,
            None => &self.default_oscillator,
        }
    }

    fn get_oscillator_mut(&mut self, index: usize) -> &oscillator::Oscillator {
        match self.oscillators.get_mut(index) {
            Some(oscillator) => oscillator,
            None => &self.default_oscillator,
        }
    }

    fn apply_attack(&self, time: f64, sample: f32) -> f32{
        let alpha = if time < self.attack {
            time / self.attack
        } else {
            1.0
        };
        sample * alpha as f32
    }

    fn pan(&self, sample: f32, left_channel: bool) -> f32 {
        if left_channel && self.pan > 0.0 {
            sample * (1.0 - self.pan)
        } else if !left_channel && self.pan < 0.0 {
            sample * (-1.0 - self.pan).abs()
        } else {
            sample
        }
    }
}

impl Default for Synth {
    fn default() -> Synth {
        Synth {
            sample_rate: 44100.0,
            time: 0.0,
            notes: vec![],
            oscillators: vec![Default::default(), Default::default(), Default::default()],
            wave_types: 7,
            pan: 0.0,
            attack: 0.0,
            default_oscillator: Default::default(),
        }
    }
}

impl Plugin for Synth {
    fn get_info(&self) -> Info {
        Info {
            name: "rvst".to_string(),
            vendor: "EmilianStankov".to_string(),
            unique_id: 8888,
            category: Category::Synth,
            inputs: 2,
            outputs: 2,
            parameters: 11,
            initial_delay: 0,
            version: 300,
            ..Info::default()
        }
    }

    fn get_parameter(&self, index: i32) -> f32 {
        match index {
            0 => self.get_oscillator(0).get_wave_type() as f32 / self.wave_types as f32,
            1 => self.get_oscillator(0).get_volume(),
            2 => (8192 + self.get_oscillator(0).get_pitch_bend()) as f32 / 16384.0,
            3 => self.get_oscillator(1).get_wave_type() as f32 / self.wave_types as f32,
            4 => self.get_oscillator(1).get_volume(),
            5 => (8192 + self.get_oscillator(1).get_pitch_bend()) as f32 / 16384.0,
            6 => self.get_oscillator(2).get_wave_type() as f32 / self.wave_types as f32,
            7 => self.get_oscillator(2).get_volume(),
            8 => (8192 + self.get_oscillator(2).get_pitch_bend()) as f32 / 16384.0,
            9 => (self.pan + 1.0) / 2.0,
            10 => self.attack as f32 / 10.0,
            _ => 0.0,
        }
    }

    fn set_parameter(&mut self, index: i32, value: f32) {
        //TODO: Make get_oscillator_mut work
        match index {
            0 => self.oscillators[0].set_wave_type(value),
            1 => self.oscillators[0].set_volume(value),
            2 => self.oscillators[0].set_pitch_bend((value * 16384.0) as i16 - 8192),
            3 => self.oscillators[1].set_wave_type(value),
            4 => self.oscillators[1].set_volume(value),
            5 => self.oscillators[1].set_pitch_bend((value * 16384.0) as i16 - 8192),
            6 => self.oscillators[2].set_wave_type(value),
            7 => self.oscillators[2].set_volume(value),
            8 => self.oscillators[2].set_pitch_bend((value * 16384.0) as i16 - 8192),
            9 => self.pan = 2.0 * value - 1.0,
            10 => self.attack = (10.0 * value) as f64,
            _ => (),
        }
    }

    fn get_parameter_name(&self, index: i32) -> String {
        match index {
            0 => "Osc 1".to_string(),
            1 => "Osc 1 Volume".to_string(),
            2 => "Osc 1 Pitch".to_string(),
            3 => "Osc 2".to_string(),
            4 => "Osc 2 Volume".to_string(),
            5 => "Osc 2 Pitch".to_string(),
            6 => "Osc 3".to_string(),
            7 => "Osc 3 Volume".to_string(),
            8 => "Osc 3 Pitch".to_string(),
            9 => "Pan".to_string(),
            10 => "Attack".to_string(),
            _ => "".to_string(),
        }
    }

    fn get_parameter_text(&self, index: i32) -> String {
        match index {
            0 => self.get_oscillator(0).get_wave_type_text(),
            1 => format!("{}%", (self.get_oscillator(0).get_volume() * 100.0).round()),
            2 => format!("{}", self.get_oscillator(0).get_pitch_bend()),
            3 => self.get_oscillator(1).get_wave_type_text(),
            4 => format!("{}%", (self.get_oscillator(1).get_volume() * 100.0).round()),
            5 => format!("{}", self.get_oscillator(1).get_pitch_bend()),
            6 => self.get_oscillator(2).get_wave_type_text(),
            7 => format!("{}%", (self.get_oscillator(2).get_volume() * 100.0).round()),
            8 => format!("{}", self.get_oscillator(2).get_pitch_bend()),
            9 => self.get_pan_text(),
            10 => format!("{}ms", (self.attack * 1000.0) as u16),
            _ => "".to_string(),
        }
    }

    fn get_parameter_label(&self, index: i32) -> String {
        match index {
            _ => "".to_string(),
        }
    }

    fn process_events(&mut self, events: &Events) {
        for event in events.events() {
            match event {
                Event::Midi(ev) => self.process_midi_event(ev.data),
                _ => (),
            }
        }
    }

    fn set_sample_rate(&mut self, rate: f32) {
        self.sample_rate = f64::from(rate);
    }

    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        let samples = buffer.samples();
        let per_sample = self.time_per_sample();

        let mut left_channel = true;
        for (input_buffer, output_buffer) in buffer.zip() {
            if self.notes.len() < 1 {
                self.time = 0.0;
            }
            let mut time = self.time;
            for (_, output_sample) in input_buffer.iter().zip(output_buffer) {
                for oscillator in self.oscillators.iter() {
                    for current_note in &self.notes {
                        *output_sample +=
                            oscillator.get_wave_value(time, *current_note)
                                * oscillator.get_volume();
                        *output_sample = *output_sample * (1.0 / self.oscillators.len() as f32);
                    }
                }
                *output_sample = self.pan(*output_sample, left_channel);
                *output_sample = self.apply_attack(time, *output_sample);
                time += per_sample;
            }
            left_channel = false;
        }
        self.time += samples as f64 * per_sample;
    }

    fn can_do(&self, can_do: CanDo) -> Supported {
        match can_do {
            CanDo::ReceiveMidiEvent => Supported::Yes,
            _ => Supported::Maybe,
        }
    }
}

plugin_main!(Synth);
