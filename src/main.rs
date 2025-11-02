use rdev::{Event, EventType, Key, listen};

mod imports;

mod utils;

mod macros;
use macros::*;

fn main() {
    println!("listening for F7 input");

    if let Err(error) = listen(callback) {
        eprintln!("failed to listen for some reason, {:?}", error)
    }
}

fn callback(event: Event) {
    if let EventType::KeyPress(key) = event.event_type {
        if key == Key::F7 {
            create_skill("ShutUp", true, true);
        }
    }
}