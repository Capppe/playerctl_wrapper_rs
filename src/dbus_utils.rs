use dbus::blocking::{Connection, Proxy};

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
