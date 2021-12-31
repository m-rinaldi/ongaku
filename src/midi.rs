use crate::{Music, MusicChunk};

pub struct Midi {
    header: Header,
    track: Track,
}

impl Midi {
    pub fn new(music: Music) -> Self {
        let header = Header::new();
        let track = Track::new(music.chunks);

        Midi { header, track }
    }

    pub fn to_file(&self, filepath: &std::path::Path) {
        let mut file = std::fs::File::create(filepath).unwrap();
        
        let mut bin = std::io::Cursor::new(self.bin());
        std::io::copy(&mut bin, &mut file).unwrap();
    }
}

impl Bin for Midi {
    type Output = Vec<u8>;

    fn bin(&self) -> Vec<u8> {
        let mut vec = Vec::new();
        vec.extend(self.header.bin());
        vec.extend(self.track.bin());
        
        vec
    }
}

struct Header {
    format: u16,
    num_tracks: u16,
    tickdiv: u16,
}

impl Header {
    pub fn new() -> Self {
        Header { format: 0, num_tracks: 1, tickdiv: 1 }
    }
}
struct Track {
    events: Vec<Event>,
}

impl Track {
    pub fn new<I: IntoIterator<Item=MusicChunk>>(iterable: I) -> Self {
        let mut events = Vec::new();
        let mut dt = 0u8;
        for music_chunk in iterable {
            use MusicChunk::*;
            use Event::*;
            match music_chunk {
                Tone(dur) => {
                    events.push(NoteOn(dt));
                    events.push(NoteOff(dur));
                    dt = 0;
                }
                Rest(dur) => {
                    dt += dur;
                }
            }
        }

        events.push(Event::EndOfTrack);

        Track { events }
    }
}

#[derive(Copy, Clone, Debug)]
enum Event {
    //TODO DeltaTime
    NoteOn(u8),
    NoteOff(u8),
    EndOfTrack,
}

trait Bin {
    type Output: IntoIterator<Item=u8>;
    fn bin(&self) -> Self::Output;
}

impl Bin for Header {
    type Output = Vec<u8>;

    fn bin(&self) -> Vec<u8> {
        const HEADER_SIZE: usize = 14;
        let mut vec = Vec::with_capacity(HEADER_SIZE);

        // identifier
        {
            vec.extend("MThd".bytes());
        }
        
        // length
        {
            vec.extend(6u32.to_be_bytes())
        }

        // format
        {
            vec.extend(self.format.to_be_bytes())
        }

        // ntracks
        {
            vec.extend(self.num_tracks.to_be_bytes())
        }

        // tickdiv
        {
            vec.extend(self.tickdiv.to_be_bytes())
        }

        vec
    }
}

impl Bin for Track {
    type Output = Vec<u8>;

    fn bin(&self) -> Vec<u8> {
        let mut track_data = Vec::new();
        
        for data in &self.events {
            track_data.extend(data.bin());
        }

        let data_len = track_data.len();
        let mut vec = Vec::with_capacity(data_len + 8);
        vec.extend("MTrk".bytes());
        {
            vec.extend((data_len as u32).to_be_bytes());
        }

        vec.extend(track_data);
        vec
    }
}

use local_vec::LocalVec;

enum Switch {
    On,
    Off,
}

fn encode_note(dur: u8, switch: Switch) -> LocalVec<u8, 4> {
    let mut vec = LocalVec::new();

    // delta time
    vec.extend(to_vli(dur));

    // whether note on / note off
    let switch_code: u8 = match switch {
        Switch::Off => 0x80,
        Switch::On  => 0x90
    };
    vec.push(switch_code);

    // TODO extend
    // tone itself
    vec.push(0x3c);

    // press velocity
    vec.push(0x64);

    vec
}

fn to_vli(val: u8) -> LocalVec<u8, 4> {
    // TODO extend this for values larger than 127
    let mut vec = LocalVec::new();
    vec.push(val & 0x7f);
    vec
}

impl Bin for Event {
    type Output = LocalVec<u8, 4>;

    fn bin(&self) -> LocalVec<u8, 4> {
        let mut vec = LocalVec::new();

        use Event::*;
        match *self {
            NoteOn(dt) => {
                vec.extend(encode_note(dt, Switch::On));
            }
            NoteOff(dt) => {
                vec.extend(encode_note(dt, Switch::Off))
            }
            EndOfTrack => {
                vec.extend(0x00ff2f00_u32.to_be_bytes())
            }
        }

        vec
    }
}