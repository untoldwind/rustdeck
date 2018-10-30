use dbus::tree::{Factory, MTFn, MethodInfo, MethodResult, Signal, Tree};
use dbus::{BusType, Connection, Message, NameFlag, Path};
use std::cell::RefCell;
use std::sync::Arc;

use daemon::{Daemon, DaemonState};
use errors::Result;

const DBUS_NAME: &str = "io.github.rustdeck1";

pub struct DbusServer {
    connection: Connection,
    signal: Arc<Signal<Daemon>>,
    tree: Arc<Tree<MTFn<Daemon>, Daemon>>,
    factory: Factory<MTFn<Daemon>, Daemon>,
    state: Arc<RefCell<DaemonState>>,
}

impl DbusServer {
    pub fn new(state: Arc<RefCell<DaemonState>>) -> Result<DbusServer> {
        let connection = Connection::get_private(BusType::System)?;
        let factory = Factory::new_fn::<Daemon>();
        let signal = Arc::new(factory.signal("key", ()));
        let tree = Self::make_tree(&factory, signal.clone(), state.clone());

        connection.register_name(DBUS_NAME, NameFlag::ReplaceExisting as u32)?;
        tree.set_registered(&connection, true)?;

        connection.add_handler(tree.clone());

        Ok(DbusServer {
            connection,
            factory,
            tree,
            signal,
            state,
        })
    }

    pub fn update_tree(&mut self) -> Result<()> {
        self.connection.extract_handler();
        self.tree.set_registered(&self.connection, false)?;

        self.tree = Self::make_tree(&self.factory, self.signal.clone(), self.state.clone());
        self.tree.set_registered(&self.connection, true)?;
        self.connection.add_handler(self.tree.clone());

        Ok(())
    }

    pub fn handle_messages(&self) {
        self.connection.incoming(100).next();
    }

    fn make_tree(
        factory: &Factory<MTFn<Daemon>, Daemon>,
        signal: Arc<Signal<Daemon>>,
        state: Arc<RefCell<DaemonState>>,
    ) -> Arc<Tree<MTFn<Daemon>, Daemon>> {
        Arc::new(
            factory.tree(state).add(
                factory.object_path("/devices", ()).introspectable().add(
                    factory
                        .interface("io.github.rustdeck1.Devices", ())
                        .add_m(
                            factory
                                .method("ListDevices", None, Self::list_devices)
                                .outarg::<Vec<Path>, _>("devices"),
                        ).add_s(signal.clone()),
                ),
            ),
        )
    }

    fn list_devices(m: &MethodInfo<MTFn<Daemon>, Daemon>) -> MethodResult {
        let mut result : Vec<Path> = Default::default();

        for serial in m.tree.get_data().borrow().devices.keys() {
            result.push(format!("/devices/{}", serial).into());
        }

        let mret = m.msg.method_return().append(result.as_slice());


        Ok(vec![mret])
    }
}
