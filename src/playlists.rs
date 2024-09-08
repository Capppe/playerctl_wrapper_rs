use std::time::Duration;

use dbus::{blocking::Connection, Path};

use crate::{
    dbus_utils,
    playerctl::Property,
    playerctld::{DBusProxy, Methods, Signals},
};

type PlaylistType<'a> = Vec<(Path<'a>, String, String)>;

pub struct Playlists {
    properties: Vec<Property>,
    interface: String,
    object_path: String,
    connection: Connection,
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

impl Methods for Playlists {
    fn interface(&self) -> &str {
        &self.interface
    }
}

impl Playlists {
    pub fn new() -> Result<Self, dbus::Error> {
        Ok(Self {
            properties: Vec::new(),
            interface: "org.mpris.MediaPlayer2.Playlists".to_string(),
            object_path: "/org/mpris/MediaPlayer2".to_string(),
            connection: Connection::new_session()?,
        })
    }

    // pub fn activate_playlists(&self, playlist_id: Path) -> Result<(), String> {
    //     let proxy = self.get_proxy(None, None)?;
    //
    //     proxy
    //         .method_call(&self.interface, "ActivatePlaylist", (playlist_id,))
    //         .map_err(|e| format!("Failed to activate playlist: {}", e))?;
    //
    //     Ok(())
    // }
    //
    // pub fn get_playlists(
    //     &self,
    //     index: u32,
    //     max_count: u32,
    //     order: String,
    //     reverse_order: bool,
    // ) -> Result<Vec<PlaylistType>, String> {
    //     let proxy = self.get_proxy(None, None)?;
    //
    //     let (playlists,): (Vec<PlaylistType>,) = proxy
    //         .method_call(
    //             &self.interface,
    //             "GetPlaylists",
    //             (index, max_count, order, reverse_order),
    //         )
    //         .map_err(|e| format!("Failed to get playlists: {}", e))?;
    //
    //     Ok(playlists)
    // }
}
