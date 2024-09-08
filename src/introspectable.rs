use std::time::Duration;

use dbus::blocking::Connection;

use crate::{
    dbus_utils,
    playerctld::{DBusProxy, Methods},
};

pub struct Introspectable {
    pub interface: String,
    object_path: String,
    connection: Connection,
}

impl<'a> DBusProxy<'a> for Introspectable {
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

impl Methods for Introspectable {
    fn interface(&self) -> &str {
        &self.interface
    }
}

impl Introspectable {
    pub fn new() -> Result<Self, dbus::Error> {
        Ok(Self {
            interface: "org.freedesktop.DBus.Introspectable".to_string(),
            object_path: "/org/mpris/MediaPlayer2".to_string(),
            connection: Connection::new_session()?,
        })
    }
}
