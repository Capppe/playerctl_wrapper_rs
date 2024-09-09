extern crate playerctl_wrapper;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use playerctl_wrapper::{playerctld::DBusItem, properties::Properties};

    // TODO: FIXME
    #[test]
    fn test_method_property_set() {
        let props = Properties::new().unwrap();

        props
            .set("org.mpris.MediaPlayer2.Player", "Volume", 0.2522)
            .unwrap();
    }

    #[tokio::test]
    async fn test_signal_properties_changed() {
        let props = Properties::new().unwrap();
        let (tx, mut rx) = tokio::sync::mpsc::channel::<HashMap<String, String>>(100);

        tokio::spawn(async move {
            while let Some(message) = rx.recv().await {
                println!("Message: {:?}", message);
            }
        });

        let _ = props
            .properties_changed(tx, Some(props.get_interface()))
            .await;
    }
}
