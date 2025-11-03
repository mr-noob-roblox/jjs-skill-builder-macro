use crate::imports::*;
use crate::utils::*;

static SKILL_CACHE: Lazy<Mutex<HashMap<String, u32>>> = Lazy::new(|| {
    Mutex::new(HashMap::new())
});

pub fn create_skill(skill_name: &str, cancel_last: bool, is_first: bool) {
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

    sleep(Duration::from_millis(100));

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