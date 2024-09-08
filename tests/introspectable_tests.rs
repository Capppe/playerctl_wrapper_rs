extern crate playerctl_wrapper_rs;

#[cfg(test)]
mod tests {
    use playerctl_wrapper_rs::{introspectable::Introspectable, playerctld::Methods};

    #[test]
    fn test_method_introspect() {
        let i = Introspectable::new().unwrap();

        let res: String = i.call_method("Introspect", ()).unwrap();

        println!("Introspect: {}", res);

        assert!(res != "")
    }
}
