use std::time::Duration;

use dbus::blocking::Connection;
use tokio::signal::unix::Signal;

use crate::{dbus_utils, playerctl::Property, playerctld::DBusProxy};

pub struct Peer {
    properties: Vec<Property>,
    signals: Vec<Signal>,
    interface: String,
    object_path: String,
    connection: Connection,
}

impl<'a> DBusProxy<'a> for Peer {
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

impl Peer {
    pub fn new() -> Result<Self, dbus::Error> {
        Ok(Self {
            properties: Vec::new(),
            signals: Vec::new(),
            interface: "org.freedesktop.DBus.Peer".to_string(),
            object_path: "/org/mpris/MediaPlayer2".to_string(),
            connection: Connection::new_session()?,
        })
    }

    pub fn get_machine_id(&self) -> Result<String, String> {
        let proxy = self.get_proxy(None)?;

        let (machine_uuid,): (String,) = proxy
            .method_call(&self.interface, "GetMachineId", ())
            .map_err(|e| format!("Failed to get machine id: {}", e))?;

        Ok(machine_uuid)
    }

    pub fn ping(&self) -> Result<(), String> {
        let proxy = self.get_proxy(None)?;

        proxy
            .method_call(&self.interface, "Ping", ())
            .map_err(|e| format!("Failed to get machine id: {}", e))?;

        Ok(())
    }
}
