use dbus::blocking::Connection;

use crate::dbus_utils;
use crate::introspectable::Introspectable;
use crate::media_player::MediaPlayer;
use crate::peer::Peer;
use crate::player::Player;
use crate::playlists::Playlists;
use crate::properties::Properties;
use crate::tracklist::Tracklist;
use std::fmt::Debug;
use std::time::Duration;

pub trait DBusProxy<'a> {
    fn get_proxy(
        &'a self,
        object_path: Option<&'a str>,
    ) -> Result<dbus::blocking::Proxy<&Connection>, String>;
}

pub struct PlayerCtld {
    pub playerctl: PlayerCtl,
    pub introspectable: Introspectable,
    pub peer: Peer,
    pub properties: Properties,
    pub media_player: MediaPlayer,
    pub player: Player,
    pub playlists: Playlists,
    pub tracklist: Tracklist,
}

impl PlayerCtld {
    pub fn new() -> Result<Self, dbus::Error> {
        Ok(Self {
            playerctl: PlayerCtl::new()?,
            introspectable: Introspectable::new()?,
            peer: Peer::new()?,
            properties: Properties::new()?,
            media_player: MediaPlayer::new()?,
            player: Player::new()?,
            playlists: Playlists::new()?,
            tracklist: Tracklist::new()?,
        })
    }
}

pub struct PlayerCtl {
    properties: Vec<Property>,
    interface: String,
    object_path: String,
    connection: Connection,
}

impl<'a> DBusProxy<'a> for PlayerCtl {
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

impl PlayerCtl {
    pub fn new() -> Result<Self, dbus::Error> {
        Ok(Self {
            properties: Vec::new(),
            interface: "com.github.altdesktop.playerctld".to_string(),
            object_path: "/org/mpris/MediaPlayer2".to_string(),
            connection: Connection::new_session()?,
        })
    }

    pub fn shift(&self) -> Result<String, String> {
        let proxy = self.get_proxy(None)?;

        let (new_player,): (String,) = proxy
            .method_call(&self.interface, "Shift", ())
            .map_err(|e| format!("Failed to shift player: {}", e))?;

        Ok(new_player)
    }

    pub fn unshift(&self) -> Result<String, String> {
        let proxy = self.get_proxy(None)?;

        let (new_player,): (String,) = proxy
            .method_call(&self.interface, "Unshift", ())
            .map_err(|e| format!("Failed to unshift player: {}", e))?;

        Ok(new_player)
    }
}

pub struct Method {
    name: String,
    args: Option<()>,
    returns: Option<()>,
    handler: Box<dyn Fn() -> String>,
}

#[derive(Debug, Default)]
pub struct Property {
    name: String,
    property_type: PropertyType,
    read: bool,
    write: bool,
}

#[derive(Debug)]
pub enum PropertyType {
    StringArray(Vec<String>),
    String(String),
}

impl Default for PropertyType {
    fn default() -> Self {
        Self::String(String::from(""))
    }
}
