use errors::{Error, ErrorKind, Result};
use hidapi::HidDevice;
use image::{DynamicImage, FilterType, GenericImageView};

use super::color::Color;
use super::key_change::KeyChange;

const NUM_KEYS: usize = 15;
const PAGE_PACKET_SIZE: usize = 8191;
const ICON_SIZE: usize = 72;
const NUM_FIRST_PAGE_PIXELS: usize = 2583;
const NUM_SECOND_PAGE_PIXELS: usize = 2601;

const HEADER_PAGE1: &[u8] = &[
    0x02, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x42, 0x4d, 0xf6, 0x3c, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x36, 0x00, 0x00, 0x00, 0x28, 0x00,
    0x00, 0x00, 0x48, 0x00, 0x00, 0x00, 0x48, 0x00, 0x00, 0x00, 0x01, 0x00, 0x18, 0x00, 0x00, 0x00,
    0x00, 0x00, 0xc0, 0x3c, 0x00, 0x00, 0xc4, 0x0e, 0x00, 0x00, 0xc4, 0x0e, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

const HEADER_PAGE2: &[u8] = &[
    0x02, 0x01, 0x02, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

pub struct StreamDeck {
    pub serial: String,
    device: HidDevice,
    last_key_state: [bool; NUM_KEYS],
}

impl StreamDeck {
    pub fn new(device: HidDevice) -> Result<StreamDeck> {
        let serial = device
            .get_serial_number_string()?
            .ok_or(Error::from(ErrorKind::NoSerial))?;

        device.set_blocking_mode(false)?;
        Ok(StreamDeck {
            serial,
            device,
            last_key_state: [false; NUM_KEYS],
        })
    }

    pub fn set_color(&self, key_index: u8, color: Color) -> Result<()> {
        let mut pixels = [0u8; ICON_SIZE * ICON_SIZE * 3];

        for i in 0..(ICON_SIZE * ICON_SIZE) {
            pixels[3 * i] = color.blue;
            pixels[3 * i + 1] = color.green;
            pixels[3 * i + 2] = color.red;
        }

        self.write_pixels(key_index, &pixels)
    }

    pub fn set_image(&self, key_index: u8, mut image: DynamicImage) -> Result<()> {
        let (width, height) = image.dimensions();
        if width != ICON_SIZE as u32 || height != ICON_SIZE as u32 {
            image = image.resize_to_fill(ICON_SIZE as u32, ICON_SIZE as u32, FilterType::Triangle)
        }
        let rgb_image = image.to_rgb();

        let mut pixels = [0u8; ICON_SIZE * ICON_SIZE * 3];

        for (x, y, pixel) in rgb_image.enumerate_pixels() {
            let offset = (y as usize + 1) * ICON_SIZE * 3 - (x as usize + 1) * 3;

            pixels[offset] = pixel[2];
            pixels[offset + 1] = pixel[1];
            pixels[offset + 2] = pixel[0];
        }

        self.write_pixels(key_index, &pixels)
    }

    pub fn wait_for_keys(&self, millis: u32) -> Result<[bool; NUM_KEYS]> {
        let mut packet = [0u8; PAGE_PACKET_SIZE];

        self.device.read_timeout(&mut packet, millis as i32)?;

        let mut result = [false; NUM_KEYS];

        for i in 0..NUM_KEYS {
            result[i] = packet[i + 1] != 0u8;
        }

        Ok(result)
    }

    pub fn wait_for_key_changes(&mut self) -> Result<Vec<KeyChange>> {
        let mut packet = [0u8; PAGE_PACKET_SIZE];

        if self.device.read(&mut packet)? == 0 {
            return Ok(vec![]);
        }

        let mut result = Vec::new();

        for i in 0..NUM_KEYS {
            if !self.last_key_state[i] && packet[i + 1] != 0u8 {
                result.push(KeyChange::Down(i as u8));
                self.last_key_state[i] = true;
            } else if self.last_key_state[i] && packet[i + 1] == 0u8 {
                result.push(KeyChange::Up(i as u8));
                self.last_key_state[i] = false;
            }
        }

        Ok(result)
    }

    fn write_pixels(&self, key_index: u8, pixels: &[u8]) -> Result<()> {
        self.write_page1(key_index, &pixels[0..(NUM_FIRST_PAGE_PIXELS * 3)])?;
        self.write_page2(
            key_index,
            &pixels[(NUM_FIRST_PAGE_PIXELS * 3)
                        ..((NUM_FIRST_PAGE_PIXELS + NUM_SECOND_PAGE_PIXELS) * 3)],
        )?;

        Ok(())
    }

    fn write_page1(&self, key_index: u8, buffer: &[u8]) -> Result<()> {
        let mut packet = [0u8; PAGE_PACKET_SIZE];

        packet[0..HEADER_PAGE1.len()].clone_from_slice(HEADER_PAGE1);
        packet[5] = key_index + 1;
        packet[HEADER_PAGE1.len()..(HEADER_PAGE1.len() + buffer.len())].clone_from_slice(buffer);

        self.device.write(&packet)?;

        Ok(())
    }

    fn write_page2(&self, key_index: u8, buffer: &[u8]) -> Result<()> {
        let mut packet = [0u8; PAGE_PACKET_SIZE];

        packet[0..HEADER_PAGE2.len()].clone_from_slice(HEADER_PAGE2);
        packet[5] = key_index + 1;
        packet[HEADER_PAGE2.len()..(HEADER_PAGE2.len() + buffer.len())].clone_from_slice(buffer);

        self.device.write(&packet)?;

        Ok(())
    }
}
