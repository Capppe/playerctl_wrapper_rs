use std::time::Duration;

use dbus::arg::Variant;
use dbus::blocking::Connection;
use dbus::message::MatchRule;
use dbus::Message;
use tokio::sync::mpsc::Sender;

use crate::dbus_utils;
use crate::introspectable::Introspectable;
use crate::media_player::MediaPlayer;
use crate::peer::Peer;
use crate::player::Player;
use crate::playerctl::PlayerCtl;
use crate::playlists::Playlists;
use crate::properties;
use crate::tracklist::Tracklist;

pub trait DBusItem {
    fn get_interface(&self) -> &str;
    fn get_object_path(&self) -> &str;
    fn get_connection(&self) -> &Connection;
}

pub trait DBusProxy<'a> {
    fn get_proxy(
        &'a self,
        dest: Option<&'a str>,
        object_path: Option<&'a str>,
    ) -> Result<dbus::blocking::Proxy<&Connection>, String>
    where
        Self: DBusItem,
    {
        let proxy = dbus_utils::create_proxy(
            dest,
            object_path.unwrap_or(&self.get_object_path()),
            Duration::from_secs(5),
            &self.get_connection(),
        )?;

        Ok(proxy)
    }
}

pub trait Signals {
    fn start_listener(
        &self,
        sender: Sender<Message>,
        interface: &str,
        signal: &str,
    ) -> impl std::future::Future<Output = Result<(), String>> + Send {
        async move {
            let (resource, connection) = dbus_tokio::connection::new_session_sync()
                .map_err(|e| format!("Failed to get DBus connection(async): {}", e))?;

            tokio::spawn(async {
                let err = resource.await;
                panic!("Lost connection to DBus: {}", err);
            });

            let rule = MatchRule::new_signal(interface.to_owned(), signal.to_owned());

            use futures_util::stream::StreamExt;
            let (incoming_signal, stream) = connection
                .add_match(rule)
                .await
                .map_err(|e| format!("Failed to add signal match rule: {}", e))?
                .stream();

            let stream = stream.for_each(|(msg, (_source,)): (Message, (String,))| {
                let sender = sender.clone();
                tokio::spawn(async move {
                    let _ = sender.send(msg).await;
                });
                async {}
            });

            futures_util::join!(stream);

            connection
                .remove_match(incoming_signal.token())
                .await
                .map_err(|e| format!("Failed to remove signal match rule: {}", e))?;

            Ok(())
        }
    }
}

pub trait Methods {
    fn call_method<T, A>(&self, method: &str, args: A) -> Result<T, String>
    where
        T: for<'z> dbus::arg::Get<'z> + dbus::arg::Arg,
        A: dbus::arg::AppendAll,
        Self: for<'a> DBusProxy<'a> + DBusItem,
    {
        let proxy = self.get_proxy(None, None)?;

        let (value,): (T,) = proxy
            .method_call(self.get_interface(), method, args)
            .map_err(|e| format!("Failed to call method {}, cause: {}", method, e))?;

        Ok(value)
    }

    fn call_method_no_return<A>(&self, method: &str, args: A) -> Result<(), String>
    where
        A: dbus::arg::AppendAll,
        Self: for<'a> DBusProxy<'a> + DBusItem,
    {
        let proxy = self.get_proxy(None, None)?;

        proxy
            .method_call(self.get_interface(), method, args)
            .map_err(|e| format!("Failed to call method {}, cause: {}", method, e))?;

        Ok(())
    }
}

pub trait Properties {
    fn get_property<T>(&self, property: &str) -> Result<T, String>
    where
        Self: DBusItem,
        T: dbus::arg::Arg + for<'z> dbus::arg::Get<'z>,
    {
        if let Ok(props) = properties::Properties::new() {
            let prop: Variant<T> = props.get(&self.get_interface(), property)?;
            Ok(prop.0)
        } else {
            Err(format!("Failed to get properties"))
        }
    }

    fn get_all_properties<T>(&self) -> Result<Vec<T>, String>
    where
        Self: DBusItem,
        T: dbus::arg::Arg + for<'z> dbus::arg::Get<'z>,
    {
        if let Ok(props) = properties::Properties::new() {
            let prop: Vec<T> = props.get_all(&self.get_interface())?;
            Ok(prop)
        } else {
            Err(format!("Failed to get all properties"))
        }
    }

    fn set_property<T>(&self, property: &str, value: T) -> Result<(), String>
    where
        Self: DBusItem,
        T: dbus::arg::Arg + for<'z> dbus::arg::Get<'z> + dbus::arg::Append,
    {
        if let Ok(props) = properties::Properties::new() {
            props.set(&self.get_interface(), property, value)
        } else {
            Err(format!("Failed to set properties"))
        }
    }
}

pub struct PlayerCtld {
    pub playerctl: PlayerCtl,
    pub introspectable: Introspectable,
    pub peer: Peer,
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
            media_player: MediaPlayer::new()?,
            player: Player::new()?,
            playlists: Playlists::new()?,
            tracklist: Tracklist::new()?,
        })
    }
}
