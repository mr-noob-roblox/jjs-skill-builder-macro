use crate::imports::*;

pub fn click_mouse(gui: &RustAutoGui) {
    gui.left_click().unwrap();

    sleep(Duration::from_millis(50));
}

pub fn move_mouse_to_pos(gui: &RustAutoGui, x: u32, y: u32, m_time: f32) {
    gui.move_mouse_to_pos(x, y, m_time).unwrap();

    nudge_mouse();

    sleep(Duration::from_millis(50));
}

pub fn move_mouse(gui: &RustAutoGui, x: i32, y: i32, m_time: f32) {
    gui.move_mouse(x, y, m_time).unwrap();

    nudge_mouse();

    sleep(Duration::from_millis(50));
}

pub fn find_stored_image_on_screen_and_move_mouse(gui: &mut RustAutoGui, precision: f32, m_time: f32, alias: &str) -> Option<Vec<(u32, u32, f32)>> {
    let location: Option<Vec<(u32, u32, f32)>> = gui.find_stored_image_on_screen_and_move_mouse(precision, m_time, alias).unwrap();

    nudge_mouse();

    sleep(Duration::from_millis(50));

    return location;
}

pub fn nudge_mouse() {
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