use dbus::arg::{PropMap, Variant};
use playerctl_wrapper_rs::{metadata::Metadata, player::Player, playerctld::Properties};

// Proper error handling omitted
fn main() {
    let player = Player::new().unwrap();

    player.play().unwrap();

    let metadata: Variant<PropMap> = player.get_property("Metadata").unwrap();

    let parsed = Metadata::from(&metadata.0).unwrap();

    println!(
        "Now playing: {:?} by {:?}",
        parsed.title.unwrap(),
        parsed.artist.unwrap()
    );

    player.pause().unwrap();

    let paused = player
        .get_property::<Variant<String>>("PlaybackStatus")
        .unwrap();

    if paused.0 == "Paused" {
        println!("Paused!")
    } else {
        println!("Playing!")
    }
}
