use dbus::tree::{DataType, Factory, MTFn};
use errors::Result;
use hidapi::HidApi;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::sync::Arc;

use dbus_server::DbusServer;
use device::{self, StreamDeck};

pub struct DaemonState {
    pub devices: HashMap<String, StreamDeck>,
}

impl DaemonState {
    fn new() -> Self {
        DaemonState {
            devices: Default::default(),
        }
    }
}

impl fmt::Debug for DaemonState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for device in self.devices.keys() {
            write!(f, "{}", device)?;
        }
        Ok(())
    }
}

pub struct Daemon {
    state: Arc<RefCell<DaemonState>>,
}

impl Daemon {
    pub fn run(&self) -> Result<()> {
        let mut hidapi = HidApi::new()?;
        let mut dbus_server = DbusServer::new(self.state.clone())?;

        loop {
            let changes = self.scan_devices(&mut hidapi)?;

            if changes {
                dbus_server.update_tree()?;
            }

            self.handle_keys();

            dbus_server.handle_messages();
        }

        Ok(())
    }

    fn scan_devices(&self, hidapi: &mut HidApi) -> Result<bool> {
        let mut state_ref = self.state.borrow_mut();
        let mut changes = false;
        let mut obsoletes: HashSet<String> = state_ref.devices.keys().cloned().collect();

        for (serial, device_info) in device::scan_devices(hidapi)? {
            obsoletes.remove(&serial);
            if state_ref.devices.contains_key(&serial) {
                continue;
            }
            info!("Adding device: {}", serial);
            let stream_deck = StreamDeck::new(device_info.open_device(hidapi)?)?;
            state_ref
                .devices
                .insert(stream_deck.serial.clone(), stream_deck);
            changes = true;
        }
        for obsolete in obsoletes {
            state_ref.devices.remove(&obsolete);
            changes = true;
        }

        Ok(changes)
    }

    fn handle_keys(&self) -> Result<()> {
        let state_ref = self.state.borrow();

        for device in state_ref.devices.values() {
            device.wait_for_keys(100)?;
        }

        Ok(())
    }
}

impl Default for Daemon {
    fn default() -> Self {
        Daemon {
            state: Arc::new(RefCell::new(DaemonState::new())),
        }
    }
}

impl DataType for Daemon {
    type Tree = Arc<RefCell<DaemonState>>;
    type ObjectPath = ();
    type Interface = ();
    type Property = ();
    type Method = Option<String>;
    type Signal = ();
}
