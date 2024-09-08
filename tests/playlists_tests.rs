extern crate playerctl_wrapper_rs;

#[cfg(test)]
mod tests {
    use dbus::arg::PropMap;
    use dbus::Path;
    use playerctl_wrapper_rs::playerctld::{Methods, Signals};
    use playerctl_wrapper_rs::playlists::Playlists;
    use playerctl_wrapper_rs::properties::Properties;

    #[test]
    fn test_method_activateplaylist() {
        let props = Playlists::new().unwrap();

        let res = props
            .call_method_no_return("ActivatePlaylist", ("PlaylistId",))
            .unwrap();

        assert!(res == ())
    }

    // FIXME: fails, need to know correct args
    #[test]
    fn test_method_getplaylists() {
        let props = Playlists::new().unwrap();

        let playlists: Vec<(Path, String, String)> = props
            .call_method("GetPlaylists", (1 as u32, 1 as u32, "", false))
            .unwrap();

        println!("Playlists: {:?}", playlists);

        assert!(playlists.is_empty())
    }

    #[tokio::test]
    async fn test_signal_playlistchanged() {
        let playlists = Playlists::new().unwrap();
        let (tx, mut rx) = tokio::sync::mpsc::channel(100);

        tokio::spawn(async move {
            while let Some(message) = rx.recv().await {
                println!("Message: {}", message);
            }
        });

        let _res = playlists.start_listener(tx, "PlaylistChanged").await;
    }
}
