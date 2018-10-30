use dbus::{Connection, BusType};
use dbus::tree::Factory;

use errors::Result;
use daemon::Daemon;

const DBUS_NAME: &str = "io.github.rustdeck1";

pub struct DbusServer {
    connection: Connection,
    daemon: &'static Daemon,
}

impl DbusServer {
    pub fn new(daemon: &'static Daemon) -> Result<DbusServer> {
        let connection = Connection::get_private(BusType::System)?;
        Ok(DbusServer {
            connection,
            daemon,
        })
    }


}