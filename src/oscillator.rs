use waves;

pub struct Oscillator {
    wave_type: u8,
    wave_types: u8,
    volume: f32,
    pitch_bend: i16,
}

impl Oscillator {
    pub fn set_wave_type(&mut self, value: f32) {
        self.wave_type = (value * self.wave_types as f32).floor() as u8
    }

    pub fn get_wave_type(&self) -> u8 {
        self.wave_type
    }

    pub fn get_wave_value(&self, time: f64, note: u8) -> f32 {
        match self.get_wave_type() {
            0 => waves::sine_wave(time, note, self.get_pitch_bend()),
            1 => waves::saw_wave(time, note, self.get_pitch_bend()),
            2 => waves::reversed_saw_wave(time, note, self.get_pitch_bend()),
            3 => waves::square_wave(time, note, self.get_pitch_bend()),
            4 => waves::triangle_wave(time, note, self.get_pitch_bend()),
            5 => waves::round_sine(time, note, self.get_pitch_bend()),
            6 => waves::noise(),
            _ => waves::sine_wave(time, note, self.get_pitch_bend()),
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
            6 => "Noise".to_string(),
            _ => "Sine".to_string(),
        }
    }

    pub fn set_volume(&mut self, value: f32) {
        self.volume = value;
    }

    pub fn get_volume(&self) -> f32 {
        self.volume
    }

    pub fn set_pitch_bend(&mut self, value: i16) {
        self.pitch_bend = value;
    }

    pub fn get_pitch_bend(&self) -> i16 {
        self.pitch_bend
    }
}

impl Default for Oscillator {
    fn default() -> Oscillator {
        Oscillator {
            wave_type: 0,
            wave_types: 7,
            volume: 1.0,
            pitch_bend: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use oscillator::Oscillator;
    #[test]
    fn test_default_oscillator() {
        let osc: Oscillator = Oscillator::default();
        assert_eq!(osc.get_wave_type(), 0);
        assert_eq!(osc.get_wave_type_text(), "Sine".to_string());
        assert_eq!(osc.get_volume(), 1.0);
        assert_eq!(osc.get_pitch_bend(), 0);
    }

    #[test]
    fn test_oscillator_wave_types() {
        let mut osc: Oscillator = Oscillator::default();

        osc.set_wave_type(0.15);
        assert_eq!(osc.get_wave_type(), 1);
        assert_eq!(osc.get_wave_type_text(), "Saw".to_string());

        osc.set_wave_type(0.30);
        assert_eq!(osc.get_wave_type(), 2);
        assert_eq!(osc.get_wave_type_text(), "Reversed Saw".to_string());

        osc.set_wave_type(0.45);
        assert_eq!(osc.get_wave_type(), 3);
        assert_eq!(osc.get_wave_type_text(), "Square".to_string());

        osc.set_wave_type(0.60);
        assert_eq!(osc.get_wave_type(), 4);
        assert_eq!(osc.get_wave_type_text(), "Triangle".to_string());

        osc.set_wave_type(0.75);
        assert_eq!(osc.get_wave_type(), 5);
        assert_eq!(osc.get_wave_type_text(), "Sine Rounded".to_string());

        osc.set_wave_type(0.90);
        assert_eq!(osc.get_wave_type(), 6);
        assert_eq!(osc.get_wave_type_text(), "Noise".to_string());

        osc.set_wave_type(1.0);
        assert_eq!(osc.get_wave_type(), 7);
        assert_eq!(osc.get_wave_type_text(), "Sine".to_string());
    }

    #[test]
    fn test_volume() {
        let mut osc: Oscillator = Oscillator::default();
        osc.set_volume(0.5);
        assert_eq!(osc.get_volume(), 0.5);
    }

    #[test]
    fn test_pitch_bend() {
        let mut osc: Oscillator = Oscillator::default();
        osc.set_pitch_bend(4096);
        assert_eq!(osc.get_pitch_bend(), 4096);
    }
}
