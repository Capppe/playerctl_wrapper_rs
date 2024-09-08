use std::time::Duration;

use crate::{
    dbus_utils,
    playerctl::Property,
    playerctld::{DBusItem, DBusProxy, Methods, Signals},
};
use dbus::{blocking::Connection, Path};

pub struct Player {
    properties: Vec<Property>,
    interface: String,
    object_path: String,
    connection: Connection,
}

impl DBusItem for Player {
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

impl<'a> DBusProxy<'a> for Player {
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

impl Signals for Player {}

impl Methods for Player {}

impl Player {
    pub fn new() -> Result<Self, dbus::Error> {
        Ok(Self {
            properties: Vec::new(),
            interface: "org.mpris.MediaPlayer2.Player".to_string(),
            object_path: "/org/mpris/MediaPlayer2".to_string(),
            connection: Connection::new_session()?,
        })
    }

    // Methods
    pub fn next(&self) -> Result<(), String> {
        self.call_method_no_return("Next", ())
    }

    pub fn open_uri(&self, uri: &str) -> Result<(), String> {
        self.call_method_no_return("OpenUri", (uri,))
    }

    pub fn pause(&self) -> Result<(), String> {
        self.call_method_no_return("Pause", ())
    }

    pub fn play_pause(&self) -> Result<(), String> {
        self.call_method_no_return("PlayPause", ())
    }

    pub fn previous(&self) -> Result<(), String> {
        self.call_method_no_return("Previous", ())
    }

    pub fn seek(&self, offset: i64) -> Result<(), String> {
        self.call_method_no_return("Seek", (offset,))
    }

    pub fn set_position(&self, track_id: Path, offset: i64) -> Result<(), String> {
        self.call_method_no_return("SetPosition", (track_id, offset))
    }

    pub fn stop(&self) -> Result<(), String> {
        self.call_method_no_return("Stop", ())
    }
}
