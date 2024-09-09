use std::{collections::HashMap, future::Future, time::Duration};

use dbus::{
    arg::{Append, Arg, RefArg, Variant},
    blocking::Connection,
    Message,
};
use tokio::sync::mpsc::Sender;

use crate::{
    dbus_utils::{self, parse_propmap},
    playerctld::{DBusItem, DBusProxy, Methods, Signals},
};

pub struct Properties {
    interface: String,
    object_path: String,
    connection: Connection,
}

impl DBusItem for Properties {
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

impl<'a> DBusProxy<'a> for Properties {
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

impl Signals for Properties {}

impl Methods for Properties {}

impl Properties {
    pub fn new() -> Result<Self, dbus::Error> {
        Ok(Self {
            interface: "org.freedesktop.DBus.Properties".to_string(),
            object_path: "/org/mpris/MediaPlayer2".to_string(),
            connection: Connection::new_session()?,
        })
    }

    // Methods
    pub fn get<T>(&self, interface_name: &str, property_name: &str) -> Result<T, String>
    where
        T: Arg + for<'z> dbus::arg::Get<'z>,
    {
        self.call_method("Get", (interface_name, property_name))
    }

    pub fn get_all<T>(&self, interface_name: &str) -> Result<Vec<T>, String>
    where
        T: Arg + for<'z> dbus::arg::Get<'z>,
    {
        self.call_method("GetAll", (interface_name,))
    }

    pub fn set<T>(&self, interface_name: &str, property_name: &str, value: T) -> Result<(), String>
    where
        T: Append + dbus::arg::Arg,
    {
        self.call_method_no_return("Set", (interface_name, property_name, value))
    }

    // Signals
    pub fn properties_changed<'a>(
        &'a self,
        sender: Sender<HashMap<String, String>>,
        interface: Option<&'a str>,
    ) -> impl Future + 'a {
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

        self.start_listener(
            tx,
            interface.unwrap_or(self.get_interface()),
            "PropertiesChanged",
        )
    }
}
