use std::time::Duration;

use dbus::{
    arg::{Append, Variant},
    blocking::Connection,
};

use crate::{
    dbus_utils,
    playerctld::{DBusItem, DBusProxy, Methods, Signals},
};

pub struct Properties {
    pub interface: String,
    object_path: String,
    connection: Connection,
}

impl DBusItem for Properties {
    fn get_interface(&self) -> &str {
        &self.interface
    }

    fn get_object_path(&self) -> &str {
        &self.object_path
    }

    fn get_connection(&self) -> &Connection {
        &self.connection
    }
}

impl<'a> DBusProxy<'a> for Properties {
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

impl Signals for Properties {}

impl Methods for Properties {}

impl Properties {
    pub fn new() -> Result<Self, dbus::Error> {
        Ok(Self {
            interface: "org.freedesktop.DBus.Properties".to_string(),
            object_path: "/org/mpris/MediaPlayer2".to_string(),
            connection: Connection::new_session()?,
        })
    }

    // Methods
    pub fn get(&self, interface_name: &str, property_name: &str) -> Result<String, String> {
        self.call_method("Get", (interface_name, property_name))
    }

    pub fn get_all(&self, interface_name: &str) -> Result<Vec<String>, String> {
        self.call_method("GetAll", (interface_name,))
    }

    pub fn set<T>(
        &self,
        interface_name: &str,
        property_name: &str,
        value: Variant<T>,
    ) -> Result<(), String>
    where
        T: Append + dbus::arg::Arg,
    {
        self.call_method_no_return("Set", (interface_name, property_name, value))
    }
}
