extern crate playerctl_wrapper;

#[cfg(test)]
mod tests {
    use playerctl_wrapper::playerctld::Signals;
    use playerctl_wrapper::playlists::Playlists;

    #[test]
    fn test_method_activateplaylist() {
        let props = Playlists::new().unwrap();

        let res = props.activate_playlist("PlaylistId".into()).unwrap();

        assert!(res == ())
    }

    // FIXME: fails, need to know correct args
    #[test]
    fn test_method_getplaylists() {
        let props = Playlists::new().unwrap();

        let playlists = props.get_playlists(1 as u32, 1 as u32, "", false).unwrap();

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
