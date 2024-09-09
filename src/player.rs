use std::time::Duration;

use crate::{
    dbus_utils,
    playerctld::{DBusItem, DBusProxy, Methods, Properties, Signals},
};
use dbus::{blocking::Connection, Message, Path};
use tokio::sync::mpsc::Sender;

pub struct Player {
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

impl Properties for Player {}

impl Player {
    pub fn new() -> Result<Self, dbus::Error> {
        Ok(Self {
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

    pub fn play(&self) -> Result<(), String> {
        self.call_method_no_return("Play", ())
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

    // Signals
    pub async fn seeked(
        &self,
        sender: Sender<(i64,)>,
        interface: Option<&str>,
    ) -> Result<(), String> {
        let (tx, mut rx) = tokio::sync::mpsc::channel::<Message>(100);

        tokio::spawn(async move {
            while let Some(message) = rx.recv().await {
                if let Ok(prop) = message.read_all::<(i64,)>() {
                    let _ = sender.send(prop).await;
                }
            }
        });

        let _ = self
            .start_listener(tx, interface.unwrap_or(self.get_interface()), "Seeked")
            .await;

        Ok(())
    }
}
