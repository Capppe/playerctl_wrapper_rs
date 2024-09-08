extern crate playerctl_wrapper;

#[cfg(test)]
mod tests {
    use playerctl_wrapper::introspectable::Introspectable;

    #[test]
    fn test_method_introspect() {
        let i = Introspectable::new().unwrap();

        let res = i.introspect().unwrap();

        println!("Introspect: {}", res);

        assert!(res != "")
    }
}
