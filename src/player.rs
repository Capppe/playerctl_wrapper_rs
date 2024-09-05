use std::time::Duration;

use crate::{dbus_utils, playerctl::Property, playerctld::DBusProxy};
use dbus::{blocking::Connection, Path};

pub struct Player {
    properties: Vec<Property>,
    interface: String,
    object_path: String,
    connection: Connection,
}

impl<'a> DBusProxy<'a> for Player {
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

impl Player {
    pub fn new() -> Result<Self, dbus::Error> {
        Ok(Self {
            properties: Vec::new(),
            interface: "org.mpris.MediaPlayer2.Player".to_string(),
            object_path: "/org/mpris/MediaPlayer2".to_string(),
            connection: Connection::new_session()?,
        })
    }

    pub fn next(&self) -> Result<(), String> {
        let proxy = self.get_proxy(None)?;

        proxy
            .method_call(&self.interface, "Next", ())
            .map_err(|e| format!("Failed to call method Next: {}", e))
    }

    fn open_uri(&self, uri: String) -> Result<(), String> {
        let proxy = self.get_proxy(None)?;

        proxy
            .method_call(&self.interface, "OpenUri", (uri,))
            .map_err(|e| format!("Failed to call method OpenUri: {}", e))
    }

    fn pause(&self) -> Result<(), String> {
        let proxy = self.get_proxy(None)?;

        proxy
            .method_call(&self.interface, "Pause", ())
            .map_err(|e| format!("Failed to call method Pause: {}", e))
    }

    fn play_pause(&self) -> Result<(), String> {
        let proxy = self.get_proxy(None)?;

        proxy
            .method_call(&self.interface, "PlayPause", ())
            .map_err(|e| format!("Failed to call method PlayPause: {}", e))
    }

    fn previous(&self) -> Result<(), String> {
        let proxy = self.get_proxy(None)?;

        proxy
            .method_call(&self.interface, "Previous", ())
            .map_err(|e| format!("Failed to call method Previous: {}", e))
    }

    fn seek(&self, offset: i64) -> Result<(), String> {
        let proxy = self.get_proxy(None)?;

        proxy
            .method_call(&self.interface, "Seek", (offset,))
            .map_err(|e| format!("Failed to call method Seek: {}", e))
    }

    fn set_position(&self, track_id: Path, offset: i64) -> Result<(), String> {
        let proxy = self.get_proxy(None)?;

        proxy
            .method_call(&self.interface, "SetPosition", (track_id, offset))
            .map_err(|e| format!("Failed to call method SetPosition: {}", e))
    }

    fn stop(&self) -> Result<(), String> {
        let proxy = self.get_proxy(None)?;

        proxy
            .method_call(&self.interface, "Stop", ())
            .map_err(|e| format!("Failed to call method Stop: {}", e))
    }
}
