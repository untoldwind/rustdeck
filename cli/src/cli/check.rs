use device::StreamDeck;
use errors::Result;

pub fn check() -> Result<()> {
    StreamDeck::open()?;

    info!("Connected to stream deck");

    Ok(())
}
