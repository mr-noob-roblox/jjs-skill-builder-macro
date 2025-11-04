use eframe::{self, App};
use egui::{Color32, Frame, Id, Layout, RichText, Stroke, vec2};
use egui_dnd::dnd;

struct GUI {
    blocks: Vec<Vec<(String, String)>> // type and value
}

// temporary code
impl Default for GUI {
    fn default() -> Self {
        Self { 
            blocks: vec![
                vec![("WAIT".to_string(), "[1]".to_string())],
                vec![("SKILL".to_string(), "[sigma boy]".to_string())]
            ]
        }
    }
}

impl App for GUI {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("press f7 to run the macro");
            ui.label("f7 again to stop it in the middle of running");

            ui.separator();

            let frame = Frame::default()
                .fill(Color32::from_rgb(60, 60, 60))
                .corner_radius(4)
                .inner_margin(4)
                .show(ui, |ui| {
                    // drag and drop
                    dnd(ui, "timeline").show_vec(&mut self.blocks, |ui, item, handle, _state| {
                        // block
                        handle.ui(ui, |ui| {
                            let block_frame = Frame::default()
                                .fill(Color32::WHITE)
                                .inner_margin(4)
                                .stroke(Stroke::new(2.0, Color32::BLACK))
                                .show(ui, |ui| {
                                    ui.set_min_size(vec2(260.0, 20.0)); // reduced by 8 pixels on each side for inner_margin and 2 for outline
                                    ui.set_max_size(vec2(260.0, 20.0)); // reduced by 8 pixels on each side for inner_margin and 2 for outline

                                    // text
                                    if let Some((typee, value)) = item.first() {
                                        ui.with_layout(Layout::left_to_right(egui::Align::Center), |ui| {
                                            ui.label(RichText::new(typee).color(Color32::BLACK));
                                            ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                                                ui.label(RichText::new(value).color(Color32::BLACK));
                                            })
                                        });
                                    }
                            });
                        });
                    })
                });
        });
    }
}

pub fn start() -> eframe::Result<()> {
    println!("starting gui");

    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "jjs skill builder macro",
        options, 
        Box::new(|_cc| Ok(Box::<GUI>::default()))
    )
}