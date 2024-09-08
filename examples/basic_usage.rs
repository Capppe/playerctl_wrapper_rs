use dbus::arg::PropMap;
use playerctl_wrapper::{metadata::Metadata, player::Player, playerctld::Properties};

// Proper error handling omitted
fn main() {
    let player = Player::new().unwrap();

    player.play().unwrap();

    let metadata: PropMap = player.get_property("Metadata").unwrap();

    let parsed = Metadata::from(&metadata);

    println!(
        "Now playing: {:?} by {:?}",
        parsed.title.unwrap(),
        parsed.artist.unwrap()
    );

    player.pause().unwrap();

    let paused: String = player.get_property("PlaybackStatus").unwrap();

    if paused == "Paused" {
        println!("Paused!")
    } else {
        println!("Playing!")
    }
}
