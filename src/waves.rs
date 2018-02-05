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
