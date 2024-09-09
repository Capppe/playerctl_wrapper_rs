extern crate playerctl_wrapper;

#[cfg(test)]
mod tests {
    use dbus::arg::PropMap;
    use playerctl_wrapper::{playerctld::DBusItem, tracklist::Tracklist};

    #[test]
    fn test_method_addtrack() {
        let tracklist = Tracklist::new().unwrap();

        let metadata = tracklist
            .add_track("uri", "aftertrack".into(), false)
            .unwrap();

        assert!(metadata == ())
    }

    #[test]
    fn test_method_gettracksmetadata() {
        let props = Tracklist::new().unwrap();

        let metadata: Vec<PropMap> = props.get_tracks_metadata(vec!["trackids".into()]).unwrap();

        assert!(!metadata.is_empty())
    }

    #[test]
    fn test_method_goto() {
        let props = Tracklist::new().unwrap();

        props.go_to("Trackid".into()).unwrap();
    }

    #[test]
    fn test_method_removetrack() {
        let props = Tracklist::new().unwrap();

        props.remove_track("Trackid".into()).unwrap();
    }

    #[tokio::test]
    async fn test_signal_trackadded() {
        let tl = Tracklist::new().unwrap();
        let (tx, mut rx) = tokio::sync::mpsc::channel(100);

        tokio::spawn(async move {
            while let Some(message) = rx.recv().await {
                println!("Message: {:?}", message);
            }
        });

        let _res = tl.track_added(tx, Some(tl.get_interface())).await;

        assert!(1 == 2)
    }

    #[tokio::test]
    async fn test_signal_tracklistreplaced() {
        let tl = Tracklist::new().unwrap();
        let (tx, mut rx) = tokio::sync::mpsc::channel(100);

        tokio::spawn(async move {
            while let Some(message) = rx.recv().await {
                println!("Message: {:?}", message);
            }
        });

        let _res = tl.track_list_replaced(tx, Some(tl.get_interface())).await;

        assert!(1 == 2)
    }

    #[tokio::test]
    async fn test_signal_trackmetadatachanged() {
        let tl = Tracklist::new().unwrap();
        let (tx, mut rx) = tokio::sync::mpsc::channel(100);

        tokio::spawn(async move {
            while let Some(message) = rx.recv().await {
                println!("Message: {:?}", message);
            }
        });

        let _res = tl
            .track_metadata_changed(tx, Some(tl.get_interface()))
            .await;

        assert!(1 == 2)
    }

    #[tokio::test]
    async fn test_signal_trackremoved() {
        let tl = Tracklist::new().unwrap();
        let (tx, mut rx) = tokio::sync::mpsc::channel(100);

        tokio::spawn(async move {
            while let Some(message) = rx.recv().await {
                println!("Message: {:?}", message);
            }
        });

        let _res = tl.track_removed(tx, Some(tl.get_interface())).await;

        assert!(1 == 2)
    }
}
