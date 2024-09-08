extern crate playerctl_wrapper_rs;

#[cfg(test)]
mod tests {
    use dbus::arg::{PropMap, RefArg, Variant};
    use playerctl_wrapper_rs::playerctld::Methods;
    use playerctl_wrapper_rs::properties::Properties;

    #[test]
    fn test_method_get() {
        let props = Properties::new().unwrap();

        let metadata: Variant<Box<dyn RefArg + 'static>> = props
            .call_method("Get", ("org.mpris.MediaPlayer2.Player", "Metadata"))
            .unwrap();

        println!("Metadata: {:?}", metadata.0);
    }

    #[test]
    fn test_method_get_all() {
        let props = Properties::new().unwrap();

        let player_props: PropMap = props
            .call_method("GetAll", ("org.mpris.MediaPlayer2.Player",))
            .unwrap();

        println!("Metadata: {:?}", player_props);

        assert!(!player_props.is_empty())
    }

    // TODO: FIXME
    #[test]
    fn test_method_set() {
        let props = Properties::new().unwrap();

        props
            .call_method_no_return("Set", ("org.mpris.MediaPlayer2.Player", "Volume", 0.2522))
            .unwrap();
    }
}
