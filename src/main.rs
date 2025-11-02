use rdev::{Event, EventType, Key, listen};
use rustautogui::{self, RustAutoGui};

use std::{
    collections::HashMap, mem, sync::Mutex, thread::sleep, time::Duration
};
use winapi::um::winuser::{INPUT, INPUT_MOUSE, MOUSEEVENTF_MOVE, SendInput};

use once_cell::{self, sync::Lazy};

static SKILL_CACHE: Lazy<Mutex<HashMap<String, u32>>> = Lazy::new(|| {
    Mutex::new(HashMap::new())
});

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
            create_wait(0.5, false);
            create_skill("ShutUp", true, false);
            create_wait(0.5, false);
            create_skill("ShutUp", true, false);
            create_wait(0.5, false);
            create_skill("ShutUp", true, false);
            create_wait(0.5, false);
            create_skill("ShutUp", true, false);
            create_wait(0.5, false);
            create_skill("ShutUp", true, false);
            create_wait(0.5, false);
            create_skill("ShutUp", true, false);
            create_wait(0.5, false);
            create_skill("ShutUp", true, false);
            create_wait(0.5, false);
            create_skill("Kamutoke", true, false);
            create_skill("Kamutoke", true, false);
            create_skill("Kamutoke", true, false);
            create_skill("Kamutoke", true, false);
            create_skill("Kamutoke", true, false);
            create_skill("Kamutoke", true, false);
            create_skill("Kamutoke", true, false);
        }
    }
}

fn click_mouse(gui: &RustAutoGui) {
    gui.left_click().unwrap();

    sleep(Duration::from_millis(50));
}

fn move_mouse_to_pos(gui: &RustAutoGui, x: u32, y: u32, m_time: f32) {
    gui.move_mouse_to_pos(x, y, m_time).unwrap();

    nudge_mouse();

    sleep(Duration::from_millis(50));
}

fn move_mouse(gui: &RustAutoGui, x: i32, y: i32, m_time: f32) {
    gui.move_mouse(x, y, m_time).unwrap();

    nudge_mouse();

    sleep(Duration::from_millis(50));
}

fn find_stored_image_on_screen_and_move_mouse(gui: &mut RustAutoGui, precision: f32, m_time: f32, alias: &str) -> Option<Vec<(u32, u32, f32)>> {
    let location: Option<Vec<(u32, u32, f32)>> = gui.find_stored_image_on_screen_and_move_mouse(precision, m_time, alias).unwrap();

    nudge_mouse();

    sleep(Duration::from_millis(50));

    return location;
}

fn nudge_mouse() {
    unsafe {
        // Move up
        let mut input: INPUT = mem::zeroed();
        input.type_ = INPUT_MOUSE;
        (*input.u.mi_mut()).dx = 0;
        (*input.u.mi_mut()).dy = -1;
        (*input.u.mi_mut()).dwFlags = MOUSEEVENTF_MOVE;
        SendInput(1, &mut input, mem::size_of::<INPUT>() as i32);

        sleep(Duration::from_millis(5));

        // Move down
        let mut input: INPUT = mem::zeroed();
        input.type_ = INPUT_MOUSE;
        (*input.u.mi_mut()).dx = 0;
        (*input.u.mi_mut()).dy = 1;
        (*input.u.mi_mut()).dwFlags = MOUSEEVENTF_MOVE;
        SendInput(1, &mut input, mem::size_of::<INPUT>() as i32);
    }
}

