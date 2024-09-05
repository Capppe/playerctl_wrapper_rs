extern crate playerctl_wrapper_rs;

#[cfg(test)]
mod tests {
    use playerctl_wrapper_rs::player::Player;

    #[test]
    fn test_next() {
        let player = Player::new().unwrap();

        let res = player.next().unwrap();

        assert!(res == ())
    }
}
