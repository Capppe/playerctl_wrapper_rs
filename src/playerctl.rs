use crate::playerctld::{DBusProxy, Methods, Signals};
use dbus::blocking::Connection;

use crate::dbus_utils;
use std::fmt::Debug;
use std::time::Duration;

pub struct PlayerCtl {
    pub interface: String,
    object_path: String,
    connection: Connection,
}

impl<'a> DBusProxy<'a> for PlayerCtl {
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

impl Signals for PlayerCtl {}

impl Methods for PlayerCtl {
    fn interface(&self) -> &str {
        &self.interface
    }
}

impl PlayerCtl {
    pub fn new() -> Result<Self, dbus::Error> {
        Ok(Self {
            interface: "com.github.altdesktop.playerctld".to_string(),
            object_path: "/org/mpris/MediaPlayer2".to_string(),
            connection: Connection::new_session()?,
        })
    }

    // Methods
    pub fn shift(&self) -> Result<String, String> {
        let proxy = self.get_proxy(None, None)?;

        let (new_player,): (String,) = proxy
            .method_call(&self.interface, "Shift", ())
            .map_err(|e| format!("Failed to shift player: {}", e))?;

        Ok(new_player)
    }

    pub fn unshift(&self) -> Result<String, String> {
        let proxy = self.get_proxy(None, None)?;

        let (new_player,): (String,) = proxy
            .method_call(&self.interface, "Unshift", ())
            .map_err(|e| format!("Failed to unshift player: {}", e))?;

        Ok(new_player)
    }
}

#[derive(Debug, Default)]
pub struct Property {
    name: String,
    property_type: PropertyType,
    read: bool,
    write: bool,
}

#[derive(Debug)]
pub enum PropertyType {
    StringArray(Vec<String>),
    String(String),
}

impl Default for PropertyType {
    fn default() -> Self {
        Self::String(String::from(""))
    }
}
