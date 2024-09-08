extern crate playerctl_wrapper;

#[cfg(test)]
mod tests {
    use dbus::Path;
    use playerctl_wrapper::player::Player;

    #[test]
    fn test_method_next() {
        let player = Player::new().unwrap();

        let res = player.next().unwrap();

        assert!(res == ())
    }

    #[test]
    fn test_method_openuri() {
        let player = Player::new().unwrap();

        let res = player
            .open_uri("https://open.spotify.com/track/1mFrjW8e8fuAOowlU3Q3Dr")
            .unwrap();

        assert!(res == ())
    }

    #[test]
    fn test_method_pause() {
        let player = Player::new().unwrap();

        let res = player.pause().unwrap();

        assert!(res == ())
    }

    #[test]
    fn test_method_playpause() {
        let player = Player::new().unwrap();

        let res = player.play_pause().unwrap();

        assert!(res == ())
    }

    #[test]
    fn test_method_previous() {
        let player = Player::new().unwrap();

        let res = player.previous().unwrap();

        assert!(res == ())
    }

    #[test]
    fn test_method_seek() {
        let player = Player::new().unwrap();

        let res = player.seek(10000000).unwrap();

        assert!(res == ())
    }

    // Fails, expected
    #[test]
    fn test_method_setposition() {
        let player = Player::new().unwrap();

        let res = player.set_position(Path::from("test"), 10000000).unwrap();

        assert!(res == ())
    }

    #[test]
    fn test_method_stop() {
        let player = Player::new().unwrap();

        let res = player.stop().unwrap();

        assert!(res == ())
    }
}
