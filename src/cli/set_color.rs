use device::{StreamDeck, Color};
use errors::Result;

pub fn set_color(key_index: u8, color: Color) -> Result<()> {
    let stream_deck = StreamDeck::open()?;

    info!("Connected to stream deck");

    stream_deck.set_color(key_index, color)?;
    
    Ok(())
}
