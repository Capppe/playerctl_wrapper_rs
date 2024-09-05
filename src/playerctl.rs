use crate::playerctld::{DBusProxy, Methods, Signals};
use dbus::arg::{self, ReadAll};
use dbus::blocking::Connection;
use tokio::sync::mpsc::Sender;

use crate::dbus_utils;
use std::fmt::Debug;
use std::time::Duration;

pub struct PlayerCtl {
    properties: Vec<Property>,
    interface: String,
    object_path: String,
    connection: Connection,
}

// Signals
struct ActivePlayerChangeBegin {
    sender: String,
}

impl arg::AppendAll for ActivePlayerChangeBegin {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.sender, i);
    }
}

impl arg::ReadAll for ActivePlayerChangeBegin {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(ActivePlayerChangeBegin { sender: i.read()? })
    }
}

impl dbus::message::SignalArgs for ActivePlayerChangeBegin {
    const NAME: &'static str = "ActivePlayerChangeBegin";
    const INTERFACE: &'static str = "com.github.altdesktop.playerctld";
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

impl Signals for PlayerCtl {}

impl Methods for PlayerCtl {
    fn call_method<T, A>(&self, method: &str, args: A) -> Result<T, String>
    where
        T: for<'z> dbus::arg::Get<'z> + dbus::arg::Arg,
        A: dbus::arg::AppendAll,
    {
        let proxy = self.get_proxy(None)?;

        let (value,): (T,) = proxy
            .method_call(&self.interface, method, args)
            .map_err(|e| format!("Failed to call method {}, cause: {}", method, e))?;

        Ok(value)
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

    // Methods
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
