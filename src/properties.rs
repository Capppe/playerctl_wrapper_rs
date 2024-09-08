use std::time::Duration;

use dbus::blocking::Connection;

use crate::{
    dbus_utils,
    playerctld::{DBusProxy, Methods, Signals},
};

pub struct Properties {
    pub interface: String,
    object_path: String,
    connection: Connection,
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

impl Methods for Properties {
    fn interface(&self) -> &str {
        &self.interface
    }
}

impl Properties {
    pub fn new() -> Result<Self, dbus::Error> {
        Ok(Self {
            interface: "org.freedesktop.DBus.Properties".to_string(),
            object_path: "/org/mpris/MediaPlayer2".to_string(),
            connection: Connection::new_session()?,
        })
    }

    pub fn get(&self, interface_name: String, property_name: String) -> Result<String, String> {
        let proxy = self
            .get_proxy(None, None)
            .map_err(|e| format!("Failed to create a proxy: {}", e))?;

        let (property,): (String,) = proxy
            .method_call(&self.interface, "Get", (interface_name, property_name))
            .map_err(|e| format!("Failed to get property: {}", e))?;

        Ok(property)
    }

    pub fn get_all(&self, interface_name: String) -> Result<Vec<String>, String> {
        let proxy = self
            .get_proxy(None, None)
            .map_err(|e| format!("Failed to create a proxy: {}", e))?;

        let (properties,): (Vec<String>,) = proxy
            .method_call(&self.interface, "GetAll", (interface_name,))
            .map_err(|e| format!("Failed to get all properties: {}", e))?;

        Ok(properties)
    }

    pub fn set(
        &self,
        interface_name: String,
        property_name: String,
        value: String,
    ) -> Result<(), String> {
        let proxy = self
            .get_proxy(None, None)
            .map_err(|e| format!("Failed to create a proxy: {}", e))?;

        proxy
            .method_call(
                &self.interface,
                "Set",
                (interface_name, property_name, value),
            )
            .map_err(|e| format!("Failed to set property: {}", e))?;

        Ok(())
    }
}
