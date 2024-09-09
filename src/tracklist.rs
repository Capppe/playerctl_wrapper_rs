use std::{collections::HashMap, time::Duration};

use dbus::{
    arg::{PropMap, RefArg, Variant},
    blocking::Connection,
    Message, Path,
};
use tokio::sync::mpsc::Sender;

use crate::{
    dbus_utils::{self, parse_propmap},
    playerctld::{DBusItem, DBusProxy, Methods, Signals},
};

pub struct Tracklist {
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

    // Signals
    pub async fn track_added(
        &self,
        sender: Sender<HashMap<String, String>>,
        interface: Option<&str>,
    ) -> Result<(), String> {
        let (tx, mut rx) = tokio::sync::mpsc::channel::<Message>(100);

        tokio::spawn(async move {
            while let Some(message) = rx.recv().await {
                if let Ok((path, props, _arr)) = message.read_all::<(
                    String,
                    HashMap<String, Variant<Box<dyn RefArg>>>,
                    Vec<String>,
                )>() {
                    let mut msg = parse_propmap(&props);
                    msg.insert("Sender".to_owned(), path);

                    let _ = sender.send(msg).await;
                }
            }
        });

        let _ = self
            .start_listener(tx, interface.unwrap_or(self.get_interface()), "TrackAdded")
            .await;

        Ok(())
    }

    pub async fn track_list_replaced(
        &self,
        sender: Sender<(Vec<Path<'static>>, Path<'static>)>,
        interface: Option<&str>,
    ) -> Result<(), String> {
        let (tx, mut rx) = tokio::sync::mpsc::channel::<Message>(100);

        tokio::spawn(async move {
            while let Some(message) = rx.recv().await {
                if let Ok((path, props)) = message.read_all::<(Vec<Path<'static>>, Path<'static>)>()
                {
                    let _ = sender.send((path, props)).await;
                }
            }
        });

        let _ = self
            .start_listener(
                tx,
                interface.unwrap_or(self.get_interface()),
                "TrackListReplaced",
            )
            .await;

        Ok(())
    }

    pub async fn track_metadata_changed(
        &self,
        sender: Sender<(Path<'static>, PropMap)>,
        interface: Option<&str>,
    ) -> Result<(), String> {
        let (tx, mut rx) = tokio::sync::mpsc::channel::<Message>(100);

        tokio::spawn(async move {
            while let Some(message) = rx.recv().await {
                if let Ok((path, props)) = message.read_all::<(Path<'static>, PropMap)>() {
                    let _ = sender.send((path, props)).await;
                }
            }
        });

        let _ = self
            .start_listener(
                tx,
                interface.unwrap_or(self.get_interface()),
                "TrackMetadataChanged",
            )
            .await;

        Ok(())
    }

    pub async fn track_removed(
        &self,
        sender: Sender<(Path<'static>,)>,
        interface: Option<&str>,
    ) -> Result<(), String> {
        let (tx, mut rx) = tokio::sync::mpsc::channel::<Message>(100);

        tokio::spawn(async move {
            while let Some(message) = rx.recv().await {
                if let Ok((path,)) = message.read_all::<(Path<'static>,)>() {
                    let _ = sender.send((path,)).await;
                }
            }
        });

        let _ = self
            .start_listener(
                tx,
                interface.unwrap_or(self.get_interface()),
                "TrackRemoved",
            )
            .await;

        Ok(())
    }
}
