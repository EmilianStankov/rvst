#[macro_use]
extern crate vst;

use vst::buffer::AudioBuffer;
use vst::plugin::{Category, Plugin, Info, CanDo};
use vst::event::Event;
use vst::api::{Supported, Events};
mod waves;

struct Synth {
    sample_rate: f64,
    time: f64,
    note_duration: f64,
    note: Option<u8>,
    wave_type: u8,
    waves: u8
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

    fn get_wave_type_text(&self) -> String {
        match self.wave_type {
            0 => "Sine".to_string(),
            1 => "Saw".to_string(),
            2 => "Reversed Saw".to_string(),
            3 => "Square".to_string(),
            4 => "Triangle".to_string(),
            _ => "Sine".to_string()
        }
    }

    fn set_wave_type(&mut self, value: f32) {
        self.wave_type = (value * self.waves as f32).floor() as u8
    }
}

impl Default for Synth {
    fn default() -> Synth {
        Synth {
            sample_rate: 44100.0,
            note_duration: 0.0,
            time: 0.0,
            note: None,
            wave_type: 0,
            waves: 5
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
            parameters: 1,
            initial_delay: 0,
            ..Info::default()
        }
    }

    fn get_parameter(&self, index: i32) -> f32 {
        match index {
            0 => self.wave_type as f32 / self.waves as f32,
            _ => 0.0
        }
    }

    fn set_parameter(&mut self, index: i32, value: f32) {
        match index {
            0 => self.set_wave_type(value),
            _ => ()
        }
    }

    fn get_parameter_name(&self, index: i32) -> String {
        match index {
            0 => "Wave Type".to_string(),
            _ => "".to_string()
        }
    }

    fn get_parameter_text(&self, index: i32) -> String {
        match index {
            0 => format!("{}", self.get_wave_type_text()),
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

        for (input_buffer, output_buffer) in buffer.zip() {
            let mut time = self.time;
            for (_, output_sample) in input_buffer.iter().zip(output_buffer) {
                if let Some(current_note) = self.note {
                    match self.wave_type {
                        0 => *output_sample = waves::sine_wave(time, current_note),
                        1 => *output_sample = waves::saw_wave(time, current_note),
                        2 => *output_sample = waves::reversed_saw_wave(time, current_note),
                        3 => *output_sample = waves::square_wave(time, current_note),
                        4 => *output_sample = waves::triangle_wave(time, current_note),
                        _ => *output_sample = waves::sine_wave(time, current_note)
                    }

                } else {
                    *output_sample = 0.0;
                }
                time += per_sample;
            }
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
