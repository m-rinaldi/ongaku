pub mod midi;

pub struct Music {
    chunks: Vec<MusicChunk>,
}

impl Music {
    pub fn new<I: IntoIterator<Item=MusicChunk>>(iter: I) -> Self {
        Music {
            chunks: iter.into_iter().collect(),
         }
    }
}
pub enum MusicChunk {
    Tone(u8),
    Rest(u8),
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
