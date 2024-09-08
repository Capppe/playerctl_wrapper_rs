use std::time::Duration;

use dbus::blocking::{Connection, Proxy};

use crate::{
    dbus_utils,
    playerctld::{DBusProxy, Methods},
};

pub struct Peer {
    pub interface: String,
    object_path: String,
    connection: Connection,
}

impl<'a> DBusProxy<'a> for Peer {
    fn get_proxy(
        &'a self,
        dest: Option<&'a str>,
        object_path: Option<&'a str>,
    ) -> Result<dbus::blocking::Proxy<&Connection>, String> {
        let proxy = dbus_utils::create_proxy(
            dest,
            object_path.unwrap_or(&self.object_path),
            Duration::from_secs(5),
            &self.connection,
        )?;

        Ok(proxy)
    }
}

impl Methods for Peer {
    fn interface(&self) -> &str {
        &self.interface
    }
}

impl Peer {
    pub fn new() -> Result<Self, dbus::Error> {
        Ok(Self {
            interface: "org.freedesktop.DBus.Peer".to_string(),
            object_path: "/org/mpris/MediaPlayer2".to_string(),
            connection: Connection::new_session()?,
        })
    }
}
