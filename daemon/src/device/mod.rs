use errors::Result;
use hidapi::{HidApi, HidDeviceInfo};

mod color;
mod stream_deck;

pub use self::stream_deck::StreamDeck;

const VENDOR_ID: u16 = 0x0fd9;
const PRODUCT_ID: u16 = 0x0060;

pub fn scan_devices(hidapi: &mut HidApi) -> Result<Vec<(String, HidDeviceInfo)>> {
    hidapi.refresh_devices()?;

    Ok(hidapi
        .devices()
        .iter()
        .filter(|device_info| {
            device_info.vendor_id == VENDOR_ID && device_info.product_id == PRODUCT_ID
        }).flat_map(|device_info| match &device_info.serial_number {
            Some(serial) => Some((serial.clone(), device_info.clone())),
            None => None,
        }).collect())
}
