use std::time::Duration;

use dbus::blocking::Connection;

use crate::{
    dbus_utils,
    playerctl::Property,
    playerctld::{DBusProxy, Methods},
};

pub struct MediaPlayer {
    properties: Vec<Property>,
    interface: String,
    object_path: String,
    connection: Connection,
}

impl<'a> DBusProxy<'a> for MediaPlayer {
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

impl Methods for MediaPlayer {
    fn interface(&self) -> &str {
        &self.interface
    }
}

impl MediaPlayer {
    pub fn new() -> Result<Self, dbus::Error> {
        Ok(Self {
            properties: Vec::new(),
            interface: "org.mpris.MediaPlayer2".to_string(),
            object_path: "/org/mpris/MediaPlayer2".to_string(),
            connection: Connection::new_session()?,
        })
    }

    // pub fn quit(&self) -> Result<(), String> {
    //     let proxy = self
    //         .get_proxy(None, None)
    //         .map_err(|e| format!("Failed to create a proxy: {}", e))?;
    //
    //     proxy
    //         .method_call(&self.interface, "Quit", ())
    //         .map_err(|e| format!("Failed to quit: {}", e))?;
    //
    //     Ok(())
    // }
    //
    // pub fn raise(&self) -> Result<(), String> {
    //     let proxy = self
    //         .get_proxy(None, None)
    //         .map_err(|e| format!("Failed to create a proxy: {}", e))?;
    //
    //     proxy
    //         .method_call(&self.interface, "Raise", ())
    //         .map_err(|e| format!("Failed to raise: {}", e))?;
    //
    //     Ok(())
    // }
}
