pub use rustautogui::{self, RustAutoGui};

pub use std::{collections::HashMap, mem, sync::Mutex, thread::sleep, time::Duration};
pub use winapi::um::winuser::{INPUT, INPUT_MOUSE, MOUSEEVENTF_MOVE, SendInput};

pub use once_cell::{self, sync::Lazy};