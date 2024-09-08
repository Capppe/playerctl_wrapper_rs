extern crate playerctl_wrapper_rs;

#[cfg(test)]
mod tests {
    use playerctl_wrapper_rs::media_player::MediaPlayer;
    use playerctl_wrapper_rs::playerctld::Methods;

    #[test]
    fn test_method_quit() {
        let props = MediaPlayer::new().unwrap();

        let res = props.call_method_no_return("Quit", ()).unwrap();

        assert!(res == ())
    }

    #[test]
    fn test_method_raise() {
        let props = MediaPlayer::new().unwrap();

        let res = props.call_method_no_return("Raise", ()).unwrap();

        assert!(res == ())
    }
}
