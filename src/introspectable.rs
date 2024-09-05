use std::time::Duration;

use dbus::blocking::Connection;

use crate::{dbus_utils, playerctld::DBusProxy};

pub struct Introspectable {
    interface: String,
    object_path: String,
    connection: Connection,
}

impl<'a> DBusProxy<'a> for Introspectable {
    fn get_proxy(
        &'a self,
        object_path: Option<&'a str>,
    ) -> Result<dbus::blocking::Proxy<&Connection>, String> {
        let proxy = dbus_utils::create_proxy(
            None,
            object_path.unwrap_or(&self.object_path),
            Duration::from_secs(5),
            &self.connection,
        )?;

        Ok(proxy)
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

    pub fn introspect(&self) -> Result<String, String> {
        let proxy = self.get_proxy(None)?;

        let (xml_data,): (String,) = proxy
            .method_call(&self.interface, "Introspect", ())
            .map_err(|e| format!("Failed to introspect: {}", e))?;

        Ok(xml_data)
    }
}
