use std::ops::{Mul, Sub};

use super::data;
use nalgebra;

pub fn calc(input:&data::Data) -> Option<data::Output> {
    let (a,b) = resolve(input);
    let out = (a.clone().transpose().mul(a.clone())).try_inverse()?.mul(a.transpose().mul(b.clone()));
    let overflow = a.clone().mul(out.clone()).sub(b);
    return  Some(data::Output {
        num: Vec::from(out.as_slice()),
        overflow: Vec::from(overflow.as_slice()),
    });
}
fn resolve(input:&data::Data) -> (nalgebra::DMatrix<f32>, nalgebra::DVector<f32>) {
    let mut rawmat = Vec::new();
    rawmat.resize(input.recipes.len()*input.items.len(),0.0);
    let mut index = 0;
    while index < input.recipes.len() {
        for (id,v) in input.recipes[index].inputs.iter() {
            rawmat[index*input.items.len()+id]+=v;
        }
        for (id,v) in input.recipes[index].outputs.iter() {
            rawmat[index*input.items.len()+id]-=v;
        }
        index += 1;
    }
    let mut rawvec = Vec::new();
    index = 0;
    rawvec.resize(input.items.len(), 0.0);
    while index < input.items.len() {
        rawvec[index]+= match input.items[index].itemtype {
            data::ItemType::Input => input.items[index].value,
            data::ItemType::Output => -input.items[index].value,
            data::ItemType::None => 0.0
        };
        index+=1
    }
    let mat = nalgebra::DMatrix::from_vec(input.recipes.len(), input.items.len(), rawmat);
    let vec = nalgebra::DVector::from_vec(rawvec);
    return (mat, vec);
}