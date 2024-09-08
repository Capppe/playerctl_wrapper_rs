use std::time::Duration;

use dbus::{arg::PropMap, blocking::Connection, Path};

use crate::{
    dbus_utils,
    playerctl::Property,
    playerctld::{DBusItem, DBusProxy, Methods, Signals},
};

pub struct Tracklist {
    properties: Vec<Property>,
    interface: String,
    object_path: String,
    connection: Connection,
}

impl DBusItem for Tracklist {
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

impl<'a> DBusProxy<'a> for Tracklist {
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

impl Signals for Tracklist {}

impl Methods for Tracklist {}

impl Tracklist {
    pub fn new() -> Result<Self, dbus::Error> {
        Ok(Self {
            properties: Vec::new(),
            interface: "org.mpris.MediaPlayer2.Tracklist".to_string(),
            object_path: "/org/mpris/MediaPlayer2".to_string(),
            connection: Connection::new_session()?,
        })
    }

    // Methods
    pub fn add_track(
        &self,
        uri: &str,
        after_track: Path,
        set_as_current: bool,
    ) -> Result<(), String> {
        self.call_method_no_return("AddTrack", (uri, after_track, set_as_current))
    }

    pub fn get_tracks_metadata(&self, track_ids: Vec<Path>) -> Result<Vec<PropMap>, String> {
        self.call_method("GetTracksMetadata", (track_ids,))
    }

    pub fn go_to(&self, track_id: Path) -> Result<(), String> {
        self.call_method_no_return("GoTo", (track_id,))
    }

    pub fn remove_track(&self, track_id: Path) -> Result<(), String> {
        self.call_method_no_return("RemoveTrack", (track_id,))
    }
}
