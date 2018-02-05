#[macro_use]
extern crate vst;

use vst::buffer::AudioBuffer;
use vst::plugin::{Category, Plugin, Info, CanDo};
use vst::event::Event;
use vst::api::{Supported, Events};
mod oscillator;
mod waves;

struct Synth {
    sample_rate: f64,
    time: f64,
    note_duration: f64,
    note: Option<u8>,
    oscillators: Vec<oscillator::Oscillator>,
    wave_types: u8,
    pan: f32,
    pitch_bend: i16,
    default_oscillator: oscillator::Oscillator
}

impl Synth {

    fn time_per_sample(&self) -> f64 {
        1.0 / self.sample_rate
    }

    fn process_midi_event(&mut self, data: [u8; 3]) {
        match data[0] {
            128 => self.note_off(data[1]),
            144 => self.note_on(data[1]),
            _ => ()
        }
    }

    fn note_on(&mut self, note: u8) {
        self.note_duration = 0.0;
        self.note = Some(note)
    }

    fn note_off(&mut self, note: u8) {
        if self.note == Some(note) {
            self.note = None
        }
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
            None => &self.default_oscillator
        }
    }

    fn get_oscillator_mut(&mut self, index: usize) -> &oscillator::Oscillator {
        match self.oscillators.get_mut(index) {
            Some(oscillator) => oscillator,
            None => &self.default_oscillator
        }
    }
}

impl Default for Synth {
    fn default() -> Synth {
        Synth {
            sample_rate: 44100.0,
            note_duration: 0.0,
            time: 0.0,
            note: None,
            oscillators: vec![Default::default(), Default::default()],
            wave_types: 6,
            pan: 0.0,
            pitch_bend: 0,
            default_oscillator: Default::default()
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
            parameters: 6,
            initial_delay: 0,
            version: 100,
            ..Info::default()
        }
    }

    fn get_parameter(&self, index: i32) -> f32 {
        match index {
            0 => self.get_oscillator(0).get_wave_type() as f32 / self.wave_types as f32,
            1 => self.get_oscillator(0).get_volume(),
            2 => self.get_oscillator(1).get_wave_type() as f32 / self.wave_types as f32,
            3 => self.get_oscillator(1).get_volume(),
            4 => (self.pan + 1.0) / 2.0,
            5 => (8192 + self.pitch_bend) as f32 / 16384.0,
            _ => 0.0
        }
    }

    fn set_parameter(&mut self, index: i32, value: f32) {
        //TODO: Make get_oscillator_mut work
        match index {
            0 => self.oscillators[0].set_wave_type(value),
            1 => self.oscillators[0].set_volume(value),
            2 => self.oscillators[1].set_wave_type(value),
            3 => self.oscillators[1].set_volume(value),
            4 => self.pan = 2.0 * value - 1.0,
            5 => self.pitch_bend = (value * 16384.0) as i16 - 8192,
            _ => ()
        }
    }

    fn get_parameter_name(&self, index: i32) -> String {
        match index {
            0 => "Osc 1".to_string(),
            1 => "Osc 1 Volume".to_string(),
            2 => "Osc 2".to_string(),
            3 => "Osc 2 Volume".to_string(),
            4 => "Pan".to_string(),
            5 => "Pitch Bend".to_string(),
            _ => "".to_string()
        }
    }

    fn get_parameter_text(&self, index: i32) -> String {
        match index {
            0 => self.get_oscillator(0).get_wave_type_text(),
            1 => format!("{}%", (self.get_oscillator(0).get_volume() * 100.0).round()),
            2 => self.get_oscillator(1).get_wave_type_text(),
            3 => format!("{}%", (self.get_oscillator(1).get_volume() * 100.0).round()),
            4 => self.get_pan_text(),
            5 => format!("{}", self.pitch_bend),
            _ => "".to_string()
        }
    }

    fn get_parameter_label(&self, index: i32) -> String {
        match index {
            _ => "".to_string()
        }
    }

    fn process_events(&mut self, events: &Events) {
        for event in events.events() {
            match event {
                Event::Midi(ev) => self.process_midi_event(ev.data),
                _ => ()
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
            let mut time = self.time;
            for (_, output_sample) in input_buffer.iter().zip(output_buffer) {
                if let Some(current_note) = self.note {
                    for oscillator in self.oscillators.iter() {
                        *output_sample += oscillator.get_wave_value(time, current_note, self.pitch_bend);
                        *output_sample = *output_sample * oscillator.get_volume();
                        *output_sample = *output_sample * (1.0 / self.oscillators.len() as f32);
                    }
                    if left_channel && self.pan > 0.0 {
                        *output_sample = *output_sample * (1.0 - self.pan)
                    } else if !left_channel && self.pan < 0.0 {
                        *output_sample = *output_sample * (-1.0 - self.pan).abs()
                    }
                } else {
                    *output_sample = 0.0;
                }
                time += per_sample;
            }
            left_channel = false;
        }
        self.time += samples as f64 * per_sample;
    }

    fn can_do(&self, can_do: CanDo) -> Supported {
        match can_do {
            CanDo::ReceiveMidiEvent => Supported::Yes,
            _ => Supported::Maybe
        }
    }
}

plugin_main!(Synth);
