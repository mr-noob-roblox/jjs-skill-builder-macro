use rdev::{Event, EventType, Key, listen};

mod imports;

mod utils;

mod macros;
use macros::*;

mod gui;
use gui as app;

use std::thread;

fn main() {
    // macro thread
    thread::spawn(|| {
        println!("listening for F7 input");

        if let Err(error) = listen(callback) {
            eprintln!("failed to listen for some reason, {:?}", error)
        }
    });

    app::start().unwrap();

    println!("bye!");
}

fn callback(event: Event) {
    if let EventType::KeyPress(key) = event.event_type {
        if key == Key::F7 {
            create_skill("ShutUp", true, true);
        }
    }
}
