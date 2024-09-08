use std::time::Duration;

use dbus::{blocking::Connection, Path};

use crate::{
    dbus_utils,
    playerctl::Property,
    playerctld::{DBusItem, DBusProxy, Methods, Signals},
};

type PlaylistType<'a> = Vec<(Path<'a>, String, String)>;

pub struct Playlists {
    properties: Vec<Property>,
    interface: String,
    object_path: String,
    connection: Connection,
}

impl DBusItem for Playlists {
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

impl<'a> DBusProxy<'a> for Playlists {
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

impl Signals for Playlists {}

impl Methods for Playlists {}

impl Playlists {
    pub fn new() -> Result<Self, dbus::Error> {
        Ok(Self {
            properties: Vec::new(),
            interface: "org.mpris.MediaPlayer2.Playlists".to_string(),
            object_path: "/org/mpris/MediaPlayer2".to_string(),
            connection: Connection::new_session()?,
        })
    }

    // Methods
    pub fn activate_playlist(&self, playlist_id: Path) -> Result<(), String> {
        self.call_method_no_return("ActivatePlaylist", (playlist_id,))
    }

    pub fn get_playlists(
        &self,
        index: u32,
        max_count: u32,
        order: &str,
        reverse_order: bool,
    ) -> Result<Vec<PlaylistType>, String> {
        self.call_method("GetPlaylists", (index, max_count, order, reverse_order))
    }
}
