extern crate playerctl_wrapper;

#[cfg(test)]
mod tests {
    use playerctl_wrapper::playerctl::PlayerCtl;
    use playerctl_wrapper::playerctld::Signals;

    #[test]
    fn test_method_shift() {
        let playerctl = PlayerCtl::new().unwrap();

        let player = playerctl.shift().unwrap();

        assert!(player != "");
    }

    #[test]
    fn test_method_unshift() {
        let playerctl = PlayerCtl::new().unwrap();

        let player = playerctl.unshift().unwrap();

        assert!(player != "");
    }

    #[tokio::test]
    async fn test_signal_active_player_change_begin() {
        let playerctl = PlayerCtl::new().unwrap();
        let (tx, mut rx) = tokio::sync::mpsc::channel(100);

        tokio::spawn(async move {
            while let Some(message) = rx.recv().await {
                println!("Message: {}", message);
            }
        });
        let _res = playerctl
            .start_listener(tx, "ActivePlayerChangeBegin")
            .await;

        assert!(1 == 2)
    }

    #[tokio::test]
    async fn test_signal_active_player_change_end() {
        let playerctl = PlayerCtl::new().unwrap();
        let (tx, mut rx) = tokio::sync::mpsc::channel(100);

        tokio::spawn(async move {
            while let Some(message) = rx.recv().await {
                println!("Message: {}", message);
            }
        });
        let _res = playerctl.start_listener(tx, "ActivePlayerChangeEnd").await;

        assert!(1 == 2)
    }
}
