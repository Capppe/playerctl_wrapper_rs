use std::collections::HashMap;

use dbus::{
    arg::{PropMap, Variant},
    blocking::{Connection, Proxy},
};

pub fn create_proxy<'a>(
    dest: Option<&'a str>,
    path: &'a str,
    timeout: std::time::Duration,
    conn: &'a Connection,
) -> Result<Proxy<'a, &'a Connection>, String> {
    Ok(Proxy::new(
        dest.unwrap_or("org.mpris.MediaPlayer2.playerctld"),
        path,
        timeout,
        &conn,
    ))
}

pub fn parse_propmap<'a>(
    // msg: &mut HashMap<String, String>,
    props: &PropMap,
) -> HashMap<String, String> {
    props
        .into_iter()
        .map(|(key, value)| {
            let value_as_string = variant_to_string(value);
            (key.clone(), value_as_string.trim().to_owned())
        })
        .collect()
}

fn variant_to_string<T: std::fmt::Debug>(variant: &Variant<T>) -> String {
    format!("{:?}", variant)
}
