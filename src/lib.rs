//TODO duration is measured in beats
#[derive(Copy, Clone)]
pub struct Dur(u8);

//TODO
#[derive(Copy, Clone)]
pub enum Pitch {
    C,
    D,
    Ef,
}

//TODO probably, Music is a generic enum
pub enum Music {
    /// a note is a pitch combined with a duration
    Note(Pitch, Dur),
    Rest(Dur),
    /// sequential composition: the sequential playing of two Music
    Seq(Box<Music>, Box<Music>),
    /// harmonic composition: playing both Music simultaneously
    Hrm(Box<Music>, Box<Music>),
}

fn trans(_: i8, pitch: Pitch) -> Pitch {
    // TODO
    pitch
}

pub fn harmonize_note(dur: Dur, pitch: Pitch) -> Music {
    use Music::*;
    Hrm(Box::new(Note(pitch, dur)), Box::new(Note(trans(0, pitch), dur)))
}

// TODO this can be generatized by using an interator as argument Iterator<Item=Pitch> instead
// of an slice
pub fn harmonize_list(dur: Dur, pitches: &[Pitch] /* TODO Iterator<Item=Pitch> instead */) -> Music {
    use Music::*;

    if pitches.is_empty() {
        return Rest(Dur(0))
    }

    Seq(
        Box::new(harmonize_note(dur, pitches[0])),
        Box::new(harmonize_list(dur, &pitches[1..])),
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
