use std::io;

fn main() {
    println!("For wich planet will you weight calculation ? Enter the number");
    println!("(1) LUNA");
    println!("(2) MERCURY");
    println!("(3) VENUS");
    println!("(4) JUPITER");
    println!("(5) SATURN");
    println!("(6) URANUS");
    println!("(7) NEPTUNE");
    println!("(8) MARS");
    println!("(9) PLUTON (I know it's not a real one)");
    println!(":");

    let mut input_planet = String::new();

    io::stdin().read_line(&mut input_planet).unwrap();
    selected_planete(input_planet);
}

fn selected_planete(planete_selected: String) {
    match planete_selected.as_str().trim() {
        "1" => println!("Your weight on Luna is : {}", calculate_weight_on_luna()),
        "2" => println!("Your weight on Mercury is : {}", calculate_weight_on_mercury()),
        "3" => println!("Your weight on Venus is : {}", calculate_weight_on_venus()),
        "4" => println!("Your weight on Jupiter is : {}", calculate_weight_on_jupiter()),
        "5" => println!("Your weight on Saturne is : {}", calculate_weight_on_saturn()),
        "6" => println!("Your weight on Urnaus is : {}", calculate_weight_on_uranus()),
        "7" => println!("Your weight on Neptune is : {}", calculate_weight_on_neptune()),
        "8" => println!("Your weight on Mars is : {}", calculate_weight_on_mars()),
        "9" => println!("Your weight on Pluton is : {}", calculate_weight_on_pluton()),
        _ => {}
    }
}

/**
 * Method for wieght calculation on Mars
 * She could be different and much better
 */
fn calculate_weight_on_mars() -> f64 {
    println!("Enter your weight(kg) : ");
    let mut input_weight = String::new();
    io::stdin().read_line(&mut input_weight).unwrap();
    let mut mars_weight:f64 = input_weight.trim().parse().unwrap();
    mars_weight = (mars_weight / 9.81) * 3.711;
    return mars_weight;
}

fn calculate_weight_on_luna() -> f64 {
    println!("Enter your weight(kg) : ");
    let mut input_weight = String::new();
    io::stdin().read_line(&mut input_weight).unwrap();
    let mut luna_weight:f64 = input_weight.trim().parse().unwrap();
    luna_weight = (luna_weight / 9.81) * 1.62;
    return luna_weight;
}

fn calculate_weight_on_mercury() -> f64 {
    println!("Enter your weight(kg) : ");
    let mut input_weight = String::new();
    io::stdin().read_line(&mut input_weight).unwrap();
    let mut mercury_weight:f64 = input_weight.trim().parse().unwrap();
    mercury_weight = (mercury_weight / 9.81) * 3.7;
    return mercury_weight;
}

fn calculate_weight_on_venus() -> f64 {
    println!("Enter your weight(kg) : ");
    let mut input_weight = String::new();
    io::stdin().read_line(&mut input_weight).unwrap();
    let mut venus_weight:f64 = input_weight.trim().parse().unwrap();
    venus_weight = (venus_weight / 9.81) * 8.87;
    return venus_weight;
}

fn calculate_weight_on_jupiter() -> f64 {
    println!("Enter your weight(kg) : ");
    let mut input_weight = String::new();
    io::stdin().read_line(&mut input_weight).unwrap();
    let mut jupiter_weight:f64 = input_weight.trim().parse().unwrap();
    jupiter_weight = (jupiter_weight / 9.81) * 24.79;
    return jupiter_weight;
}

fn calculate_weight_on_saturn() -> f64 {
    println!("Enter your weight(kg) : ");
    let mut input_weight = String::new();
    io::stdin().read_line(&mut input_weight).unwrap();
    let mut saturn_weight:f64 = input_weight.trim().parse().unwrap();
    saturn_weight = (saturn_weight / 9.81) * 10.64;
    return saturn_weight;
}

fn calculate_weight_on_uranus() -> f64 {
    println!("Enter your weight(kg) : ");
    let mut input_weight = String::new();
    io::stdin().read_line(&mut input_weight).unwrap();
    let mut uranus_weight:f64 = input_weight.trim().parse().unwrap();
    uranus_weight = (uranus_weight / 9.81) * 8.87;
    return uranus_weight;
}
fn calculate_weight_on_neptune() -> f64 {
    println!("Enter your weight(kg) : ");
    let mut input_weight = String::new();
    io::stdin().read_line(&mut input_weight).unwrap();
    let mut neptune_weight:f64 = input_weight.trim().parse().unwrap();
    neptune_weight = (neptune_weight / 9.81) * 11.15;
    return neptune_weight;
}

fn calculate_weight_on_pluton() -> f64 {
    println!("Enter your weight(kg) : ");
    let mut input_weight = String::new();
    io::stdin().read_line(&mut input_weight).unwrap();
    let mut pluton_weight:f64 = input_weight.trim().parse().unwrap();
    pluton_weight = (pluton_weight / 9.81) * 0.62;
    return pluton_weight;
}