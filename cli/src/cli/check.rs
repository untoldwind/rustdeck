use device::{StreamDeck, scan_devices};
use errors::Result;

pub fn check() -> Result<()> {
    for device_info in scan_devices()? {
        info!("Found: {:?}", device_info)
    }

    StreamDeck::open()?;

    info!("Connected to stream deck");

    Ok(())
}
