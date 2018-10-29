use errors::{ErrorKind, Result};

#[derive(Default, Clone, Copy)]
#[repr(packed)]
pub struct Color {
    pub blue: u8,
    pub green: u8,
    pub red: u8,
}

impl Color {
    #[allow(dead_code)]
    pub fn new(red: u8, green: u8, blue: u8) -> Color {
        Color { red, green, blue }
    }

    pub fn parse(color_str: &str) -> Result<Color> {
        let parts: Vec<u8> = color_str
            .split(',')
            .map(|p| p.parse::<u8>().unwrap_or(0))
            .collect();

        if parts.len() != 3 {
            return Err(ErrorKind::InvalidColorFormat.into());
        }

        Ok(Color {
            red: parts[0],
            green: parts[1],
            blue: parts[2],
        })
    }
}
