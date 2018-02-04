use std::f64::consts::PI;

pub fn midi_value_to_freq(pitch: u8) -> f64 {
    const A4: i8 = 69;
    const A4_FREQ: f64 = 440.0;

    // There are 128 midi notes. https://en.wikipedia.org/wiki/MIDI_tuning_standard
    ((f64::from(pitch as i8 - A4)) / 12.0).exp2() * A4_FREQ
}

pub fn sine_wave(time: f64, note: u8) -> f32 {
    (time * midi_value_to_freq(note) * 2.0 * PI).sin() as f32
}

pub fn square_wave(time: f64, note: u8) -> f32 {
    let sine = sine_wave(time, note);
    if sine > 0.0 {
        1.0
    } else {
        -1.0
    }
}

pub fn saw_wave(time: f64, note: u8) -> f32 {
    let period_time = 1.0 / midi_value_to_freq(note);
    let t = time % period_time;

    ((t / period_time) * 2.0 - 1.0) as f32
}

pub fn reversed_saw_wave(time: f64, note: u8) -> f32 {
    -saw_wave(time, note)
}

pub fn triangle_wave(time: f64, note: u8) -> f32 {
    2.0 * saw_wave(time, note).abs() - 1.0
}
