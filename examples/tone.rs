use ongaku::MusicChunk;


use ongaku::Music;
fn main() {
    use MusicChunk::*;

    let arr = [Tone(2), Rest(1), Tone(3), Rest(1)];
    let iter = arr.into_iter();

    let music = Music::new(
        iter.cycle().take(100)
    );

    use ongaku::midi::Midi;

    let midi = Midi::new(music);
    midi.to_file("Midi.mid".as_ref());
}