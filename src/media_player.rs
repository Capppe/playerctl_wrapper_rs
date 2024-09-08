use std::time::Duration;

use dbus::blocking::Connection;

use crate::{
    dbus_utils,
    playerctl::Property,
    playerctld::{DBusItem, DBusProxy, Methods},
};

pub struct MediaPlayer {
    properties: Vec<Property>,
    interface: String,
    object_path: String,
    connection: Connection,
}

impl DBusItem for MediaPlayer {
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

impl<'a> DBusProxy<'a> for MediaPlayer {
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

impl Methods for MediaPlayer {}

impl MediaPlayer {
    pub fn new() -> Result<Self, dbus::Error> {
        Ok(Self {
            properties: Vec::new(),
            interface: "org.mpris.MediaPlayer2".to_string(),
            object_path: "/org/mpris/MediaPlayer2".to_string(),
            connection: Connection::new_session()?,
        })
    }

    // Methods
    pub fn quit(&self) -> Result<(), String> {
        self.call_method_no_return("Quit", ())
    }

    pub fn raise(&self) -> Result<(), String> {
        self.call_method_no_return("Raise", ())
    }
}
