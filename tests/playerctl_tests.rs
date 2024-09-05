extern crate playerctl_wrapper_rs;

#[cfg(test)]
mod tests {
    use playerctl_wrapper_rs::playerctl::PlayerCtl;
    use playerctl_wrapper_rs::playerctld::{Methods, Signals};

    #[test]
    fn test_method_shift() {
        let playerctl = PlayerCtl::new().unwrap();

        let player: String = playerctl.call_method("Shift", ()).unwrap();

        println!("Player: {}", player);

        assert!(player != "");
    }

    #[test]
    fn test_method_unshift() {
        let playerctl = PlayerCtl::new().unwrap();

        let player: String = playerctl.call_method("Unshift", ()).unwrap();

        println!("Player: {}", player);

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
