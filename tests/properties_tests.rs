extern crate playerctl_wrapper;

#[cfg(test)]
mod tests {
    use playerctl_wrapper::properties::Properties;

    // TODO: FIXME
    #[test]
    fn test_method_property_set() {
        let props = Properties::new().unwrap();

        props
            .set("org.mpris.MediaPlayer2.Player", "Volume", 0.2522)
            .unwrap();
    }
}
