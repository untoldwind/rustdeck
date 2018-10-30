use device::StreamDeck;
use errors::Result;

pub fn listen() -> Result<()> {
    let stream_deck = StreamDeck::open()?;

    info!("Connected to stream deck");

    loop {
        let keys = stream_deck.wait_for_keys()?;

        info!(
            "1: {} 2: {} 3: {} 4: {} 5: {}",
            keys[0], keys[1], keys[2], keys[3], keys[4]
        );
        info!(
            "6: {} 7: {} 8: {} 9: {} 10: {}",
            keys[5], keys[6], keys[7], keys[8], keys[9]
        );
        info!(
            "11: {} 12: {} 13: {} 14: {} 15: {}",
            keys[10], keys[11], keys[12], keys[13], keys[14]
        );
    }
    Ok(())
}
