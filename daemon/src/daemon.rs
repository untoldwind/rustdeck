use dbus::tree::{Factory, DataType, MTFn};
use errors::Result;
use hidapi::HidApi;
use std::collections::HashMap;
use std::fmt;

use device::StreamDeck;

lazy_static! {
    static ref daemon : Daemon = Daemon::new();
}

pub struct Daemon {
    devices: HashMap<String, StreamDeck>,
}

impl Daemon {
    fn new() -> Self {
        Daemon {
            devices: HashMap::new(),
        }
    }

}

impl Default for &'static Daemon {
    fn default() -> Self {
        &daemon
    }
}

impl fmt::Debug for Daemon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for device in self.devices.keys() {
            write!(f, "{}", device)?;
        }
        Ok(())
    }
}

impl DataType for &'static Daemon {
    type Tree = ();
    type ObjectPath = ();
    type Interface = ();
    type Property = ();
    type Method = &'static Daemon;
    type Signal = ();
}

unsafe impl Sync for Daemon {}

pub fn daemon_loop() -> Result<()> {
    Ok(())
}
