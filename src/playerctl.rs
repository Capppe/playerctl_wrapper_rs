use std::collections::HashMap;

use crate::playerctld::{DBusItem, DBusProxy, Methods, Signals};
use dbus::{blocking::Connection, Message};
use tokio::sync::mpsc::Sender;

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

    // Signals
    pub async fn active_player_change_begin(
        &self,
        sender: Sender<(String,)>,
        interface: Option<&str>,
    ) -> Result<(), String> {
        let (tx, mut rx) = tokio::sync::mpsc::channel::<Message>(100);

        tokio::spawn(async move {
            while let Some(message) = rx.recv().await {
                if let Ok(prop) = message.read_all::<(String,)>() {
                    let _ = sender.send(prop).await;
                }
            }
        });

        let _ = self
            .start_listener(
                tx,
                interface.unwrap_or(self.get_interface()),
                "ActivePlayerChangeBegin",
            )
            .await;

        Ok(())
    }

    pub async fn active_player_change_end(
        &self,
        sender: Sender<(String,)>,
        interface: Option<&str>,
    ) -> Result<(), String> {
        let (tx, mut rx) = tokio::sync::mpsc::channel::<Message>(100);

        tokio::spawn(async move {
            while let Some(message) = rx.recv().await {
                if let Ok(prop) = message.read_all::<(String,)>() {
                    let _ = sender.send(prop).await;
                }
            }
        });

        let _ = self
            .start_listener(
                tx,
                interface.unwrap_or(self.get_interface()),
                "ActivePlayerChangeEnd",
            )
            .await;

        Ok(())
    }
}
