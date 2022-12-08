use crate::{Data, BLANK};
use eframe::egui;

pub fn test_gui(data: Data) {
    let options = eframe::NativeOptions::default();
    eframe::run_native("My egui App", options, Box::new(|_cc| Box::new(data)));
}

impl Data {
    //fn new
}

impl eframe::App for Data {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                ui.label("Your name: ");
                //ui.text_edit_singleline(&mut self.name);
            });
            //ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            //if ui.button("Click each year").clicked() {
            //    self.age += 1;
            //}
            //ui.label(format!("Hello '{}', age {}", self.name, self.age));

            egui::Grid::new("grid").show(ui, |ui| {
                ui.label("First row, first column");
                ui.label("First row, second column");
                ui.end_row();

                ui.label("Second row, first column");
                ui.label("Second row, second column");
                ui.label("Second row, third column");
                ui.end_row();
            })
        });
    }
}
