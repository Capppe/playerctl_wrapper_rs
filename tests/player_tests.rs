extern crate playerctl_wrapper_rs;

#[cfg(test)]
mod tests {
    use dbus::Path;
    use playerctl_wrapper_rs::{player::Player, playerctld::Methods};

    #[test]
    fn test_method_next() {
        let player = Player::new().unwrap();

        let res = player.call_method_no_return("Next", ()).unwrap();

        assert!(res == ())
    }

    #[test]
    fn test_method_openuri() {
        let player = Player::new().unwrap();

        let res = player
            .call_method_no_return(
                "OpenUri",
                ("https://open.spotify.com/track/1mFrjW8e8fuAOowlU3Q3Dr",),
            )
            .unwrap();

        assert!(res == ())
    }

    #[test]
    fn test_method_pause() {
        let player = Player::new().unwrap();

        let res = player.call_method_no_return("Pause", ()).unwrap();

        assert!(res == ())
    }

    #[test]
    fn test_method_playpause() {
        let player = Player::new().unwrap();

        let res = player.call_method_no_return("PlayPause", ()).unwrap();

        assert!(res == ())
    }

    #[test]
    fn test_method_previous() {
        let player = Player::new().unwrap();

        let res = player.call_method_no_return("Previous", ()).unwrap();

        assert!(res == ())
    }

    #[test]
    fn test_method_seek() {
        let player = Player::new().unwrap();

        let res = player.call_method_no_return("Seek", (10000000,)).unwrap();

        assert!(res == ())
    }

    #[test]
    fn test_method_setposition() {
        let player = Player::new().unwrap();

        let res = player
            .call_method_no_return("SetPosition", (Path::from("test"), 10000000))
            .unwrap();

        assert!(res == ())
    }

    #[test]
    fn test_method_stop() {
        let player = Player::new().unwrap();

        let res = player.call_method_no_return("Stop", ()).unwrap();

        assert!(res == ())
    }
}
