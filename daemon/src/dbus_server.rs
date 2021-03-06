use dbus::tree::{
    Factory, Interface, MTFn, MethodErr, MethodInfo, MethodResult, ObjectPath, Signal, Tree,
};
use dbus::{BusType, Connection, ErrorName, Message, NameFlag, Path};
use std::cell::RefCell;
use std::sync::Arc;

use daemon::{Daemon, DaemonState};
use device::{Color, KeyChange, StreamDeck};
use errors::Result;
use image::{self, ImageFormat};

const DBUS_NAME: &str = "io.github.rustdeck1";

pub struct DbusServer {
    connection: Connection,
    key_up_signal: Arc<Signal<Daemon>>,
    key_down_signal: Arc<Signal<Daemon>>,
    devices_interface: Arc<Interface<MTFn<Daemon>, Daemon>>,
    devices_object: Arc<ObjectPath<MTFn<Daemon>, Daemon>>,
    tree: Arc<Tree<MTFn<Daemon>, Daemon>>,
    factory: Factory<MTFn<Daemon>, Daemon>,
    state: Arc<RefCell<DaemonState>>,
}

impl DbusServer {
    pub fn new(state: Arc<RefCell<DaemonState>>) -> Result<DbusServer> {
        let connection = Connection::get_private(BusType::System)?;
        let factory = Factory::new_fn::<Daemon>();
        let key_up_signal = Arc::new(
            factory
                .signal("key_up", ())
                .sarg::<&str, _>("serial")
                .sarg::<u8, _>("key"),
        );
        let key_down_signal = Arc::new(
            factory
                .signal("key_down", ())
                .sarg::<&str, _>("serial")
                .sarg::<u8, _>("key"),
        );
        let devices_interface = Arc::new(
            factory
                .interface("io.github.rustdeck1.Devices", None)
                .add_m(
                    factory
                        .method("ListDevices", (), Self::list_devices)
                        .outarg::<Vec<Path>, _>("devices"),
                ).add_s(key_up_signal.clone())
                .add_s(key_down_signal.clone()),
        );
        let devices_object = Arc::new(
            factory
                .object_path("/devices", ())
                .introspectable()
                .add(devices_interface.clone()),
        );
        let tree = Arc::new(factory.tree(state.clone()).add(devices_object.clone()));

        connection.register_name(DBUS_NAME, NameFlag::ReplaceExisting as u32)?;
        tree.set_registered(&connection, true)?;

        connection.add_handler(tree.clone());

        Ok(DbusServer {
            connection,
            factory,
            tree,
            key_up_signal,
            key_down_signal,
            devices_interface,
            devices_object,
            state,
        })
    }

    pub fn update_tree(&mut self) -> Result<()> {
        self.connection.extract_handler();
        self.tree.set_registered(&self.connection, false)?;

        let state_ref = self.state.borrow();

        let mut tree = self
            .factory
            .tree(self.state.clone())
            .add(self.devices_object.clone());
        for serial in state_ref.devices.keys() {
            tree = tree.add(self.create_derive_object(serial));
        }

        self.tree = Arc::new(tree);
        self.tree.set_registered(&self.connection, true)?;
        self.connection.add_handler(self.tree.clone());

        Ok(())
    }

    fn create_derive_object(&self, serial: &String) -> ObjectPath<MTFn<Daemon>, Daemon> {
        self.factory
            .object_path(format!("/devices/{}", serial), ())
            .introspectable()
            .add(
                self.factory
                    .interface("io.github.rustdeck1.Device", Some(serial.clone()))
                    .add_m(
                        self.factory
                            .method("GetSerial", (), Self::get_serial)
                            .outarg::<&str, _>("serial"),
                    ).add_m(
                        self.factory
                            .method("FillColor", (), Self::fill_color)
                            .inarg::<u8, _>("key")
                            .inarg::<u8, _>("red")
                            .inarg::<u8, _>("green")
                            .inarg::<u8, _>("blue"),
                    ).add_m(
                        self.factory
                            .method("SetImage", (), Self::set_image)
                            .inarg::<u8, _>("key")
                            .inarg::<&str, _>("format")
                            .inarg::<&[u8], _>("data"),
                    ),
            )
    }

    pub fn send_key_changes(&self, key_changes: Vec<(String, KeyChange)>) -> Result<()> {
        for (serial, key_change) in key_changes {
            match key_change {
                KeyChange::Up(key) => {
                    let msg = self.key_up_signal.emit(
                        self.devices_object.get_name(),
                        self.devices_interface.get_name(),
                        &[serial.as_str().into(), key.into()],
                    );
                    self.connection.send(msg);
                }
                KeyChange::Down(key) => {
                    let msg = self.key_down_signal.emit(
                        self.devices_object.get_name(),
                        self.devices_interface.get_name(),
                        &[serial.as_str().into(), key.into()],
                    );
                    self.connection.send(msg);
                }
            }
        }

        Ok(())
    }

    pub fn handle_messages(&self) {
        self.connection.incoming(100).next();
    }

    fn list_devices(m: &MethodInfo<MTFn<Daemon>, Daemon>) -> MethodResult {
        let mut result: Vec<Path> = Default::default();

        for serial in m.tree.get_data().borrow().devices.keys() {
            result.push(format!("/devices/{}", serial).into());
        }

        let mret = m.msg.method_return().append(result.as_slice());

        Ok(vec![mret])
    }

    fn get_serial(m: &MethodInfo<MTFn<Daemon>, Daemon>) -> MethodResult {
        if let Some(serial) = m.iface.get_data() {
            let mret = m.msg.method_return().append(serial.as_str());
            Ok(vec![mret])
        } else {
            Err(MethodErr::failed(&"No serial"))
        }
    }

    fn for_device<F>(m: &MethodInfo<MTFn<Daemon>, Daemon>, f: F) -> MethodResult
    where
        F: FnOnce(&StreamDeck) -> Result<()>,
    {
        let serial = m
            .iface
            .get_data()
            .as_ref()
            .ok_or(MethodErr::failed(&"No serial"))?;
        let state_ref = m.tree.get_data().borrow();
        let device = state_ref
            .devices
            .get(serial)
            .ok_or(MethodErr::failed(&"Invalid serial"))?;

        f(device).map_err(|err| {
                error!("{:?}", err);
                MethodErr::failed(&"Internal error")
            })?;

        let mret = m.msg.method_return();

        Ok(vec![mret])

    }

    fn fill_color(m: &MethodInfo<MTFn<Daemon>, Daemon>) -> MethodResult {
        let (key, red, green, blue) = m.msg.read4::<u8, u8, u8, u8>()?;
        Self::for_device(m, |device| {
            device.set_color(key, Color { red, green, blue })

        })
    }

    fn set_image(m: &MethodInfo<MTFn<Daemon>, Daemon>) -> MethodResult {
        let (key, format, data) = m.msg.read3::<u8, &str, &[u8]>()?;
        let image_format = match format {
            "png" => ImageFormat::PNG,
            "pnm" => ImageFormat::PNM,
            "jpg" => ImageFormat::JPEG,
            "gif" => ImageFormat::GIF,
            "ico" => ImageFormat::ICO,
            _ => return Err(MethodErr::invalid_arg(&"Invalid image format")),
        };
        Self::for_device(m, |device| {
            let image = image::load_from_memory(data)?;

            device.set_image(key, image)
        })
    }
}
