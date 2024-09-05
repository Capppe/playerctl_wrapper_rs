extern crate playerctl_wrapper_rs;

#[cfg(test)]
mod tests {
    use playerctl_wrapper_rs::introspectable::Introspectable;

    #[test]
    fn test_introspect() {
        let i = Introspectable::new().unwrap();

        let res = i.introspect();

        assert!(res.is_ok())
    }
}
