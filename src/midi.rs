struct Midi {
    header: Header,
    track: Track,
}

struct Header {
    format: u16,
    num_tracks: u16,
    tickdiv: u16,
}
struct Track {
    events: Vec<Event>,
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

        vec
    }
}

use local_vec::LocalVec;

impl Bin for Event {
    type Output = LocalVec<u8, 4>;
    fn bin(&self) -> LocalVec<u8, 4> {
        let mut vec = LocalVec<_,4>::new();

        use Event::*;
        match *self {
            NoteOn(dt) => {
            }
            NoteOff(dt) => {

            }
            EndOfTrack => {
                vec.extend(0x00ff2f00_u32.to_be_bytes())
            }
        }

        vec
    }
}