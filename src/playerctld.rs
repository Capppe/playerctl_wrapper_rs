use dbus::blocking::{Connection, Proxy};
use dbus::message::MatchRule;
use tokio::sync::mpsc::Sender;

use crate::introspectable::Introspectable;
use crate::media_player::MediaPlayer;
use crate::peer::Peer;
use crate::player::Player;
use crate::playerctl::PlayerCtl;
use crate::playlists::Playlists;
use crate::properties::Properties;
use crate::tracklist::Tracklist;

pub trait DBusProxy<'a> {
    fn get_proxy(
        &'a self,
        dest: Option<&'a str>,
        object_path: Option<&'a str>,
    ) -> Result<dbus::blocking::Proxy<&Connection>, String>;
}

pub trait Signals {
    fn start_listener(
        &self,
        sender: Sender<String>,
        signal: &str,
    ) -> impl std::future::Future<Output = Result<(), String>> + Send {
        async move {
            let (resource, connection) = dbus_tokio::connection::new_session_sync()
                .map_err(|e| format!("Failed to get DBus connection(async): {}", e))?;

            tokio::spawn(async {
                let err = resource.await;
                panic!("Lost connection to DBus: {}", err);
            });

            let rule = MatchRule::new_signal("com.github.altdesktop.playerctld", signal.to_owned());

            use futures_util::stream::StreamExt;
            let (incoming_signal, stream) = connection
                .add_match(rule)
                .await
                .map_err(|e| format!("Failed to add signal match rule: {}", e))?
                .stream();
            let stream = stream.for_each(|(_, (source,)): (_, (String,))| {
                let sender = sender.clone();
                tokio::spawn(async move {
                    let _ = sender.send(source).await;
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
        Self: for<'a> DBusProxy<'a>,
    {
        let proxy = self.get_proxy(None, None)?;

        let (value,): (T,) = proxy
            // .method_call(interface, method, args)
            .method_call(self.interface(), method, args)
            .map_err(|e| format!("Failed to call method {}, cause: {}", method, e))?;

        Ok(value)
    }

    fn call_method_no_return<A>(&self, method: &str, args: A) -> Result<(), String>
    where
        A: dbus::arg::AppendAll,
        Self: for<'a> DBusProxy<'a>,
    {
        let proxy = self.get_proxy(None, None)?;

        proxy
            .method_call(self.interface(), method, args)
            .map_err(|e| format!("Failed to call method {}, cause: {}", method, e))?;

        Ok(())
    }

    fn interface(&self) -> &str;
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
