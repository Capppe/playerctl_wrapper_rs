extern crate playerctl_wrapper_rs;

#[cfg(test)]
mod tests {
    use playerctl_wrapper_rs::properties::Properties;

    #[test]
    fn test_method_get() {
        // let props = Properties::new().unwrap();

        // let metadata = props
        //     .get("org.mpris.MediaPlayer2.Player", "Metadata")
        //     .unwrap();

        // println!("Metadata: {:?}", metadata);
    }

    #[test]
    fn test_method_get_all() {
        // let props = Properties::new().unwrap();

        // let player_props = props.get_all("org.mpris.MediaPlayer2.Player").unwrap();
        //
        // println!("Metadata: {:?}", player_props);
        //
        // assert!(!player_props.is_empty())
    }

    // TODO: FIXME
    #[test]
    fn test_method_set() {
        let props = Properties::new().unwrap();

        props
            .set(
                "org.mpris.MediaPlayer2.Player",
                "Volume",
                dbus::arg::Variant(0.2522),
            )
            .unwrap();
    }
}
