use crate::playerctld::{DBusItem, DBusProxy, Methods, Signals};
use dbus::blocking::Connection;

pub struct PlayerCtl {
    interface: String,
    object_path: String,
    connection: Connection,
}

impl DBusItem for PlayerCtl {
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

impl<'a> DBusProxy<'a> for PlayerCtl {}

impl Signals for PlayerCtl {}

impl Methods for PlayerCtl {}

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
        self.call_method("Shift", ())
    }

    pub fn unshift(&self) -> Result<String, String> {
        self.call_method("Unshift", ())
    }
}
