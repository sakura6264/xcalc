use super::calc;
use super::data;
use super::utils;
use eframe::egui;
use eframe::egui::ScrollArea;
use rfd;
use serde_json;
use std::fs;
pub struct App {
    data: data::Data,
    overflow: Vec<f32>,
    num: Vec<f32>,
    err: Option<String>,
    op: f32,
}
impl App {
    pub fn new() -> Self {
        Self {
            data: data::Data::default(),
            overflow: Vec::new(),
            num: Vec::new(),
            err: None,
            op: 1.0,
        }
    }
}
impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.data.check();
        if self.num.len() != self.data.recipes.len() {
            self.num.resize(self.data.recipes.len(), 0.0);
        }
        if self.overflow.len() != self.data.recipes.len() {
            self.overflow.resize(self.data.recipes.len(), 0.0);
        }
        if self.op < 1.0 {
            self.op = 1.0;
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                let mut cusor = ui.cursor();
                cusor.set_height(0.0);
                cusor.set_width(ui.available_width() / 2.0 - 5.0);
                ui.allocate_ui_at_rect(cusor, |ui| {
                    ui.vertical(|ui| {
                        ScrollArea::vertical()
                            .always_show_scroll(true)
                            .min_scrolled_height(500.0)
                            .id_source("Scroll 1")
                            .show(ui, |ui| {
                                let mut id = 0;
                                let mut delete: Option<usize> = None;
                                while id < self.data.items.len() {
                                    if utils::draw_item(
                                        ui,
                                        id,
                                        self.data.items.get_mut(id).unwrap(),
                                    ) {
                                        delete = Some(id);
                                    }
                                    id += 1;
                                }
                                if let Some(i) = delete {
                                    self.data.items.remove(i);
                                }
                            });
                        ui.separator();
                        ui.horizontal(|u| {
                            if u.button(" + ").clicked() {
                                self.data.items.push(data::Item::default());
                            }
                            if u.button("Clear").clicked() {
                                self.data.items.clear();
                            }
                        });
                    });
                });
                ui.separator();
                cusor = ui.cursor();
                cusor.set_height(0.0);
                cusor.set_width(ui.available_width());
                ui.allocate_ui_at_rect(cusor, |ui| {
                    ui.vertical(|ui| {
                        ScrollArea::vertical()
                            .always_show_scroll(true)
                            .min_scrolled_height(500.0)
                            .id_source("Scroll 2")
                            .show(ui, |ui| {
                                let mut id = 0;
                                let mut delete: Option<usize> = None;
                                while id < self.data.recipes.len() {
                                    if utils::draw_recipe(
                                        ui,
                                        id,
                                        self.data.recipes.get_mut(id).unwrap(),
                                        self.num.get(id).unwrap(),
                                        self.overflow.get(id).unwrap(),
                                        self.data.items.len(),
                                    ) {
                                        delete = Some(id);
                                    }
                                    id += 1;
                                }
                                if let Some(i) = delete {
                                    self.data.recipes.remove(i);
                                    self.num.remove(i);
                                    self.overflow.remove(i);
                                }
                            });
                        ui.separator();
                        ui.horizontal(|u| {
                            if !self.data.items.is_empty() {
                                if u.button(" + ").clicked() {
                                    self.data.recipes.push(data::Recipe::default());
                                    self.num.push(0.0);
                                    self.overflow.push(0.0);
                                }
                                if u.button("Clear").clicked() {
                                    self.data.recipes.clear();
                                    self.num.clear();
                                    self.overflow.clear();
                                }
                            }
                        });
                    });
                });
            });
            let check = !(self.data.recipes.is_empty() || self.data.items.is_empty());
            ui.horizontal(|ui| {
                if ui.button("Calculate").clicked() && check {
                    //TODO
                    let ret = calc::calc(&self.data);
                    match ret {
                        Some(r) => {
                            self.num = r.num;
                            self.overflow = r.overflow;
                            self.err = None;
                        }
                        None => {
                            self.err = Some(String::from(
                                "Cannot Calculate. Please change the arguments.",
                            ));
                        }
                    }
                }
                if ui.button("Load").clicked() {
                    let file = rfd::FileDialog::new()
                        .add_filter("JSON File", &["json"])
                        .pick_file();
                    if let Some(f) = file {
                        match fs::read_to_string(f) {
                            Ok(r) => match serde_json::from_str::<data::Data>(r.as_str()) {
                                Ok(rr) => {
                                    self.data = rr;
                                    self.err = None;
                                }
                                Err(ee) => {
                                    self.err = Some(String::from(format!(
                                        "Deserialize Error: {}",
                                        ee.to_string()
                                    )));
                                }
                            },
                            Err(e) => {
                                self.err = Some(String::from(format!(
                                    "Read File Error: {}",
                                    e.to_string()
                                )));
                            }
                        }
                    }
                }
                if ui.button("Save").clicked() && check {
                    let file = rfd::FileDialog::new()
                        .add_filter("JSON File", &["json"])
                        .save_file();
                    if let Some(f) = file {
                        match serde_json::to_string(&self.data) {
                            Ok(s) => match fs::write(f, s.as_bytes()) {
                                Ok(_) => {
                                    self.err = None;
                                }
                                Err(ee) => {
                                    self.err = Some(String::from(format!(
                                        "Write File Error: {}",
                                        ee.to_string()
                                    )));
                                }
                            },
                            Err(e) => {
                                self.err = Some(String::from(format!(
                                    "Serialize Error: {}",
                                    e.to_string()
                                )));
                            }
                        }
                    }
                }
                if ui.button("Mul").clicked() {
                    for i in self.data.items.iter_mut() {
                        i.value *= self.op;
                    }
                    for i in self.num.iter_mut() {
                        *i *= self.op;
                    }
                    for i in self.overflow.iter_mut() {
                        *i *= self.op;
                    }
                }
                if ui.button("Div").clicked() {
                    for i in self.data.items.iter_mut() {
                        i.value /= self.op;
                    }
                    for i in self.num.iter_mut() {
                        *i /= self.op;
                    }
                    for i in self.overflow.iter_mut() {
                        *i /= self.op;
                    }
                }
                ui.add(egui::DragValue::new(&mut self.op).speed(0.5).clamp_range(1.0..=f32::INFINITY));
            });
            if let Some(e) = self.err.clone() {
                ui.heading(format!("ERROR: {}", e));
            }
        });
    }
}
