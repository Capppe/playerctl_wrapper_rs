extern crate playerctl_wrapper;

#[cfg(test)]
mod tests {
    use dbus::arg::PropMap;
    use playerctl_wrapper::{metadata::Metadata, player::Player, playerctld::Properties};

    #[test]
    fn test_parse_metadata() {
        let player = Player::new().unwrap();

        let metadata: PropMap = player.get_property("Metadata").unwrap();
        let parsed = Metadata::from(&metadata);

        println!("Parsed: {:?}", parsed);
    }
}
