use serde::{Serialize,Deserialize};
#[derive(Serialize,Deserialize)]
pub enum ItemType{
    Input,
    Output,
    None,
}
#[derive(Serialize,Deserialize)]
pub struct Item{
    pub name: String,
    pub itemtype: ItemType,
    pub value: f32
}
#[derive(Serialize,Deserialize)]
pub struct Recipe{
    pub name: String,
    pub inputs: Vec<(usize,f32)>,
    pub outputs: Vec<(usize,f32)>
}
impl Default for Item{
    fn default()->Self{
        Self{
            name: String::from("Item"),
            itemtype: ItemType::None,
            value: 0.0
        }
    }
}
impl Item{
    pub fn new(num: usize)->Self{
        Self{
            name: String::from(format!("Item #{}",num)),
            itemtype: ItemType::None,
            value:0.0
        }
    }
}
impl Default for Recipe{
    fn default()->Self{
        Self{
            name: String::from("Recipe"),
            inputs: Vec::new(),
            outputs: Vec::new()
        }
    }
}
impl Recipe{
    pub fn new(num: i32)->Self{
        Self{
            name: String::from(format!("Recipe #{}",num)),
            inputs: Vec::new(),
            outputs: Vec::new()
        }
    }
}
#[derive(Serialize,Deserialize)]
pub struct Data{
    pub items: Vec<Item>,
    pub recipes: Vec<Recipe>
}
impl Default for Data{
    fn default()->Self{
        Self{
            items: Vec::new(),
            recipes: Vec::new()
        }
    }
}

impl Data{
    pub fn check(&mut self){
        for item in self.items.iter_mut() {
            if item.value < 0.0 {
                item.value = 0.0;
            }
        }
        for rec in self.recipes.iter_mut(){
            for (idt,_) in rec.inputs.iter_mut(){
                if *idt >= self.items.len() {
                    *idt = 0;
                }
            }
            for (idt,_) in rec.outputs.iter_mut(){
                if *idt >= self.items.len() {
                    *idt = 0;
                }
            }
        }
    }
}
#[derive(Serialize,Deserialize)]
pub struct Output{
    pub num: Vec<f32>,
    pub overflow: Vec<f32>
}