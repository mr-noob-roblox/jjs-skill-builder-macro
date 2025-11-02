use crate::imports::*;
use crate::utils::*;

pub fn create_wait(time: f32, is_first: bool) {
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