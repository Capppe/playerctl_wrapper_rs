extern crate playerctl_wrapper;

#[cfg(test)]
mod tests {
    use playerctl_wrapper::peer::Peer;
    use playerctl_wrapper::playerctld::Methods;

    #[test]
    fn test_method_get_machine_id() {
        let peer = Peer::new().unwrap();

        let machine_id = peer.get_machine_id().unwrap();

        println!("Player: {}", machine_id);

        assert!(machine_id != "");
    }

    #[test]
    fn test_method_ping() {
        let peer = Peer::new().unwrap();

        let res = peer.call_method_no_return("Ping", ()).unwrap();

        assert!(res == ());
    }
}