fn create_wait(time: f32, is_first: bool) {
    println!("create wait");

    let mut gui = rustautogui::RustAutoGui::new(false).unwrap();

    gui.store_template_from_file_custom(
        "./assets/blocks/WaitBlock.png",
        Some((667, 381, 595, 351)),
        rustautogui::MatchMode::Segmented,
        "waitBlock",
        0.0,
    )
    .unwrap();

    gui.store_template_from_file_custom(
        "./assets/properties/WaitBlockTime.png",
        Some((664, 419, 294, 308)),
        rustautogui::MatchMode::Segmented,
        "waitBlockTime",
        0.0,
    )
    .unwrap();

    find_stored_image_on_screen_and_move_mouse(&mut gui, 0.95, 0.0, "waitBlock");

    click_mouse(&gui);

    find_stored_image_on_screen_and_move_mouse(&mut gui, 0.8, 0.0, "waitBlockTime");

    move_mouse(&gui, 108, 0, 0.0);

    click_mouse(&gui);

    gui.keyboard_command("backspace").unwrap();
    gui.keyboard_input(&time.to_string()).unwrap();

    move_mouse_to_pos(&gui, 1075, 438, 0.0);

    if !is_first {
        click_mouse(&gui);

        sleep(Duration::from_millis(50));

        click_mouse(&gui);
    } else {
        click_mouse(&gui);
    }

    sleep(Duration::from_millis(100));
}

fn create_skill(skill_name: &str, cancel_last: bool, is_first: bool) {
    println!("create skill");

    let mut gui = rustautogui::RustAutoGui::new(false).unwrap();

    // skill block and skill
    gui.store_template_from_file_custom(
        "./assets/blocks/SkillBlock.png",
        Some((664, 419, 294, 308)),
        rustautogui::MatchMode::Segmented,
        "skillBlock",
        0.0,
    )
    .unwrap();

    gui.store_template_from_file_custom(
        &format!("./assets/skills/{}.png", skill_name),
        Some((664, 419, 294, 308)),
        rustautogui::MatchMode::Segmented,
        "skill",
        0.0,
    )
    .unwrap();

    // properties
    gui.store_template_from_file_custom(
        "./assets/properties/SkillCancelLast.png",
        Some((664, 419, 294, 308)),
        rustautogui::MatchMode::Segmented,
        "cancelLast",
        0.0,
    )
    .unwrap();

    if find_stored_image_on_screen_and_move_mouse(&mut gui, 0.5, 0.0, "skillBlock").is_none() {
        let _ = msgbox::create("error", "failed to create skill block (make sure skill is cleared)", msgbox::IconType::Error);
        std::process::exit(1);
    }

    click_mouse(&gui);

    move_mouse_to_pos(&gui, 935, 430, 0.0);

    gui.click_down(rustautogui::MouseClick::LEFT).unwrap();

    let mut pos: Option<Vec<(u32, u32, f32)>> = None;

    // skill scan and caching
    let mut skill_cache = SKILL_CACHE.lock().unwrap();
    let cached_skill = skill_cache.get(skill_name);

    match cached_skill {
        Some(_) => {
            move_mouse_to_pos(&gui, 935, *cached_skill.unwrap(), 0.0);

            pos = gui.find_stored_image_on_screen(0.99, "skill").unwrap();
        },
        None => {
            for _ in 0..60 {
                pos = gui.find_stored_image_on_screen(0.99, "skill").unwrap();

                match pos {
                    Some(_) => break,
                    None => (),
                };

                move_mouse(&gui, 0, 3, 0.0);
            }

            let (_, y) = gui.get_mouse_position().unwrap();
            skill_cache.insert(skill_name.to_string(), y as u32);
        }
    }

    gui.click_up(rustautogui::MouseClick::LEFT).unwrap();

    if pos.is_none() {
        let _ = msgbox::create("error", "failed to select skill for skill block", msgbox::IconType::Error);
        std::process::exit(1);
    }

    let found_pos = pos.unwrap();
    let (x, y, _) = found_pos[0];
    move_mouse_to_pos(&gui, x, y, 0.0);

    gui.left_click().unwrap();

    // properties
    gui.scroll_down(100).unwrap();
    sleep(Duration::from_millis(50));
    gui.scroll_down(100).unwrap();

    // cancel last
    if cancel_last {
        find_stored_image_on_screen_and_move_mouse(&mut gui, 0.8, 0.0, "cancelLast");
        move_mouse(&gui, 105, 0, 0.0);
        click_mouse(&gui);
    }

    // deselect
    move_mouse_to_pos(&gui, 1075, 438, 0.0);

    if !is_first {
        click_mouse(&gui);

        sleep(Duration::from_millis(50));

        click_mouse(&gui);
    } else {
        click_mouse(&gui);
    }

    sleep(Duration::from_millis(100));
}