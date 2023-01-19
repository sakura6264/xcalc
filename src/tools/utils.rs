use eframe::egui;
use super::data;

pub fn draw_item(ui: &mut egui::Ui, id:usize, item: &mut data::Item) -> bool {
    let mut ans = false;
    ui.vertical(|ui|{
        ui.separator();
        ui.horizontal(|ui|{
            ui.label(format!("#{}",id));
            ui.text_edit_singleline(&mut item.name);
        
        if ui.button(" - ").clicked() {
            ans = true;
        }
        match item.itemtype{
            data::ItemType::Input => {
                
                if ui.button("Input").on_hover_text("Click to change mode.").clicked(){
                    item.itemtype = data::ItemType::Output;
                }
                ui.add(egui::DragValue::new(&mut item.value).speed(0.5).clamp_range(0.0..=f32::INFINITY));
            },
            data::ItemType::Output => {
                if ui.button("Output").on_hover_text("Click to change mode.").clicked(){
                    item.itemtype = data::ItemType::None;
                }
                ui.add(egui::DragValue::new(&mut item.value).speed(0.5).clamp_range(0.0..=f32::INFINITY));
            },
            data::ItemType::None => {
                if ui.button("None").on_hover_text("Click to change mode.").clicked(){
                    item.itemtype = data::ItemType::Input;
                }
            }
        };});
    });
    return ans;
}

pub fn draw_recipe(ui: &mut egui::Ui, id:usize, recipe: &mut data::Recipe,num:&f32,overflow:&f32, max: usize) -> bool{
    let mut ans = false;
    ui.vertical(|ui|{
        ui.separator();
        ui.horizontal(|ui|{
            ui.label(format!("#{}",id));
            ui.text_edit_singleline(&mut recipe.name);
            ans = ui.button(" - ").clicked();
        });
        ui.horizontal(|ui|{
            ui.label(format!("Num: {:.2} ",num));
            ui.label(format!("Overflow: {:.2}",overflow));
        });
        
        ui.horizontal(|ui|{
            let mut cursor = ui.cursor();
            cursor.set_height(0.0);
            cursor.set_width(ui.available_width()/2.0-5.0);
            ui.allocate_ui_at_rect(cursor, |ui|{
                ui.vertical(|ui|{
                ui.label("Inputs :  ");
                let mut index = 0;
                let mut delete: Option<usize> = None;
                while index < recipe.inputs.len() {
                    let (idt,v) = recipe.inputs.get_mut(index).unwrap();
                    ui.separator();
                    ui.horizontal(|ui|{
                        ui.label("#id: ");
                        ui.add(egui::DragValue::new(idt).speed(1).clamp_range(0..=max));
                        ui.label("speed: ");
                        ui.add(egui::DragValue::new(v).speed(0.5).clamp_range(0.0..=f32::INFINITY));
                        if ui.button(" - ").clicked() {
                            delete = Some(index);
                        }
                    });
                    index +=1;
                }
                if let Some(i) = delete {
                    recipe.inputs.remove(i);
                }
                ui.separator();
                if ui.button(" + ").clicked() {
                    recipe.inputs.push((0,0.0));
                }
                });
                
            });
            ui.separator();
            cursor = ui.cursor();
            cursor.set_height(0.0);
            cursor.set_width(ui.available_width());
            ui.allocate_ui_at_rect(cursor,|ui|{
                ui.vertical(|ui|{
                ui.label("Outputs :  ");
                let mut index = 0;
                let mut delete: Option<usize> = None;
                while index < recipe.outputs.len() {
                    let (idt,v) = recipe.outputs.get_mut(index).unwrap();
                    ui.separator();
                    ui.horizontal(|ui|{
                        ui.label("#id: ");
                        ui.add(egui::DragValue::new(idt).speed(1).clamp_range(0..=max));
                        ui.label("speed: ");
                        ui.add(egui::DragValue::new(v).speed(0.5).clamp_range(0.0..=f32::INFINITY));
                        if ui.button(" - ").clicked() {
                            delete = Some(index);
                        }
                    });
                    index +=1;
                }
                if let Some(i) = delete {
                    recipe.outputs.remove(i);
                }
                ui.separator();
                if ui.button(" + ").clicked() {
                    recipe.outputs.push((0,0.0));
                }
                });
                
            });
        });
        ui.separator();
    });
    return ans;
}
