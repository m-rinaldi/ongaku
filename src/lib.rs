mod midi;

struct Music {
    chunks: Vec<MusicChunk>,
}
pub enum MusicChunk {
    Tone,
    Rest,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
