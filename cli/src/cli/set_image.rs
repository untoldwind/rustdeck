use device::StreamDeck;
use errors::Result;
use image;
use std::fs::File;
use std::io::BufReader;

pub fn set_image(key_index: u8, image_file_name: &str) -> Result<()> {
    let image_file = File::open(image_file_name)?;
    let reader = BufReader::new(&image_file);

    let image = image::load(reader, image::ImageFormat::PNG)?;

    let stream_deck = StreamDeck::open()?;

    info!("Connected to stream deck");

    stream_deck.set_image(key_index, image)?;

    Ok(())
}
