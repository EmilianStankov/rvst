#[derive(Debug)]
pub struct Note {
    pitch: u8,
    velocity: u8,
}

impl Note {
    pub fn new(pitch: u8, velocity: u8) -> Note {
        Note {
            pitch: pitch,
            velocity: velocity,
        }
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

#[cfg(test)]
mod tests {
    use note::Note;
    const A4: u8 = 69;
    #[test]
    fn test_note() {
        let note: Note = Note::new(A4, 100);
        assert_eq!(note.get_pitch(), A4);
        assert_eq!(note.get_velocity(), 100);
    }

    #[test]
    fn test_note_equality() {
        let note: Note = Note::new(A4, 100);
        assert_eq!(note, Note::new(A4, 100));
        assert_eq!(note, Note::new(A4, 10));
    }

    #[test]
    fn test_note_inequality() {
        let note: Note = Note::new(A4, 100);
        assert_ne!(note, Note::new(70, 100));
    }

    #[test]
    fn test_note_mutability() {
        let mut note: Note = Note::new(A4, 100);
        note.set_pitch(42);
        assert_ne!(note, Note::new(A4, 100));
        assert_eq!(note.get_pitch(), 42);
        assert_eq!(note, Note::new(42, 100));

        note.set_pitch(A4);
        note.set_velocity(42);
        assert_eq!(note.get_velocity(), 42);
        assert_eq!(note, Note::new(A4, 100));
    }
}
