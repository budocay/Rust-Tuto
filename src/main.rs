use std::io::{self, Read};

fn main() {
    
    //calculate_weight_on_mars();
}

fn calculate_weight_on_mars(weight:i32) -> f64 {
    let heart_gravity:f64 = 9.81;
    let mars_gravity = 3.711;
    let weight_with_gravity = weight as f64 * mars_gravity;
    let weight_on_mars = weight_with_gravity / heart_gravity;
    return weight_on_mars;
}