extern crate playerctl_wrapper_rs;

#[cfg(test)]
mod tests {
    use dbus::arg::{PropMap, RefArg, Variant};
    use playerctl_wrapper_rs::playerctld::{Methods, Signals};
    use playerctl_wrapper_rs::properties::Properties;
    use playerctl_wrapper_rs::tracklist::Tracklist;

    #[test]
    fn test_method_addtrack() {
        let tracklist = Tracklist::new().unwrap();

        let metadata = tracklist
            .call_method_no_return("AddTrack", ("uri", "aftertrack", false))
            .unwrap();

        assert!(metadata == ())
    }

    #[test]
    fn test_method_gettracksmetadata() {
        let props = Tracklist::new().unwrap();

        let metadata: Vec<PropMap> = props
            .call_method("GetTracksMetadata", (vec!["trackids"],))
            .unwrap();

        println!("Metadata: {:?}", metadata);

        assert!(!metadata.is_empty())
    }

    #[test]
    fn test_method_goto() {
        let props = Tracklist::new().unwrap();

        props.call_method_no_return("GoTo", ("Trackid",)).unwrap();
    }

    #[test]
    fn test_method_removetrack() {
        let props = Tracklist::new().unwrap();

        props
            .call_method_no_return("RemoveTrack", ("Trackid",))
            .unwrap();
    }

    #[tokio::test]
    async fn test_signal_trackadded() {
        let tl = Tracklist::new().unwrap();
        let (tx, mut rx) = tokio::sync::mpsc::channel(100);

        tokio::spawn(async move {
            while let Some(message) = rx.recv().await {
                println!("Message: {}", message);
            }
        });

        let _res = tl.start_listener(tx, "TrackAdded").await;

        assert!(1 == 2)
    }

    #[tokio::test]
    async fn test_signal_tracklistreplaced() {
        let tl = Tracklist::new().unwrap();
        let (tx, mut rx) = tokio::sync::mpsc::channel(100);

        tokio::spawn(async move {
            while let Some(message) = rx.recv().await {
                println!("Message: {}", message);
            }
        });

        let _res = tl.start_listener(tx, "TrackListReplaced").await;

        assert!(1 == 2)
    }

    #[tokio::test]
    async fn test_signal_trackmetadatachanged() {
        let tl = Tracklist::new().unwrap();
        let (tx, mut rx) = tokio::sync::mpsc::channel(100);

        tokio::spawn(async move {
            while let Some(message) = rx.recv().await {
                println!("Message: {}", message);
            }
        });

        let _res = tl.start_listener(tx, "TrackMetadataChanged").await;

        assert!(1 == 2)
    }

    #[tokio::test]
    async fn test_signal_trackremoved() {
        let tl = Tracklist::new().unwrap();
        let (tx, mut rx) = tokio::sync::mpsc::channel(100);

        tokio::spawn(async move {
            while let Some(message) = rx.recv().await {
                println!("Message: {}", message);
            }
        });

        let _res = tl.start_listener(tx, "TrackRemoved").await;

        assert!(1 == 2)
    }
}
