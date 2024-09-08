extern crate playerctl_wrapper_rs;

#[cfg(test)]
mod tests {
    use dbus::arg::{PropMap, Variant};
    use playerctl_wrapper_rs::{metadata::Metadata, player::Player, playerctld::Properties};

    #[test]
    fn test_parse_metadata() {
        let player = Player::new().unwrap();

        let metadata: Variant<PropMap> = player.get_property("Metadata").unwrap();
        let parsed = Metadata::from(&metadata.0);

        println!("Parsed: {:?}", parsed);
    }
}
