use std::time::Duration;

use dbus::{arg::PropMap, blocking::Connection, Path};

use crate::{dbus_utils, playerctl::Property, playerctld::DBusProxy};

pub struct Tracklist {
    properties: Vec<Property>,
    interface: String,
    object_path: String,
    connection: Connection,
}

impl<'a> DBusProxy<'a> for Tracklist {
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

impl Tracklist {
    pub fn new() -> Result<Self, dbus::Error> {
        Ok(Self {
            properties: Vec::new(),
            interface: "org.mpris.MediaPlayer2.Tracklist".to_string(),
            object_path: "/org/mpris/MediaPlayer2".to_string(),
            connection: Connection::new_session()?,
        })
    }

    pub fn add_track(
        &self,
        uri: String,
        after_track: Path,
        set_as_current: bool,
    ) -> Result<(), String> {
        let proxy = self.get_proxy(None)?;

        proxy
            .method_call(
                &self.interface,
                "AddTrack",
                (uri, after_track, set_as_current),
            )
            .map_err(|e| format!("Failed to add track: {}", e))?;

        Ok(())
    }

    pub fn get_tracks_metadata(&self, track_ids: Vec<Path>) -> Result<Vec<PropMap>, String> {
        let proxy = self.get_proxy(None)?;

        let (metadata,): (Vec<PropMap>,) = proxy
            .method_call(&self.interface, "GetTracksMetadata", (track_ids,))
            .map_err(|e| format!("Failed to get tracks metadata: {}", e))?;

        Ok(metadata)
    }

    pub fn go_to(&self, track_id: Path) -> Result<(), String> {
        let proxy = self.get_proxy(None)?;

        proxy
            .method_call(&self.interface, "GetTracksMetadata", (track_id,))
            .map_err(|e| format!("Failed to get tracks metadata: {}", e))?;

        Ok(())
    }

    pub fn remove_track(&self, track_id: Path) -> Result<(), String> {
        let proxy = self.get_proxy(None)?;

        proxy
            .method_call(&self.interface, "RemoveTrack", (track_id,))
            .map_err(|e| format!("Failed to remove track: {}", e))?;

        Ok(())
    }
}
