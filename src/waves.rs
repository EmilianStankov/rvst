extern crate rand;
use std::f64::consts::PI;

pub fn midi_value_to_freq(pitch: u8, pitch_bend: i16) -> f64 {
    const A4: i8 = 69;
    const A4_FREQ: f64 = 440.0;
    const CENT: f64 = 1.00057778951;

    // There are 128 midi notes. https://en.wikipedia.org/wiki/MIDI_tuning_standard
    let mut pitch_modifier = 1.0;
    if pitch_bend < 0 {
        pitch_modifier = 1.0 / CENT.powf(((-pitch_bend as f64 / 8192.0) * 1200.0).round());
    } else if pitch_bend > 0 {
        pitch_modifier = CENT.powf(((pitch_bend as f64 / 8192.0) * 1200.0).round());
    }
    ((f64::from(pitch as i8 - A4)) / 12.0).exp2() * A4_FREQ * pitch_modifier
}

pub fn sine_wave(time: f64, note: u8, pitch_bend: i16) -> f32 {
    (time * midi_value_to_freq(note, pitch_bend) * 2.0 * PI).sin() as f32
}

pub fn square_wave(time: f64, note: u8, pitch_bend: i16) -> f32 {
    let sine = sine_wave(time, note, pitch_bend);
    if sine > 0.0 {
        1.0
    } else {
        -1.0
    }
}

pub fn saw_wave(time: f64, note: u8, pitch_bend: i16) -> f32 {
    let period_time = 1.0 / midi_value_to_freq(note, pitch_bend);
    let t = time % period_time;

    ((t / period_time) * 2.0 - 1.0) as f32
}

pub fn reversed_saw_wave(time: f64, note: u8, pitch_bend: i16) -> f32 {
    -saw_wave(time, note, pitch_bend)
}

pub fn triangle_wave(time: f64, note: u8, pitch_bend: i16) -> f32 {
    2.0 * saw_wave(time, note, pitch_bend).abs() - 1.0
}

pub fn round_sine(time: f64, note: u8, pitch_bend: i16) -> f32 {
    sine_wave(time, note, pitch_bend).round()
}

pub fn noise() -> f32 {
    rand::random::<f32>() * 2.0 - 1.0
}

#[cfg(test)]
mod tests {
    use waves::midi_value_to_freq;
    const A4: u8 = 69;
    #[test]
    fn test_midi_value_to_freq() {
        //RHS values from: https://soundslikejoe.com/wp-content/uploads/2014/07/Bt5EVfVCEAAdeZe1.png
        assert_eq!((midi_value_to_freq(12, 0) * 10.0).round() / 10.0, 16.4);
        assert_eq!((midi_value_to_freq(20, 0) * 10.0).round() / 10.0, 26.0);
        assert_eq!((midi_value_to_freq(35, 0) * 10.0).round() / 10.0, 61.7);
        assert_eq!(midi_value_to_freq(A4, 0), 440.0);
        assert_eq!((midi_value_to_freq(71, 0) * 10.0).round() / 10.0, 493.9);
        assert_eq!((midi_value_to_freq(118, 0) * 10.0).round() / 10.0, 7458.6);
        assert_eq!((midi_value_to_freq(127, 0) * 10.0).round() / 10.0, 12543.9);
    }

    use waves::sine_wave;
    #[test]
    fn test_sine_wave() {
        assert_eq!(sine_wave(0.0, A4, 0), 0.0);
        assert_eq!(sine_wave(441.0/1760.0, A4, 0), 1.0);
        assert_eq!(sine_wave(443.0/1760.0, A4, 0), -1.0);
    }

    use waves::square_wave;
    #[test]
    fn test_square_wave() {
        assert_eq!(square_wave(0.0, A4, 0), -1.0);
        assert_eq!(square_wave(440.0, A4, 0), 1.0);
        assert_eq!(square_wave(441.0, A4, 0), -1.0);
        assert_eq!(square_wave(879.0, A4, 0), -1.0);
        assert_eq!(square_wave(880.0, A4, 0), 1.0);
    }

    use waves::saw_wave;
    #[test]
    fn test_saw_wave() {
        assert_eq!(saw_wave(0.0, A4, 0), -1.0);
        assert_eq!(saw_wave(0.5/440.0, A4, 0), 0.0);
        assert_eq!(saw_wave(0.99999999/440.0, A4, 0), 1.0);
        assert_eq!(saw_wave(1.0/440.0, A4, 0), -1.0);
        assert_eq!(saw_wave(1.25/440.0, A4, 0), -0.5);
        assert_eq!(saw_wave(1.75/440.0, A4, 0), 0.5);
    }

    use waves::reversed_saw_wave;
    #[test]
    fn test_reversed_saw_wave() {
        assert_eq!(reversed_saw_wave(0.0, A4, 0), 1.0);
        assert_eq!(reversed_saw_wave(0.5/440.0, A4, 0), 0.0);
        assert_eq!(reversed_saw_wave(0.99999999/440.0, A4, 0), -1.0);
        assert_eq!(reversed_saw_wave(1.0/440.0, A4, 0), 1.0);
        assert_eq!(reversed_saw_wave(1.25/440.0, A4, 0), 0.5);
        assert_eq!(reversed_saw_wave(1.75/440.0, A4, 0), -0.5);
    }

    use waves::triangle_wave;
    #[test]
    fn test_triangle_wave() {
        assert_eq!(triangle_wave(0.0, A4, 0), 1.0);
        assert_eq!(triangle_wave(0.5/440.0, A4, 0), -1.0);
        assert_eq!(triangle_wave(0.99999999/440.0, A4, 0), 1.0);
        assert_eq!(triangle_wave(1.0/440.0, A4, 0), 1.0);
        assert_eq!(triangle_wave(1.25/440.0, A4, 0), 0.0);
        assert_eq!(triangle_wave(1.75/440.0, A4, 0), 0.0);
    }

    use waves::round_sine;
    #[test]
    fn test_round_sine() {
        assert_eq!(round_sine(0.0, A4, 0), 0.0);
        assert_eq!(round_sine(110.0/1760.0, A4, 0), 0.0);
        assert_eq!(round_sine(220.0/1760.0, A4, 0), 0.0);
        assert_eq!(round_sine(330.0/1760.0, A4, 0), 0.0);
        assert_eq!(round_sine(441.0/1760.0, A4, 0), 1.0);
        assert_eq!(round_sine(443.0/1760.0, A4, 0), -1.0);
    }
}
