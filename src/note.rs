pub struct Note {
    pitch: u8,
    velocity: u8,
}

impl Note {
    pub fn new(pitch: u8, velocity: u8) -> Note {
        Note{pitch: pitch, velocity: velocity}
    }

    pub fn get_pitch(&self) -> u8 {
        self.pitch
    }

    pub fn set_pitch(&mut self, pitch: u8) {
        self.pitch = pitch;
    }

    pub fn get_velocity(&self) -> u8 {
        self.velocity
    }

    pub fn set_velocity(&mut self, velocity: u8) {
        self.velocity = velocity;
    }
}

impl PartialEq for Note {
    fn eq(&self, other: &Note) -> bool {
        self.pitch == other.pitch
    }
}
