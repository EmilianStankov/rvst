use waves;

pub struct Oscillator {
    wave_type: u8,
    wave_types: u8,
    volume: f32
}

impl Oscillator {
    pub fn set_wave_type(&mut self, value: f32) {
        self.wave_type = (value * self.wave_types as f32).floor() as u8
    }

    pub fn get_wave_type(&self) -> u8 {
        self.wave_type
    }

    pub fn get_wave_value(&self, time: f64, note: u8, pitch_bend: i16) -> f32 {
        match self.get_wave_type() {
            0 => waves::sine_wave(time, note, pitch_bend),
            1 => waves::saw_wave(time, note, pitch_bend),
            2 => waves::reversed_saw_wave(time, note, pitch_bend),
            3 => waves::square_wave(time, note, pitch_bend),
            4 => waves::triangle_wave(time, note, pitch_bend),
            5 => waves::round_sine(time, note, pitch_bend),
            _ => waves::sine_wave(time, note, pitch_bend)
        }
    }

    pub fn get_wave_type_text(&self) -> String {
        match self.wave_type {
            0 => "Sine".to_string(),
            1 => "Saw".to_string(),
            2 => "Reversed Saw".to_string(),
            3 => "Square".to_string(),
            4 => "Triangle".to_string(),
            5 => "Sine Rounded".to_string(),
            _ => "Sine".to_string()
        }
    }

    pub fn set_volume(&mut self, value: f32) {
        self.volume = value;
    }

    pub fn get_volume(&self) -> f32 {
        self.volume
    }
}

impl Default for Oscillator {
    fn default() -> Oscillator {
        Oscillator {
            wave_type: 0,
            wave_types: 6,
            volume: 1.0
        }
    }
}
