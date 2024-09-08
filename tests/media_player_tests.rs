extern crate playerctl_wrapper;

#[cfg(test)]
mod tests {
    use playerctl_wrapper::media_player::MediaPlayer;

    #[test]
    fn test_method_quit() {
        let props = MediaPlayer::new().unwrap();

        let res = props.quit().unwrap();

        assert!(res == ())
    }

    #[test]
    fn test_method_raise() {
        let props = MediaPlayer::new().unwrap();

        let res = props.raise().unwrap();

        assert!(res == ())
    }
}
